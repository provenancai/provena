use std::collections::{BTreeMap, BTreeSet};

use provena_core::{CapabilityName, PluginId};
use provena_ledger::LedgerEntry;
use provena_sdk::{CapabilityState, PluginManifest};
use serde::Serialize;
use thiserror::Error;

/// Errors that can occur during kernel operations.
#[derive(Debug, Error)]
pub enum KernelError {
    /// Returned when a plugin attempts to register as `Active` for a singleton
    /// capability that already has an active plugin. Only one plugin may be
    /// `Active` for a singleton capability at a time.
    ///
    /// Note: `Standby` registrations are always permitted, even for singletons.
    /// This allows migration targets to be registered before cutover.
    #[error("singleton capability '{capability}' already has an active plugin")]
    DuplicateActiveSingleton { capability: String },

    /// Returned when `activate_capability` is called but the authorizing ledger
    /// entry does not carry the `Human` provenance class. Capability activation
    /// is always a Human provenance event — it cannot be automated.
    #[error("capability activation requires a Human provenance event")]
    ActivationRequiresHumanProvenance,

    /// Returned when `activate_capability` is called for a plugin that is not
    /// registered for the requested capability.
    #[error("plugin '{plugin_id}' is not registered for capability '{capability}'")]
    PluginNotRegistered {
        plugin_id: String,
        capability: String,
    },
}

/// A point-in-time snapshot of kernel registration state.
///
/// Returned by [`Kernel::health`]. This is a snapshot — it reflects the state
/// of the kernel at the moment it was called, not a live view.
#[derive(Debug, Clone, Serialize)]
pub struct KernelHealth {
    /// The number of distinct plugins currently registered across all capabilities,
    /// regardless of their `Active` or `Standby` state.
    pub registered_plugins: usize,

    /// The number of distinct capability names currently registered.
    pub registered_capabilities: usize,

    /// The number of capabilities currently in `Active` state.
    pub active_capabilities: usize,

    /// The number of capabilities currently in `Standby` state.
    pub standby_capabilities: usize,
}

/// The provena microkernel.
///
/// The kernel is the routing hub of the system. It knows nothing about what
/// plugins do — only what capabilities they advertise and how to route requests
/// to the highest-priority active plugin for a given capability.
///
/// # Routing model
///
/// Each capability name maps to a priority-ordered list of plugin manifests.
/// Lower priority numbers win (0 is highest priority). [`Kernel::route`] returns
/// only the highest-priority `Active` plugin — `Standby` plugins are never routed.
///
/// # Singleton capabilities
///
/// Singleton capabilities may have only one `Active` plugin at a time.
/// Multiple `Standby` registrations are always permitted — this is what
/// enables the migration model. See [`Kernel::activate_capability`].
///
/// # Migration model
///
/// To migrate a singleton capability from plugin A to plugin B:
///
/// 1. Register plugin B with [`CapabilityDescriptor::new_standby`]
/// 2. Run the migration (copy artifacts, verify integrity)
/// 3. Record a Human provenance event in the ledger authorizing cutover
/// 4. Call [`Kernel::activate_capability`] with the ledger entry — this
///    atomically promotes B to `Active` and demotes A to `Standby`
/// 5. Deregister plugin A when no longer needed
///
/// The kernel enforces step 4 — activation without a Human provenance event
/// is always rejected.
#[derive(Debug, Default)]
pub struct Kernel {
    /// The route table. Maps each capability name to a priority-ordered list
    /// of (state, manifest) pairs. Ordered by priority ascending (lowest wins).
    routes: BTreeMap<CapabilityName, Vec<(CapabilityState, PluginManifest)>>,
}

