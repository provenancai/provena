use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("capability names must not be empty")]
    EmptyCapability,
    #[error("user IDs must not be empty")]
    EmptyUserId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct PluginId(Uuid);

impl PluginId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl Default for PluginId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CapabilityName(String);

impl CapabilityName {
    pub fn new(value: impl Into<String>) -> Result<Self, CoreError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(CoreError::EmptyCapability);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn is_storage_capability(&self) -> bool {
        self.0.starts_with("storage.")
    }
}

impl AsRef<str> for CapabilityName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProvenanceClass {
    Deterministic,
    Machine,
    Human,
}

/// A validated user identifier.
///
/// Wraps a non-empty string identifying the human or system actor
/// responsible for a provenance event. Used in [`LedgerEntry`] to
/// record who authorized or triggered each event.
///
/// [`LedgerEntry`]: provena_ledger::LedgerEntry
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(String);

impl UserId {
    /// Create a new `UserId` from a string value.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::EmptyUserId`] if the value is empty or whitespace-only.
    pub fn new(value: impl Into<String>) -> Result<Self, CoreError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(CoreError::EmptyUserId);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for UserId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
