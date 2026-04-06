use provena_core::{CapabilityName, PluginId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityDescriptor {
    pub name: CapabilityName,
    pub priority: u16,
    pub singleton: bool,
}

impl CapabilityDescriptor {
    pub fn new(name: CapabilityName, priority: u16, singleton: bool) -> Self {
        Self {
            name,
            priority,
            singleton,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub plugin_id: PluginId,
    pub display_name: String,
    pub capabilities: Vec<CapabilityDescriptor>,
}

impl PluginManifest {
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

pub trait Plugin {
    fn manifest(&self) -> &PluginManifest;
}
