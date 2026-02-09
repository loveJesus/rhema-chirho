// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Cross-reference graph types — directed edges between verses.

use serde::{Deserialize, Serialize};
use std::fmt;

use crate::keys_chirho::VerseRefChirho;

/// The type of a cross-reference edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum XRefTypeChirho {
    /// Direct cross-reference (explicit markup).
    DirectChirho,
    /// Parallel passage (synoptic gospels, OT history, etc.).
    ParallelChirho,
    /// OT quotation cited in NT.
    QuotationChirho,
    /// Verbal or thematic allusion (not a direct quote).
    AllusionChirho,
    /// Prophecy → fulfillment link.
    ProphecyFulfillmentChirho,
    /// Imported from an external dataset (e.g. TSK).
    ExternalDatasetChirho,
}

impl fmt::Display for XRefTypeChirho {
    fn fmt(&self, f_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DirectChirho => write!(f_chirho, "Direct"),
            Self::ParallelChirho => write!(f_chirho, "Parallel"),
            Self::QuotationChirho => write!(f_chirho, "Quotation"),
            Self::AllusionChirho => write!(f_chirho, "Allusion"),
            Self::ProphecyFulfillmentChirho => write!(f_chirho, "ProphecyFulfillment"),
            Self::ExternalDatasetChirho => write!(f_chirho, "ExternalDataset"),
        }
    }
}

impl XRefTypeChirho {
    /// Parse from a string (case-insensitive).
    pub fn from_str_chirho(s_chirho: &str) -> Option<Self> {
        match s_chirho.to_lowercase().as_str() {
            "direct" => Some(Self::DirectChirho),
            "parallel" => Some(Self::ParallelChirho),
            "quotation" => Some(Self::QuotationChirho),
            "allusion" => Some(Self::AllusionChirho),
            "prophecyfulfillment" | "prophecy_fulfillment" => {
                Some(Self::ProphecyFulfillmentChirho)
            }
            "externaldataset" | "external_dataset" => Some(Self::ExternalDatasetChirho),
            _ => None,
        }
    }
}

/// A single directed cross-reference edge: source → target.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CrossRefEntryChirho {
    /// Source verse.
    pub source_chirho: VerseRefChirho,
    /// Target verse.
    pub target_chirho: VerseRefChirho,
    /// Type of cross-reference.
    pub xref_type_chirho: XRefTypeChirho,
    /// Confidence (1–5, higher = more certain).
    pub confidence_chirho: u8,
    /// Optional note/description.
    pub note_chirho: Option<String>,
    /// Source dataset identifier (e.g. "osis", "gbf", "parallel", "tsk").
    pub source_dataset_chirho: String,
}

impl CrossRefEntryChirho {
    /// Create a new cross-reference entry.
    pub fn new_chirho(
        source_chirho: VerseRefChirho,
        target_chirho: VerseRefChirho,
        xref_type_chirho: XRefTypeChirho,
    ) -> Self {
        Self {
            source_chirho,
            target_chirho,
            xref_type_chirho,
            confidence_chirho: 3,
            note_chirho: None,
            source_dataset_chirho: "unknown".to_string(),
        }
    }

    /// Set confidence level.
    pub fn with_confidence_chirho(mut self, confidence_chirho: u8) -> Self {
        self.confidence_chirho = confidence_chirho.clamp(1, 5);
        self
    }

    /// Set note.
    pub fn with_note_chirho(mut self, note_chirho: &str) -> Self {
        self.note_chirho = Some(note_chirho.to_string());
        self
    }

    /// Set source dataset.
    pub fn with_dataset_chirho(mut self, dataset_chirho: &str) -> Self {
        self.source_dataset_chirho = dataset_chirho.to_string();
        self
    }
}

/// Trait for a cross-reference graph store.
pub trait CrossRefGraphChirho {
    /// Error type for graph operations.
    type ErrorChirho: std::error::Error;

    /// Insert a single cross-reference edge.
    fn insert_xref_chirho(
        &self,
        entry_chirho: &CrossRefEntryChirho,
    ) -> Result<(), Self::ErrorChirho>;

