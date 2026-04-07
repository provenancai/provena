use chrono::{DateTime, Utc};
use provena_core::{ProvenanceClass, UserId};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
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

impl Default for LedgerEntryId {
    fn default() -> Self {
        Self::new()
    }
}

/// A single immutable entry in the provenance ledger.
///
/// Every field is set at construction time and never mutated.
/// `content_hash` is a SHA-256 digest over the entry's identity and
/// content fields, providing tamper-evidence and a stable reference
/// for downstream systems.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerEntry {
    pub id: LedgerEntryId,
    pub provenance_class: ProvenanceClass,
    /// The user or system actor responsible for this provenance event.
    pub actor: UserId,
    pub summary: String,
    pub recorded_at: DateTime<Utc>,
    /// SHA-256 digest of `id || provenance_class || actor || summary || recorded_at`.
    ///
    /// Computed at construction time. Provides tamper-evidence for the entry.
    pub content_hash: [u8; 32],
}

impl LedgerEntry {
    pub fn new(
        provenance_class: ProvenanceClass,
        actor: UserId,
        summary: impl Into<String>,
    ) -> Result<Self, LedgerError> {
        let summary = summary.into();

        if summary.trim().is_empty() {
            return Err(LedgerError::EmptySummary);
        }

        let id = LedgerEntryId::new();
        let recorded_at = Utc::now();
        let content_hash = compute_content_hash(&id, provenance_class, &actor, &summary, recorded_at);

        Ok(Self {
            id,
            provenance_class,
            actor,
            summary,
            recorded_at,
            content_hash,
        })
    }
}

/// Compute a SHA-256 digest over the entry's identity and content fields.
///
/// Input (in order):
/// - UUID bytes of the entry ID (16 bytes, big-endian)
/// - provenance class discriminant (1 byte: 0=Deterministic, 1=Machine, 2=Human)
/// - actor string bytes (length-prefixed with u64 LE)
/// - summary string bytes (length-prefixed with u64 LE)
/// - recorded_at Unix timestamp seconds (i64 LE)
/// - recorded_at subsecond nanoseconds (u32 LE)
fn compute_content_hash(
    id: &LedgerEntryId,
    provenance_class: ProvenanceClass,
    actor: &UserId,
    summary: &str,
    recorded_at: DateTime<Utc>,
) -> [u8; 32] {
    let mut hasher = Sha256::new();

    // Entry ID — UUID bytes (16 bytes)
    hasher.update(id.0.as_bytes());

    // Provenance class as a stable 1-byte discriminant
    let class_byte: u8 = match provenance_class {
        ProvenanceClass::Deterministic => 0,
        ProvenanceClass::Machine => 1,
        ProvenanceClass::Human => 2,
    };
    hasher.update([class_byte]);

    // Actor — length-prefixed UTF-8 bytes
    let actor_bytes = actor.as_str().as_bytes();
    hasher.update((actor_bytes.len() as u64).to_le_bytes());
    hasher.update(actor_bytes);

    // Summary — length-prefixed UTF-8 bytes
    let summary_bytes = summary.as_bytes();
    hasher.update((summary_bytes.len() as u64).to_le_bytes());
    hasher.update(summary_bytes);

    // Timestamp — seconds and nanoseconds (stable over serialization format changes)
    hasher.update(recorded_at.timestamp().to_le_bytes());
    hasher.update(recorded_at.timestamp_subsec_nanos().to_le_bytes());

    let result = hasher.finalize();
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&result);
    hash
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
