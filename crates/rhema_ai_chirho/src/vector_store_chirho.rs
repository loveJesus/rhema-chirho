// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Vector store — trait + SQLite implementation for portable vector search.

use rusqlite::{params, Connection};

use crate::embedding_chirho::cosine_similarity_chirho;
use crate::error_chirho::AiResultChirho;

/// A stored vector with metadata.
#[derive(Debug, Clone)]
pub struct VectorEntryChirho {
    /// Unique key (e.g. "John 3:16").
    pub key_chirho: String,
    /// The text that was embedded.
    pub text_chirho: String,
    /// Module name (e.g. "KJV").
    pub module_chirho: String,
    /// The embedding vector.
    pub embedding_chirho: Vec<f32>,
}

/// A search result from vector similarity.
#[derive(Debug, Clone)]
pub struct VectorSearchResultChirho {
    pub key_chirho: String,
    pub text_chirho: String,
    pub module_chirho: String,
    pub similarity_chirho: f32,
}

/// Trait for vector store implementations.
pub trait VectorStoreChirho {
    /// Store a vector entry.
    fn store_chirho(&self, entry_chirho: &VectorEntryChirho) -> AiResultChirho<()>;

    /// Store a batch of vector entries.
    fn store_batch_chirho(&self, entries_chirho: &[VectorEntryChirho]) -> AiResultChirho<()>;

    /// Search for similar vectors.
    fn search_similar_chirho(
        &self,
        query_embedding_chirho: &[f32],
        module_chirho: &str,
        top_k_chirho: usize,
        threshold_chirho: f32,
    ) -> AiResultChirho<Vec<VectorSearchResultChirho>>;

    /// Check if a module has been indexed.
    fn has_module_chirho(&self, module_chirho: &str) -> AiResultChirho<bool>;

    /// Get the total number of stored vectors.
    fn count_chirho(&self) -> AiResultChirho<u64>;

    /// Delete all vectors for a module.
    fn delete_module_chirho(&self, module_chirho: &str) -> AiResultChirho<()>;
}

/// SQLite-backed vector store — portable, single-file, WASM-compatible.
pub struct SqliteVectorStoreChirho {
    conn_chirho: Connection,
}

impl SqliteVectorStoreChirho {
    /// Open or create a vector store at the given path.
    pub fn open_chirho(path_chirho: &str) -> AiResultChirho<Self> {
        let conn_chirho = Connection::open(path_chirho)?;
        let store_chirho = Self { conn_chirho };
        store_chirho.init_schema_chirho()?;
        Ok(store_chirho)
    }

    /// Create an in-memory vector store (for testing).
    pub fn in_memory_chirho() -> AiResultChirho<Self> {
        let conn_chirho = Connection::open_in_memory()?;
        let store_chirho = Self { conn_chirho };
        store_chirho.init_schema_chirho()?;
        Ok(store_chirho)
    }

    fn init_schema_chirho(&self) -> AiResultChirho<()> {
        self.conn_chirho.execute_batch(
            "CREATE TABLE IF NOT EXISTS vectors_chirho (
                id_chirho INTEGER PRIMARY KEY AUTOINCREMENT,
                key_chirho TEXT NOT NULL,
                text_chirho TEXT NOT NULL,
                module_chirho TEXT NOT NULL,
                embedding_chirho BLOB NOT NULL,
                UNIQUE(key_chirho, module_chirho)
            );
            CREATE INDEX IF NOT EXISTS idx_vectors_module_chirho
                ON vectors_chirho(module_chirho);",
        )?;
        Ok(())
    }

    /// Serialize an f32 vector to bytes.
    fn serialize_embedding_chirho(embedding_chirho: &[f32]) -> Vec<u8> {
        let mut bytes_chirho = Vec::with_capacity(embedding_chirho.len() * 4);
        for val_chirho in embedding_chirho {
            bytes_chirho.extend_from_slice(&val_chirho.to_le_bytes());
        }
        bytes_chirho
    }

    /// Deserialize bytes back to an f32 vector.
    fn deserialize_embedding_chirho(bytes_chirho: &[u8]) -> Vec<f32> {
        bytes_chirho
            .chunks_exact(4)
            .map(|chunk_chirho| f32::from_le_bytes(chunk_chirho.try_into().unwrap()))
            .collect()
    }
}

