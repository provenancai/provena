use chrono::{DateTime, Utc};
use provena_core::ProvenanceClass;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum LedgerError {
    #[error("ledger entry summary must not be empty")]
    EmptySummary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LedgerEntryId(Uuid);

impl LedgerEntryId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerEntry {
    pub id: LedgerEntryId,
    pub provenance_class: ProvenanceClass,
    pub actor: String,
    pub summary: String,
    pub recorded_at: DateTime<Utc>,
}

impl LedgerEntry {
    pub fn new(
        provenance_class: ProvenanceClass,
        actor: impl Into<String>,
        summary: impl Into<String>,
    ) -> Result<Self, LedgerError> {
        let summary = summary.into();

        if summary.trim().is_empty() {
            return Err(LedgerError::EmptySummary);
        }

        Ok(Self {
            id: LedgerEntryId::new(),
            provenance_class,
            actor: actor.into(),
            summary,
            recorded_at: Utc::now(),
        })
    }
}

pub trait LedgerStore {
    fn append(&mut self, entry: LedgerEntry) -> Result<(), LedgerError>;
    fn entries(&self) -> &[LedgerEntry];
}

#[derive(Debug, Default)]
pub struct InMemoryLedger {
    entries: Vec<LedgerEntry>,
}

impl LedgerStore for InMemoryLedger {
    fn append(&mut self, entry: LedgerEntry) -> Result<(), LedgerError> {
        self.entries.push(entry);
        Ok(())
    }

    fn entries(&self) -> &[LedgerEntry] {
        &self.entries
    }
}