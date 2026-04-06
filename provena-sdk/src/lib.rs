use provena_core::{CapabilityName, PluginId};
use serde::{Deserialize, Serialize};

/// The operational state of a capability registration.
///
/// Controls whether the kernel will route requests to a plugin for a given
/// capability. Only `Active` plugins are returned by [`Kernel::route`].
///
/// # Migration model
///
/// During a storage migration, both the source and destination plugins are
/// registered simultaneously — the source as `Active`, the destination as
/// `Standby`. Once the migration is complete and a Human provenance event
/// has been recorded authorizing the cutover, the kernel transitions the
/// destination to `Active` and the source to `Standby` atomically.
///
/// This transition is never automatic. It always requires explicit
/// authorization via a Human provenance event. See [`Kernel::activate_capability`].
///
/// [`Kernel::route`]: provena_kernel::Kernel::route
/// [`Kernel::activate_capability`]: provena_kernel::Kernel::activate_capability
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum CapabilityState {
    /// The plugin is actively routing for this capability.
    /// Only one plugin per capability may be Active at a time.
    #[default]
    Active,

    /// The plugin is registered but not routing.
    /// Used for migration targets and standby configurations.
    Standby,
}

/// Describes a single capability that a plugin advertises.
///
/// A plugin manifest contains one or more capability descriptors, each
/// declaring a capability name, routing priority, singleton constraint,
/// and operational state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityDescriptor {
    /// The capability name. Encodes both the operation and the role,
    /// e.g. `storage.azure.blob.sensitive`. See `CLAUDE.md` for naming conventions.
    pub name: CapabilityName,

    /// Routing priority. Lower numbers win — 0 is highest authority.
    /// Mirrors the DDR authority model.
    pub priority: u16,

    /// If true, only one plugin may be `Active` for this capability at a time.
    /// A second registration attempt returns [`KernelError::DuplicateActiveSingleton`].
    /// Standby registrations are always permitted regardless of singleton state.
    pub singleton: bool,

    /// The operational state of this capability registration.
    /// Defaults to `Active`. Set to `Standby` for migration targets.
    pub state: CapabilityState,
}

impl CapabilityDescriptor {
    /// Create a new capability descriptor in the `Active` state.
    ///
    /// This is the standard constructor for plugin registrations. Use
    /// [`CapabilityDescriptor::new_standby`] when registering a migration target.
    pub fn new(name: CapabilityName, priority: u16, singleton: bool) -> Self {
        Self {
            name,
            priority,
            singleton,
            state: CapabilityState::Active,
        }
    }

    /// Create a new capability descriptor in the `Standby` state.
    ///
    /// Use this when registering a migration target. The plugin will be
    /// registered but will not receive routed requests until promoted to
    /// `Active` via a Human provenance event.
    pub fn new_standby(name: CapabilityName, priority: u16, singleton: bool) -> Self {
        Self {
            name,
            priority,
            singleton,
            state: CapabilityState::Standby,
        }
    }
}

/// The manifest a plugin presents to the kernel at registration time.
///
/// Contains the plugin's identity, display name, and the list of capabilities
/// it can serve. This is the runtime representation of the `plugin.toml`
/// on-disk format — they carry the same information at different layers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    /// Unique identifier for this plugin instance.
    pub plugin_id: PluginId,

    /// Human-readable name for logging and diagnostics.
    pub display_name: String,

    /// The capabilities this plugin advertises to the kernel.
    pub capabilities: Vec<CapabilityDescriptor>,
}

impl PluginManifest {
    /// Create a new plugin manifest.
    pub fn new(
        plugin_id: PluginId,
        display_name: impl Into<String>,
        capabilities: Vec<CapabilityDescriptor>,
    ) -> Self {
        Self {
            plugin_id,
            display_name: display_name.into(),
            capabilities,
        }
    }
}

/// Trait that all out-of-process plugins must satisfy.
///
/// In practice, plugins are separate processes communicating over HTTP.
/// This trait is the in-process representation used for testing and
/// reference implementations.
pub trait Plugin {
    fn manifest(&self) -> &PluginManifest;
}
