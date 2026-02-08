// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Index manifest — tracks index metadata and version for cache invalidation.

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error_chirho::IndexErrorChirho;

/// Manifest file name within an index directory.
const MANIFEST_FILE_CHIRHO: &str = "rhema_manifest.json";

/// Current schema version. Bump when the index schema changes.
const SCHEMA_VERSION_CHIRHO: u32 = 1;

/// Manifest stored alongside a Tantivy index directory.
/// Allows the engine to detect stale indexes or schema changes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexManifestChirho {
    /// Schema version at time of indexing.
    pub schema_version_chirho: u32,
    /// Module name that was indexed.
    pub module_name_chirho: String,
    /// Number of documents (verses) in the index.
    pub document_count_chirho: usize,
    /// ISO 8601 timestamp when the index was built.
    pub created_at_chirho: String,
}

impl IndexManifestChirho {
    /// Create a new manifest for a freshly built index.
    pub fn new_chirho(module_name_chirho: &str, document_count_chirho: usize) -> Self {
        Self {
            schema_version_chirho: SCHEMA_VERSION_CHIRHO,
            module_name_chirho: module_name_chirho.to_string(),
            document_count_chirho,
            created_at_chirho: now_iso8601_chirho(),
        }
    }

    /// Write the manifest to the index directory.
    pub fn write_chirho(&self, index_dir_chirho: &Path) -> Result<(), IndexErrorChirho> {
        let path_chirho = index_dir_chirho.join(MANIFEST_FILE_CHIRHO);
        let json_chirho = serde_json::to_string_pretty(self)
            .map_err(|e_chirho| IndexErrorChirho::ManifestChirho {
                message_chirho: format!("Failed to serialize manifest: {e_chirho}"),
            })?;
        std::fs::write(&path_chirho, json_chirho)?;
        Ok(())
    }

    /// Read a manifest from an index directory.
    pub fn read_chirho(index_dir_chirho: &Path) -> Result<Self, IndexErrorChirho> {
        let path_chirho = index_dir_chirho.join(MANIFEST_FILE_CHIRHO);
        let json_chirho = std::fs::read_to_string(&path_chirho)?;
        serde_json::from_str(&json_chirho).map_err(|e_chirho| IndexErrorChirho::ManifestChirho {
            message_chirho: format!("Failed to parse manifest: {e_chirho}"),
        })
    }

    /// Check if this manifest's schema version is current.
    pub fn is_current_chirho(&self) -> bool {
        self.schema_version_chirho == SCHEMA_VERSION_CHIRHO
    }
}

/// Simple ISO 8601 timestamp without external deps.
fn now_iso8601_chirho() -> String {
    // Use a fixed format for reproducibility. In production, consider
    // chrono or time crate. For now we use a simple approach.
    let duration_chirho = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    format!("epoch:{}", duration_chirho.as_secs())
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_manifest_roundtrip_chirho() {
        let manifest_chirho = IndexManifestChirho::new_chirho("KJV", 31102);
        let json_chirho = serde_json::to_string(&manifest_chirho).unwrap();
        let parsed_chirho: IndexManifestChirho = serde_json::from_str(&json_chirho).unwrap();
        assert_eq!(parsed_chirho.module_name_chirho, "KJV");
        assert_eq!(parsed_chirho.document_count_chirho, 31102);
        assert!(parsed_chirho.is_current_chirho());
    }

    #[test]
    fn test_manifest_stale_chirho() {
        let mut manifest_chirho = IndexManifestChirho::new_chirho("ESV", 100);
        manifest_chirho.schema_version_chirho = 0;
        assert!(!manifest_chirho.is_current_chirho());
    }
}
