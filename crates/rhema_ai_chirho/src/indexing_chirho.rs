// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Batch embedding pipeline — generate and store embeddings for Bible modules.

use crate::embedding_chirho::EmbeddingModelChirho;
use crate::error_chirho::AiResultChirho;
use crate::vector_store_chirho::{VectorEntryChirho, VectorStoreChirho};

/// Progress callback for embedding indexing.
pub type ProgressCallbackChirho = Box<dyn Fn(usize, usize) + Send>;

/// Configuration for batch embedding indexing.
#[derive(Debug, Clone)]
pub struct IndexingConfigChirho {
    /// Number of texts to embed in a single batch.
    pub batch_size_chirho: usize,
    /// Module name being indexed.
    pub module_name_chirho: String,
}

impl Default for IndexingConfigChirho {
    fn default() -> Self {
        Self {
            batch_size_chirho: 32,
            module_name_chirho: String::new(),
        }
    }
}

/// A verse entry to be embedded.
#[derive(Debug, Clone)]
pub struct VerseEntryChirho {
    /// Verse reference key (e.g. "John 3:16").
    pub key_chirho: String,
    /// Verse text.
    pub text_chirho: String,
}

/// Batch embedding indexer — generates embeddings and stores them in the vector store.
pub struct EmbeddingIndexerChirho<'a> {
    model_chirho: &'a dyn EmbeddingModelChirho,
    store_chirho: &'a dyn VectorStoreChirho,
}

impl<'a> EmbeddingIndexerChirho<'a> {
    pub fn new_chirho(
        model_chirho: &'a dyn EmbeddingModelChirho,
        store_chirho: &'a dyn VectorStoreChirho,
    ) -> Self {
        Self {
            model_chirho,
            store_chirho,
        }
    }

    /// Index a batch of verses — generate embeddings and store them.
    pub async fn index_verses_chirho(
        &self,
        verses_chirho: &[VerseEntryChirho],
        config_chirho: &IndexingConfigChirho,
        progress_chirho: Option<&ProgressCallbackChirho>,
    ) -> AiResultChirho<usize> {
        let total_chirho = verses_chirho.len();
        let mut indexed_chirho = 0usize;

        for chunk_chirho in verses_chirho.chunks(config_chirho.batch_size_chirho) {
            let texts_chirho: Vec<String> =
                chunk_chirho.iter().map(|v_chirho| v_chirho.text_chirho.clone()).collect();

            let embeddings_chirho = self.model_chirho.embed_batch_chirho(&texts_chirho).await?;

            let entries_chirho: Vec<VectorEntryChirho> = chunk_chirho
                .iter()
                .zip(embeddings_chirho.into_iter())
                .map(|(verse_chirho, embedding_chirho)| VectorEntryChirho {
                    key_chirho: verse_chirho.key_chirho.clone(),
                    text_chirho: verse_chirho.text_chirho.clone(),
                    module_chirho: config_chirho.module_name_chirho.clone(),
                    embedding_chirho,
                })
                .collect();

            self.store_chirho.store_batch_chirho(&entries_chirho)?;
            indexed_chirho += entries_chirho.len();

            if let Some(cb_chirho) = progress_chirho {
                cb_chirho(indexed_chirho, total_chirho);
            }
        }

        Ok(indexed_chirho)
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use crate::embedding_chirho::MockEmbeddingModelChirho;
    use crate::vector_store_chirho::SqliteVectorStoreChirho;

    #[tokio::test]
    async fn test_index_verses_chirho() {
        let model_chirho = MockEmbeddingModelChirho::new_chirho(64);
        let store_chirho = SqliteVectorStoreChirho::in_memory_chirho().unwrap();
        let indexer_chirho = EmbeddingIndexerChirho::new_chirho(&model_chirho, &store_chirho);

        let verses_chirho = vec![
            VerseEntryChirho {
                key_chirho: "John 3:16".to_string(),
                text_chirho: "For God so loved the world".to_string(),
            },
            VerseEntryChirho {
                key_chirho: "John 3:17".to_string(),
                text_chirho: "For God sent not his Son".to_string(),
            },
            VerseEntryChirho {
                key_chirho: "Rom 8:28".to_string(),
                text_chirho: "And we know that all things work together".to_string(),
            },
        ];

        let config_chirho = IndexingConfigChirho {
            batch_size_chirho: 2,
            module_name_chirho: "KJV".to_string(),
        };

        let count_chirho = indexer_chirho
            .index_verses_chirho(&verses_chirho, &config_chirho, None)
            .await
            .unwrap();

        assert_eq!(count_chirho, 3);
        assert_eq!(store_chirho.count_chirho().unwrap(), 3);
        assert!(store_chirho.has_module_chirho("KJV").unwrap());
    }

    #[tokio::test]
    async fn test_index_with_progress_chirho() {
        let model_chirho = MockEmbeddingModelChirho::new_chirho(32);
        let store_chirho = SqliteVectorStoreChirho::in_memory_chirho().unwrap();
        let indexer_chirho = EmbeddingIndexerChirho::new_chirho(&model_chirho, &store_chirho);

        let verses_chirho = vec![
            VerseEntryChirho {
                key_chirho: "Gen 1:1".to_string(),
                text_chirho: "In the beginning".to_string(),
            },
        ];

        let config_chirho = IndexingConfigChirho {
            batch_size_chirho: 10,
            module_name_chirho: "KJV".to_string(),
        };

        let progress_called_chirho = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let progress_clone_chirho = progress_called_chirho.clone();
        let callback_chirho: ProgressCallbackChirho =
            Box::new(move |_done_chirho, _total_chirho| {
                progress_clone_chirho.store(true, std::sync::atomic::Ordering::SeqCst);
            });

        indexer_chirho
            .index_verses_chirho(&verses_chirho, &config_chirho, Some(&callback_chirho))
            .await
            .unwrap();

        assert!(progress_called_chirho.load(std::sync::atomic::Ordering::SeqCst));
    }
}