    /// Insert a batch of cross-reference edges.
    fn insert_xrefs_batch_chirho(
        &self,
        entries_chirho: &[CrossRefEntryChirho],
    ) -> Result<usize, Self::ErrorChirho>;

    /// Get all cross-references FROM a given verse.
    fn xrefs_from_chirho(
        &self,
        source_chirho: &VerseRefChirho,
    ) -> Result<Vec<CrossRefEntryChirho>, Self::ErrorChirho>;

    /// Get all cross-references TO a given verse.
    fn xrefs_to_chirho(
        &self,
        target_chirho: &VerseRefChirho,
    ) -> Result<Vec<CrossRefEntryChirho>, Self::ErrorChirho>;

    /// BFS expansion from a seed verse, returning all verse refs reachable
    /// within `depth_chirho` hops. If `include_seed_chirho` is true, the
    /// seed itself is included in the result.
    fn bfs_expand_chirho(
        &self,
        seed_chirho: &VerseRefChirho,
        depth_chirho: u32,
        include_seed_chirho: bool,
    ) -> Result<Vec<VerseRefChirho>, Self::ErrorChirho>;

    /// Count total cross-reference edges in the store.
    fn count_xrefs_chirho(&self) -> Result<u64, Self::ErrorChirho>;
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_xref_type_display_chirho() {
        assert_eq!(XRefTypeChirho::DirectChirho.to_string(), "Direct");
        assert_eq!(XRefTypeChirho::ParallelChirho.to_string(), "Parallel");
        assert_eq!(XRefTypeChirho::QuotationChirho.to_string(), "Quotation");
        assert_eq!(XRefTypeChirho::AllusionChirho.to_string(), "Allusion");
        assert_eq!(
            XRefTypeChirho::ProphecyFulfillmentChirho.to_string(),
            "ProphecyFulfillment"
        );
        assert_eq!(
            XRefTypeChirho::ExternalDatasetChirho.to_string(),
            "ExternalDataset"
        );
    }

    #[test]
    fn test_xref_type_from_str_chirho() {
        assert_eq!(
            XRefTypeChirho::from_str_chirho("Direct"),
            Some(XRefTypeChirho::DirectChirho)
        );
        assert_eq!(
            XRefTypeChirho::from_str_chirho("parallel"),
            Some(XRefTypeChirho::ParallelChirho)
        );
        assert_eq!(
            XRefTypeChirho::from_str_chirho("QUOTATION"),
            Some(XRefTypeChirho::QuotationChirho)
        );
        assert_eq!(XRefTypeChirho::from_str_chirho("invalid"), None);
    }

    #[test]
    fn test_cross_ref_entry_new_chirho() {
        let source_chirho = VerseRefChirho::new_chirho("John", 3, 16).unwrap();
        let target_chirho = VerseRefChirho::new_chirho("Romans", 5, 8).unwrap();
        let entry_chirho = CrossRefEntryChirho::new_chirho(
            source_chirho.clone(),
            target_chirho.clone(),
            XRefTypeChirho::DirectChirho,
        );
        assert_eq!(entry_chirho.source_chirho, source_chirho);
        assert_eq!(entry_chirho.target_chirho, target_chirho);
        assert_eq!(entry_chirho.xref_type_chirho, XRefTypeChirho::DirectChirho);
        assert_eq!(entry_chirho.confidence_chirho, 3);
        assert!(entry_chirho.note_chirho.is_none());
        assert_eq!(entry_chirho.source_dataset_chirho, "unknown");
    }

    #[test]
    fn test_cross_ref_entry_builder_chirho() {
        let entry_chirho = CrossRefEntryChirho::new_chirho(
            VerseRefChirho::new_chirho("Gen", 1, 1).unwrap(),
            VerseRefChirho::new_chirho("John", 1, 1).unwrap(),
            XRefTypeChirho::AllusionChirho,
        )
        .with_confidence_chirho(5)
        .with_note_chirho("Creation theme")
        .with_dataset_chirho("osis");

        assert_eq!(entry_chirho.confidence_chirho, 5);
        assert_eq!(entry_chirho.note_chirho.as_deref(), Some("Creation theme"));
        assert_eq!(entry_chirho.source_dataset_chirho, "osis");
    }