impl Kernel {
    /// Register a plugin with the kernel.
    ///
    /// Each capability in the manifest is added to the route table in priority
    /// order. The capability's `state` field determines whether it is
    /// immediately active or held in standby.
    ///
    /// Registration is atomic — if any constraint fails, no capabilities from
    /// this manifest are added.
    ///
    /// # Errors
    ///
    /// Returns [`KernelError::DuplicateActiveSingleton`] if the manifest
    /// advertises a singleton capability as `Active` when one is already active.
    /// Standby registrations never trigger this error.
    pub fn register_plugin(&mut self, manifest: PluginManifest) -> Result<(), KernelError> {
        // Validate all constraints before mutating state — registration is atomic.
        for capability in &manifest.capabilities {
            if capability.singleton && capability.state == CapabilityState::Active {
                let has_active = self
                    .routes
                    .get(&capability.name)
                    .map(|entries| {
                        entries
                            .iter()
                            .any(|(state, _)| *state == CapabilityState::Active)
                    })
                    .unwrap_or(false);

                if has_active {
                    return Err(KernelError::DuplicateActiveSingleton {
                        capability: capability.name.as_str().to_owned(),
                    });
                }
            }
        }

        // All constraints passed — mutate the route table.
        for capability in &manifest.capabilities {
            let route_set = self.routes.entry(capability.name.clone()).or_default();
            route_set.push((capability.state, manifest.clone()));

            // Sort by priority ascending so route() can return the first Active entry.
            route_set.sort_by_key(|(_, registered_manifest)| {
                registered_manifest
                    .capabilities
                    .iter()
                    .find(|descriptor| descriptor.name == capability.name)
                    .map(|descriptor| descriptor.priority)
                    .unwrap_or(u16::MAX) // Unknown priority sorts last.
            });
        }

        Ok(())
    }

    /// Route a request to the highest-priority active plugin for a capability.
    ///
    /// Returns the manifest of the highest-priority `Active` plugin for the
    /// requested capability, or `None` if no active plugin is registered.
    ///
    /// `Standby` plugins are never returned by this method.
    ///
    /// Note: the manifest does not yet carry an endpoint URL — actual HTTP
    /// forwarding is not yet implemented. See known technical debt in `CLAUDE.md`.
    pub fn route(&self, capability: &CapabilityName) -> Option<&PluginManifest> {
        self.routes.get(capability).and_then(|entries| {
            entries
                .iter()
                .find(|(state, _)| *state == CapabilityState::Active)
                .map(|(_, manifest)| manifest)
        })
    }

    /// Promote a plugin to `Active` for a capability, demoting any current
    /// active plugin to `Standby`.
    ///
    /// This is the cutover step in a storage migration. It is atomic —
    /// the promotion and demotion happen together or not at all.
    ///
    /// # Authorization
    ///
    /// The caller must supply a [`LedgerEntry`] that was recorded with
    /// [`ProvenanceClass::Human`]. The kernel enforces this — activation
    /// without a Human provenance event is always rejected. The ledger entry
    /// is not stored by the kernel; it is the caller's responsibility to
    /// persist it before calling this method.
    ///
    /// # Errors
    ///
    /// - [`KernelError::ActivationRequiresHumanProvenance`] if the ledger entry
    ///   is not a Human provenance event.
    /// - [`KernelError::PluginNotRegistered`] if the plugin is not registered
    ///   for the requested capability.
    ///
    /// [`ProvenanceClass::Human`]: provena_core::ProvenanceClass::Human
    pub fn activate_capability(
        &mut self,
        capability: &CapabilityName,
        plugin_id: PluginId,
        authorization: &LedgerEntry,
    ) -> Result<(), KernelError> {
        use provena_core::ProvenanceClass;

        // Enforce the Human provenance gate — this can never be automated.
        if authorization.provenance_class != ProvenanceClass::Human {
            return Err(KernelError::ActivationRequiresHumanProvenance);
        }

        let entries =
            self.routes
                .get_mut(capability)
                .ok_or_else(|| KernelError::PluginNotRegistered {
                    plugin_id: plugin_id.as_uuid().to_string(),
                    capability: capability.as_str().to_owned(),
                })?;

        // Verify the target plugin is actually registered for this capability.
        let target_exists = entries
            .iter()
            .any(|(_, manifest)| manifest.plugin_id == plugin_id);

        if !target_exists {
            return Err(KernelError::PluginNotRegistered {
                plugin_id: plugin_id.as_uuid().to_string(),
                capability: capability.as_str().to_owned(),
            });
        }

        // Atomic promotion: demote all Active to Standby, promote target to Active.
        for (state, manifest) in entries.iter_mut() {
            if manifest.plugin_id == plugin_id {
                *state = CapabilityState::Active;
            } else if *state == CapabilityState::Active {
                *state = CapabilityState::Standby;
            }
        }

        Ok(())
    }