impl VectorStoreChirho for SqliteVectorStoreChirho {
    fn store_chirho(&self, entry_chirho: &VectorEntryChirho) -> AiResultChirho<()> {
        let blob_chirho = Self::serialize_embedding_chirho(&entry_chirho.embedding_chirho);
        self.conn_chirho.execute(
            "INSERT OR REPLACE INTO vectors_chirho
                (key_chirho, text_chirho, module_chirho, embedding_chirho)
                VALUES (?1, ?2, ?3, ?4)",
            params![
                entry_chirho.key_chirho,
                entry_chirho.text_chirho,
                entry_chirho.module_chirho,
                blob_chirho,
            ],
        )?;
        Ok(())
    }

    fn store_batch_chirho(&self, entries_chirho: &[VectorEntryChirho]) -> AiResultChirho<()> {
        let tx_chirho = self.conn_chirho.unchecked_transaction()?;
        for entry_chirho in entries_chirho {
            let blob_chirho = Self::serialize_embedding_chirho(&entry_chirho.embedding_chirho);
            tx_chirho.execute(
                "INSERT OR REPLACE INTO vectors_chirho
                    (key_chirho, text_chirho, module_chirho, embedding_chirho)
                    VALUES (?1, ?2, ?3, ?4)",
                params![
                    entry_chirho.key_chirho,
                    entry_chirho.text_chirho,
                    entry_chirho.module_chirho,
                    blob_chirho,
                ],
            )?;
        }
        tx_chirho.commit()?;
        Ok(())
    }