    #[test]
    fn test_confidence_clamp_chirho() {
        let entry_chirho = CrossRefEntryChirho::new_chirho(
            VerseRefChirho::new_chirho("Gen", 1, 1).unwrap(),
            VerseRefChirho::new_chirho("Gen", 1, 2).unwrap(),
            XRefTypeChirho::DirectChirho,
        )
        .with_confidence_chirho(10);
        assert_eq!(entry_chirho.confidence_chirho, 5);

        let entry2_chirho = CrossRefEntryChirho::new_chirho(
            VerseRefChirho::new_chirho("Gen", 1, 1).unwrap(),
            VerseRefChirho::new_chirho("Gen", 1, 2).unwrap(),
            XRefTypeChirho::DirectChirho,
        )
        .with_confidence_chirho(0);
        assert_eq!(entry2_chirho.confidence_chirho, 1);
    }

    #[test]
    fn test_cross_ref_serde_roundtrip_chirho() {
        let entry_chirho = CrossRefEntryChirho::new_chirho(
            VerseRefChirho::new_chirho("Matthew", 1, 23).unwrap(),
            VerseRefChirho::new_chirho("Isaiah", 7, 14).unwrap(),
            XRefTypeChirho::ProphecyFulfillmentChirho,
        )
        .with_confidence_chirho(5)
        .with_note_chirho("Virgin birth prophecy");

        let json_chirho = serde_json::to_string(&entry_chirho).unwrap();
        let parsed_chirho: CrossRefEntryChirho = serde_json::from_str(&json_chirho).unwrap();
        assert_eq!(entry_chirho, parsed_chirho);
    }

    #[test]
    fn test_cross_ref_equality_chirho() {
        let a_chirho = CrossRefEntryChirho::new_chirho(
            VerseRefChirho::new_chirho("John", 3, 16).unwrap(),
            VerseRefChirho::new_chirho("Romans", 5, 8).unwrap(),
            XRefTypeChirho::DirectChirho,
        );
        let b_chirho = CrossRefEntryChirho::new_chirho(
            VerseRefChirho::new_chirho("John", 3, 16).unwrap(),
            VerseRefChirho::new_chirho("Romans", 5, 8).unwrap(),
            XRefTypeChirho::DirectChirho,
        );
        let c_chirho = CrossRefEntryChirho::new_chirho(
            VerseRefChirho::new_chirho("John", 3, 16).unwrap(),
            VerseRefChirho::new_chirho("Romans", 5, 9).unwrap(),
            XRefTypeChirho::DirectChirho,
        );
        assert_eq!(a_chirho, b_chirho);
        assert_ne!(a_chirho, c_chirho);
    }

    #[test]
    fn test_cross_ref_hash_dedup_chirho() {
        let mut set_chirho = std::collections::HashSet::new();
        let entry_chirho = CrossRefEntryChirho::new_chirho(
            VerseRefChirho::new_chirho("Ps", 23, 1).unwrap(),
            VerseRefChirho::new_chirho("John", 10, 11).unwrap(),
            XRefTypeChirho::AllusionChirho,
        );
        set_chirho.insert(entry_chirho.clone());
        set_chirho.insert(entry_chirho);
        assert_eq!(set_chirho.len(), 1);
    }

    #[test]
    fn test_xref_type_serde_roundtrip_chirho() {
        for variant_chirho in [
            XRefTypeChirho::DirectChirho,
            XRefTypeChirho::ParallelChirho,
            XRefTypeChirho::QuotationChirho,
            XRefTypeChirho::AllusionChirho,
            XRefTypeChirho::ProphecyFulfillmentChirho,
            XRefTypeChirho::ExternalDatasetChirho,
        ] {
            let json_chirho = serde_json::to_string(&variant_chirho).unwrap();
            let parsed_chirho: XRefTypeChirho = serde_json::from_str(&json_chirho).unwrap();
            assert_eq!(variant_chirho, parsed_chirho);
        }
    }
}