    /// Return a snapshot of current kernel health.
    ///
    /// Counts distinct registered plugins (by [`PluginId`]), distinct capability
    /// names, and the breakdown of active vs standby capability registrations.
    pub fn health(&self) -> KernelHealth {
        let plugin_ids: BTreeSet<PluginId> = self
            .routes
            .values()
            .flat_map(|entries| entries.iter().map(|(_, manifest)| manifest.plugin_id))
            .collect();

        let active_capabilities = self
            .routes
            .values()
            .flat_map(|entries| entries.iter())
            .filter(|(state, _)| *state == CapabilityState::Active)
            .count();

        let standby_capabilities = self
            .routes
            .values()
            .flat_map(|entries| entries.iter())
            .filter(|(state, _)| *state == CapabilityState::Standby)
            .count();

        KernelHealth {
            registered_plugins: plugin_ids.len(),
            registered_capabilities: self.routes.len(),
            active_capabilities,
            standby_capabilities,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use provena_core::{CapabilityName, PluginId, ProvenanceClass, UserId};
    use provena_ledger::LedgerEntry;
    use provena_sdk::{CapabilityDescriptor, PluginManifest};

    /// Convenience constructor for capability names in tests.
    fn cap(name: &str) -> CapabilityName {
        CapabilityName::new(name).unwrap()
    }

    /// Convenience constructor for plugin manifests in tests.
    ///
    /// Uses a placeholder endpoint URL since routing HTTP requests is not
    /// exercised by unit tests.
    fn manifest(
        id: PluginId,
        name: &str,
        capabilities: Vec<CapabilityDescriptor>,
    ) -> PluginManifest {
        PluginManifest::new(id, name, "http://localhost:0", capabilities)
    }

    /// Convenience constructor for a Human provenance ledger entry in tests.
    fn human_auth(actor: &str, summary: &str) -> LedgerEntry {
        let user_id = UserId::new(actor).unwrap();
        LedgerEntry::new(ProvenanceClass::Human, user_id, summary).unwrap()
    }

    /// Registering a second Active plugin for a singleton capability must be
    /// rejected. Connectivity loss on a singleton is a hard stop — silent
    /// rerouting is never allowed.
    #[test]
    fn singleton_active_rejects_second_active_registration() {
        let mut kernel = Kernel::default();
        let storage = cap("storage.azure.blob.standard");

        let first = manifest(
            PluginId::new(),
            "Azure Blob A",
            vec![CapabilityDescriptor::new(storage.clone(), 0, true)],
        );
        let second = manifest(
            PluginId::new(),
            "Azure Blob B",
            vec![CapabilityDescriptor::new(storage.clone(), 1, true)],
        );

        kernel.register_plugin(first).unwrap();
        let err = kernel.register_plugin(second).unwrap_err();

        assert!(
            matches!(
                err,
                KernelError::DuplicateActiveSingleton { ref capability }
                    if capability == "storage.azure.blob.standard"
            ),
            "expected DuplicateActiveSingleton, got: {err}"
        );
    }

    /// A migration target may register as Standby for a singleton capability
    /// even when an Active plugin already exists. This is the first step of
    /// the migration model.
    #[test]
    fn singleton_permits_standby_registration_alongside_active() {
        let mut kernel = Kernel::default();
        let storage = cap("storage.azure.blob.standard");

        let active = manifest(
            PluginId::new(),
            "Azure Blob A",
            vec![CapabilityDescriptor::new(storage.clone(), 0, true)],
        );
        let standby = manifest(
            PluginId::new(),
            "Azure Blob B",
            vec![CapabilityDescriptor::new_standby(storage.clone(), 1, true)],
        );

        kernel.register_plugin(active).unwrap();
        // This must succeed — standby registration is always permitted.
        kernel.register_plugin(standby).unwrap();
    }

    /// When two plugins compete for the same non-singleton capability, route()
    /// must return the plugin with the lowest priority number (highest authority),
    /// regardless of registration order.
    #[test]
    fn route_returns_highest_authority_active_plugin() {
        let mut kernel = Kernel::default();
        let analytics = cap("analytics.query");

        let id_low = PluginId::new(); // priority 10 — lower authority
        let id_high = PluginId::new(); // priority 0  — higher authority

        // Register lower-authority plugin first to prove sorting is not
        // insertion-order dependent.
        kernel
            .register_plugin(manifest(
                id_low,
                "Analytics Secondary",
                vec![CapabilityDescriptor::new(analytics.clone(), 10, false)],
            ))
            .unwrap();
        kernel
            .register_plugin(manifest(
                id_high,
                "Analytics Primary",
                vec![CapabilityDescriptor::new(analytics.clone(), 0, false)],
            ))
            .unwrap();

        let routed = kernel
            .route(&analytics)
            .expect("capability should be routable");

        assert_eq!(
            routed.plugin_id, id_high,
            "route() must return the plugin with priority 0, not priority 10"
        );
    }

    /// Standby plugins must never be returned by route(), even if they have
    /// a higher priority number than the active plugin.
    #[test]
    fn route_never_returns_standby_plugin() {
        let mut kernel = Kernel::default();
        let storage = cap("storage.azure.blob.standard");

        let active_id = PluginId::new();
        let standby_id = PluginId::new();

        kernel
            .register_plugin(manifest(
                active_id,
                "Active Storage",
                vec![CapabilityDescriptor::new(storage.clone(), 10, true)],
            ))
            .unwrap();

        kernel
            .register_plugin(manifest(
                standby_id,
                "Standby Storage",
                vec![CapabilityDescriptor::new_standby(storage.clone(), 0, true)],
            ))
            .unwrap();

        let routed = kernel
            .route(&storage)
            .expect("capability should be routable");

        assert_eq!(
            routed.plugin_id, active_id,
            "route() must not return a Standby plugin even if it has higher priority"
        );
    }

    /// activate_capability must atomically promote the target plugin to Active
    /// and demote the current Active plugin to Standby, when authorized by a
    /// Human provenance event.
    #[test]
    fn activate_capability_promotes_standby_and_demotes_active() {
        let mut kernel = Kernel::default();
        let storage = cap("storage.azure.blob.standard");

        let old_id = PluginId::new();
        let new_id = PluginId::new();

        kernel
            .register_plugin(manifest(
                old_id,
                "Old Storage",
                vec![CapabilityDescriptor::new(storage.clone(), 0, true)],
            ))
            .unwrap();

        kernel
            .register_plugin(manifest(
                new_id,
                "New Storage",
                vec![CapabilityDescriptor::new_standby(storage.clone(), 1, true)],
            ))
            .unwrap();

        // Verify old plugin is currently active.
        assert_eq!(kernel.route(&storage).unwrap().plugin_id, old_id);

        // Authorize the cutover with a Human provenance event.
        let auth = human_auth("matthew.ryrie", "authorized storage migration cutover");
        kernel.activate_capability(&storage, new_id, &auth).unwrap();

        // New plugin must now be active.
        assert_eq!(
            kernel.route(&storage).unwrap().plugin_id,
            new_id,
            "new plugin must be active after cutover"
        );
    }

    /// activate_capability must be rejected if the authorizing ledger entry
    /// is not a Human provenance event. Automated cutover is never permitted.
    #[test]
    fn activate_capability_rejects_non_human_provenance() {
        let mut kernel = Kernel::default();
        let storage = cap("storage.azure.blob.standard");

        let old_id = PluginId::new();
        let new_id = PluginId::new();

        kernel
            .register_plugin(manifest(
                old_id,
                "Old Storage",
                vec![CapabilityDescriptor::new(storage.clone(), 0, true)],
            ))
            .unwrap();

        kernel
            .register_plugin(manifest(
                new_id,
                "New Storage",
                vec![CapabilityDescriptor::new_standby(storage.clone(), 1, true)],
            ))
            .unwrap();

        // Attempt cutover with a Machine provenance event — must be rejected.
        let machine_auth = LedgerEntry::new(
            ProvenanceClass::Machine,
            UserId::new("automated-job").unwrap(),
            "auto cutover",
        )
        .unwrap();

        let err = kernel
            .activate_capability(&storage, new_id, &machine_auth)
            .unwrap_err();

        assert!(
            matches!(err, KernelError::ActivationRequiresHumanProvenance),
            "expected ActivationRequiresHumanProvenance, got: {err}"
        );
    }

    /// A manifest that advertises the same capability name twice would produce
    /// duplicate entries in the route list, skewing routing and health counts.
    /// The kernel must reject such manifests.
    ///
    /// This test is ignored because duplicate capability validation is known
    /// technical debt. Run with:
    ///
    /// ```text
    /// cargo test -- --include-ignored
    /// ```
    #[test]
    #[ignore = "known debt: duplicate capability validation not yet implemented"]
    fn manifest_with_duplicate_capability_is_rejected() {
        let mut kernel = Kernel::default();
        let cap_name = cap("analytics.query");

        let bad_manifest = manifest(
            PluginId::new(),
            "Duplicated Caps Plugin",
            vec![
                CapabilityDescriptor::new(cap_name.clone(), 0, false),
                CapabilityDescriptor::new(cap_name.clone(), 5, false),
            ],
        );

        let result = kernel.register_plugin(bad_manifest);
        assert!(
            result.is_err(),
            "register_plugin must return an error when the manifest contains duplicate capabilities"
        );
    }
}
