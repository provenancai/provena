use std::collections::{BTreeMap, BTreeSet};

use provena_core::{CapabilityName, PluginId};
use provena_sdk::PluginManifest;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum KernelError {
    #[error("singleton capability {capability} already has a registered plugin")]
    DuplicateSingletonCapability { capability: String },
}

#[derive(Debug, Clone, Serialize)]
pub struct KernelHealth {
    pub registered_plugins: usize,
    pub registered_capabilities: usize,
}

#[derive(Debug, Default)]
pub struct Kernel {
    routes: BTreeMap<CapabilityName, Vec<PluginManifest>>,
}

impl Kernel {
    pub fn register_plugin(&mut self, manifest: PluginManifest) -> Result<(), KernelError> {
        for capability in &manifest.capabilities {
            if capability.singleton && self.routes.contains_key(&capability.name) {
                return Err(KernelError::DuplicateSingletonCapability {
                    capability: capability.name.as_str().to_owned(),
                });
            }
        }

        for capability in &manifest.capabilities {
            let route_set = self.routes.entry(capability.name.clone()).or_default();
            route_set.push(manifest.clone());
            route_set.sort_by_key(|registered_manifest| {
                registered_manifest
                    .capabilities
                    .iter()
                    .find(|descriptor| descriptor.name == capability.name)
                    .map(|descriptor| descriptor.priority)
                    .unwrap_or(u16::MAX)
            });
        }

        Ok(())
    }

    pub fn route(&self, capability: &CapabilityName) -> Option<&PluginManifest> {
        self.routes.get(capability).and_then(|manifests| manifests.first())
    }

    pub fn health(&self) -> KernelHealth {
        let plugin_ids: BTreeSet<PluginId> = self
            .routes
            .values()
            .flat_map(|manifests| manifests.iter().map(|manifest| manifest.plugin_id))
            .collect();

        KernelHealth {
            registered_plugins: plugin_ids.len(),
            registered_capabilities: self.routes.len(),
        }
    }
}