    fn search_similar_chirho(
        &self,
        query_embedding_chirho: &[f32],
        module_chirho: &str,
        top_k_chirho: usize,
        threshold_chirho: f32,
    ) -> AiResultChirho<Vec<VectorSearchResultChirho>> {
        // Load all vectors for the module and compute similarity in Rust.
        // This is O(n) brute force — suitable for Bible-scale data (~31k verses).
        // For larger datasets, consider ANN indexes.
        let mut stmt_chirho = self.conn_chirho.prepare(
            "SELECT key_chirho, text_chirho, module_chirho, embedding_chirho
             FROM vectors_chirho WHERE module_chirho = ?1",
        )?;

        let mut results_chirho: Vec<VectorSearchResultChirho> = stmt_chirho
            .query_map(params![module_chirho], |row_chirho| {
                let key_chirho: String = row_chirho.get(0)?;
                let text_chirho: String = row_chirho.get(1)?;
                let module_chirho: String = row_chirho.get(2)?;
                let blob_chirho: Vec<u8> = row_chirho.get(3)?;
                Ok((key_chirho, text_chirho, module_chirho, blob_chirho))
            })?
            .filter_map(|row_result_chirho| {
                let (key_chirho, text_chirho, module_chirho, blob_chirho) =
                    row_result_chirho.ok()?;
                let embedding_chirho = Self::deserialize_embedding_chirho(&blob_chirho);
                let similarity_chirho =
                    cosine_similarity_chirho(query_embedding_chirho, &embedding_chirho);
                if similarity_chirho >= threshold_chirho {
                    Some(VectorSearchResultChirho {
                        key_chirho,
                        text_chirho,
                        module_chirho,
                        similarity_chirho,
                    })
                } else {
                    None
                }
            })
            .collect();

        // Sort by similarity descending and take top-k.
        results_chirho
            .sort_by(|a_chirho, b_chirho| {
                b_chirho
                    .similarity_chirho
                    .partial_cmp(&a_chirho.similarity_chirho)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
        results_chirho.truncate(top_k_chirho);

        Ok(results_chirho)
    }

    fn has_module_chirho(&self, module_chirho: &str) -> AiResultChirho<bool> {
        let count_chirho: u64 = self.conn_chirho.query_row(
            "SELECT COUNT(*) FROM vectors_chirho WHERE module_chirho = ?1",
            params![module_chirho],
            |row_chirho| row_chirho.get(0),
        )?;
        Ok(count_chirho > 0)
    }

    fn count_chirho(&self) -> AiResultChirho<u64> {
        let count_chirho: u64 = self.conn_chirho.query_row(
            "SELECT COUNT(*) FROM vectors_chirho",
            [],
            |row_chirho| row_chirho.get(0),
        )?;
        Ok(count_chirho)
    }

    fn delete_module_chirho(&self, module_chirho: &str) -> AiResultChirho<()> {
        self.conn_chirho.execute(
            "DELETE FROM vectors_chirho WHERE module_chirho = ?1",
            params![module_chirho],
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    fn make_entry_chirho(
        key_chirho: &str,
        text_chirho: &str,
        embedding_chirho: Vec<f32>,
    ) -> VectorEntryChirho {
        VectorEntryChirho {
            key_chirho: key_chirho.to_string(),
            text_chirho: text_chirho.to_string(),
            module_chirho: "KJV".to_string(),
            embedding_chirho,
        }
    }

    #[test]
    fn test_store_and_count_chirho() {
        let store_chirho = SqliteVectorStoreChirho::in_memory_chirho().unwrap();
        assert_eq!(store_chirho.count_chirho().unwrap(), 0);

        let entry_chirho = make_entry_chirho("John 3:16", "For God so loved", vec![1.0, 0.0, 0.0]);
        store_chirho.store_chirho(&entry_chirho).unwrap();
        assert_eq!(store_chirho.count_chirho().unwrap(), 1);
    }

    #[test]
    fn test_store_batch_chirho() {
        let store_chirho = SqliteVectorStoreChirho::in_memory_chirho().unwrap();
        let entries_chirho = vec![
            make_entry_chirho("John 3:16", "For God so loved", vec![1.0, 0.0, 0.0]),
            make_entry_chirho("John 3:17", "For God sent not", vec![0.0, 1.0, 0.0]),
        ];
        store_chirho.store_batch_chirho(&entries_chirho).unwrap();
        assert_eq!(store_chirho.count_chirho().unwrap(), 2);
    }

    #[test]
    fn test_search_similar_chirho() {
        let store_chirho = SqliteVectorStoreChirho::in_memory_chirho().unwrap();
        let entries_chirho = vec![
            make_entry_chirho("John 3:16", "For God so loved", vec![1.0, 0.0, 0.0]),
            make_entry_chirho("John 3:17", "For God sent not", vec![0.9, 0.1, 0.0]),
            make_entry_chirho("Gen 1:1", "In the beginning", vec![0.0, 0.0, 1.0]),
        ];
        store_chirho.store_batch_chirho(&entries_chirho).unwrap();

        let query_chirho = vec![1.0, 0.0, 0.0];
        let results_chirho = store_chirho
            .search_similar_chirho(&query_chirho, "KJV", 2, 0.5)
            .unwrap();
        assert_eq!(results_chirho.len(), 2);
        assert_eq!(results_chirho[0].key_chirho, "John 3:16");
    }

    #[test]
    fn test_has_module_chirho() {
        let store_chirho = SqliteVectorStoreChirho::in_memory_chirho().unwrap();
        assert!(!store_chirho.has_module_chirho("KJV").unwrap());

        let entry_chirho = make_entry_chirho("John 3:16", "For God so loved", vec![1.0]);
        store_chirho.store_chirho(&entry_chirho).unwrap();
        assert!(store_chirho.has_module_chirho("KJV").unwrap());
    }

    #[test]
    fn test_delete_module_chirho() {
        let store_chirho = SqliteVectorStoreChirho::in_memory_chirho().unwrap();
        let entry_chirho = make_entry_chirho("John 3:16", "For God so loved", vec![1.0]);
        store_chirho.store_chirho(&entry_chirho).unwrap();
        assert_eq!(store_chirho.count_chirho().unwrap(), 1);

        store_chirho.delete_module_chirho("KJV").unwrap();
        assert_eq!(store_chirho.count_chirho().unwrap(), 0);
    }

    #[test]
    fn test_upsert_chirho() {
        let store_chirho = SqliteVectorStoreChirho::in_memory_chirho().unwrap();
        let entry1_chirho = make_entry_chirho("John 3:16", "Original text", vec![1.0, 0.0]);
        store_chirho.store_chirho(&entry1_chirho).unwrap();

        let entry2_chirho = make_entry_chirho("John 3:16", "Updated text", vec![0.5, 0.5]);
        store_chirho.store_chirho(&entry2_chirho).unwrap();

        assert_eq!(store_chirho.count_chirho().unwrap(), 1);
    }

    #[test]
    fn test_embedding_serialization_roundtrip_chirho() {
        let original_chirho = vec![1.0f32, -0.5, 0.0, 3.14159];
        let bytes_chirho = SqliteVectorStoreChirho::serialize_embedding_chirho(&original_chirho);
        let restored_chirho = SqliteVectorStoreChirho::deserialize_embedding_chirho(&bytes_chirho);
        assert_eq!(original_chirho, restored_chirho);
    }
}
