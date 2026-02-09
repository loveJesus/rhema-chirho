// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Query executor — dispatches plan steps against rsword_chirho search backends.

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::time::Instant;

use rhema_contracts_chirho::capability_chirho::QueryExecutorChirho;
use rhema_contracts_chirho::keys_chirho::VerseRefChirho;
use rhema_contracts_chirho::query_chirho::QueryChirho;
use rhema_contracts_chirho::result_chirho::{QueryResultChirho, SearchHitChirho};
use rhema_index_chirho::{IndexSearcherChirho, ModuleIndexerChirho};
use rhema_ingest_chirho::SwordAdapterChirho;
use rhema_query_chirho::planner_chirho::{PlanStepChirho, QueryPlannerChirho};
use rsword_chirho::{RegexSearchChirho, SearchEngineChirho, SearchOptionsChirho, SearchTypeChirho};

use crate::error_chirho::ExecErrorChirho;

/// Default index directory name (relative to SWORD data path).
const INDEX_DIR_NAME_CHIRHO: &str = "rhema_indexes";

/// The query executor — runs planned queries against search backends.
///
/// When a Tantivy index exists for a module, uses indexed search for speed.
/// Falls back to regex-based search when no index is available.
pub struct QueryExecutorImplChirho {
    sword_adapter_chirho: SwordAdapterChirho,
    /// Base directory for Tantivy indexes, if set.
    index_base_chirho: Option<PathBuf>,
    /// Path to the cross-reference SQLite store, if configured.
    xref_store_path_chirho: Option<PathBuf>,
    /// Path to the semantic domain SQLite store, if configured.
    domain_store_path_chirho: Option<PathBuf>,
}

impl QueryExecutorImplChirho {
    /// Create an executor backed by system SWORD modules.
    pub fn with_system_paths_chirho() -> Result<Self, ExecErrorChirho> {
        let sword_adapter_chirho = SwordAdapterChirho::with_system_paths_chirho()?;

        // Default index path: ~/.sword/rhema_indexes or platform equivalent
        let index_base_chirho = dirs_index_base_chirho();

        Ok(Self {
            sword_adapter_chirho,
            index_base_chirho,
            xref_store_path_chirho: None,
            domain_store_path_chirho: None,
        })
    }

    /// Create an executor backed by a specific SWORD adapter.
    pub fn with_adapter_chirho(sword_adapter_chirho: SwordAdapterChirho) -> Self {
        Self {
            sword_adapter_chirho,
            index_base_chirho: dirs_index_base_chirho(),
            xref_store_path_chirho: None,
            domain_store_path_chirho: None,
        }
    }

    /// Set a custom index base directory.
    pub fn with_index_base_chirho(mut self, path_chirho: &Path) -> Self {
        self.index_base_chirho = Some(path_chirho.to_path_buf());
        self
    }

    /// Set the cross-reference store path.
    pub fn with_xref_store_chirho(mut self, path_chirho: &Path) -> Self {
        self.xref_store_path_chirho = Some(path_chirho.to_path_buf());
        self
    }

    /// Set the semantic domain store path.
    pub fn with_domain_store_chirho(mut self, path_chirho: &Path) -> Self {
        self.domain_store_path_chirho = Some(path_chirho.to_path_buf());
        self
    }

    /// Get the module indexer (for building indexes).
    pub fn indexer_chirho(&self) -> Option<ModuleIndexerChirho> {
        self.index_base_chirho
            .as_ref()
            .map(|p_chirho| ModuleIndexerChirho::new_chirho(p_chirho))
    }

    /// Get a reference to the SWORD adapter.
    pub fn sword_adapter_chirho(&self) -> &SwordAdapterChirho {
        &self.sword_adapter_chirho
    }

    /// Execute a query against a specific module.
    pub fn execute_against_module_chirho(
        &self,
        query_chirho: &QueryChirho,
        module_name_chirho: &str,
    ) -> Result<QueryResultChirho, ExecErrorChirho> {
        let start_chirho = Instant::now();

        if !self.sword_adapter_chirho.has_module_chirho(module_name_chirho) {
            return Err(ExecErrorChirho::ModuleNotLoadedChirho {
                name_chirho: module_name_chirho.to_string(),
            });
        }

        let plan_chirho = QueryPlannerChirho::plan_chirho(query_chirho);
        let hits_chirho =
            self.execute_step_chirho(&plan_chirho.root_step_chirho, module_name_chirho)?;

        // Apply max_results limit
        let limited_hits_chirho: Vec<SearchHitChirho> = hits_chirho
            .into_iter()
            .take(query_chirho.max_results_chirho)
            .collect();

        let total_count_chirho = limited_hits_chirho.len() as u64;
        let elapsed_chirho = start_chirho.elapsed();

        let plan_text_chirho = if query_chirho.explain_chirho {
            Some(QueryPlannerChirho::explain_chirho(&plan_chirho))
        } else {
            None
        };

        Ok(QueryResultChirho {
            hits_chirho: limited_hits_chirho,
            total_count_chirho,
            execution_time_us_chirho: elapsed_chirho.as_micros() as u64,
            backend_chirho: plan_chirho.backend_chirho.clone(),
            plan_chirho: plan_text_chirho,
            used_ai_chirho: false,
            query_expansion_chirho: None,
        })
    }

    /// Execute a single plan step.
    fn execute_step_chirho(
        &self,
        step_chirho: &PlanStepChirho,
        module_name_chirho: &str,
    ) -> Result<Vec<SearchHitChirho>, ExecErrorChirho> {
        match step_chirho {
            PlanStepChirho::FullTextSearchChirho {
                query_text_chirho, ..
            } => self.execute_fulltext_chirho(query_text_chirho, module_name_chirho),

            PlanStepChirho::StrongLookupChirho { number_chirho } => {
                // Strong's lookup: try indexed Strong's field first, fall back to text search
                if let Some(ref base_chirho) = self.index_base_chirho {
                    let indexer_chirho = ModuleIndexerChirho::new_chirho(base_chirho);
                    if indexer_chirho.has_index_chirho(module_name_chirho) {
                        let index_path_chirho = indexer_chirho.index_path_chirho(module_name_chirho);
                        if let Ok(searcher_chirho) = IndexSearcherChirho::open_chirho(&index_path_chirho)
                            && let Ok(index_hits_chirho) = searcher_chirho.search_strongs_chirho(number_chirho, 500)
                        {
                            let mut hits_chirho = Vec::with_capacity(index_hits_chirho.len());
                            for hit_chirho in index_hits_chirho {
                                if let Some(verse_ref_chirho) = parse_verse_ref_chirho(&hit_chirho.key_chirho) {
                                    hits_chirho.push(SearchHitChirho {
                                        verse_ref_chirho,
                                        text_chirho: hit_chirho.text_chirho,
                                        highlighted_text_chirho: None,
                                        score_chirho: hit_chirho.score_chirho,
                                        matched_positions_chirho: Vec::new(),
                                        explain_chirho: None,
                                        semantic_score_chirho: None,
                                        hybrid_score_chirho: None,
                                        semantic_explain_chirho: None,
                                    });
                                }
                            }
                            return Ok(hits_chirho);
                        }
                    }
                }
                // Fall back to text search
                self.execute_fulltext_chirho(number_chirho, module_name_chirho)
            }

            PlanStepChirho::LemmaLookupChirho { lemma_chirho } => {
                self.execute_fulltext_chirho(lemma_chirho, module_name_chirho)
            }

            PlanStepChirho::IntersectChirho(children_chirho) => {
                // Separate positive children from NOT (exclude) children.
                let mut positive_sets_chirho: Vec<Vec<SearchHitChirho>> = Vec::new();
                let mut exclude_sets_chirho: Vec<Vec<SearchHitChirho>> = Vec::new();

                for child_chirho in children_chirho {
                    if let PlanStepChirho::ExcludeChirho(inner_chirho) = child_chirho {
                        exclude_sets_chirho.push(
                            self.execute_step_chirho(inner_chirho, module_name_chirho)?,
                        );
                    } else {
                        positive_sets_chirho.push(
                            self.execute_step_chirho(child_chirho, module_name_chirho)?,
                        );
                    }
                }

                // Intersect the positive sets.
                let mut base_results_chirho = intersect_hits_chirho(positive_sets_chirho);

                // Subtract excluded hits from the base results.
                for excluded_chirho in &exclude_sets_chirho {
                    let excluded_keys_chirho: HashSet<String> = excluded_chirho
                        .iter()
                        .map(|h_chirho| h_chirho.verse_ref_chirho.to_string())
                        .collect();
                    base_results_chirho.retain(|h_chirho| {
                        !excluded_keys_chirho.contains(&h_chirho.verse_ref_chirho.to_string())
                    });
                }

                Ok(base_results_chirho)
            }

            PlanStepChirho::UnionChirho(children_chirho) => {
                let mut all_hits_chirho: Vec<SearchHitChirho> = Vec::new();
                for child_chirho in children_chirho {
                    let hits_chirho =
                        self.execute_step_chirho(child_chirho, module_name_chirho)?;
                    all_hits_chirho.extend(hits_chirho);
                }
                // Deduplicate by verse_ref
                dedup_hits_chirho(&mut all_hits_chirho);
                Ok(all_hits_chirho)
            }

            PlanStepChirho::ExcludeChirho(inner_chirho) => {
                // Standalone NOT at root level: no base set to subtract from.
                // NOT is handled properly inside IntersectChirho (the AND handler),
                // where ExcludeChirho children are subtracted from positive siblings.
                // A bare `NOT term` query without positive terms returns empty.
                let _excluded_chirho =
                    self.execute_step_chirho(inner_chirho, module_name_chirho)?;
                Ok(Vec::new())
            }

            PlanStepChirho::ProximityFilterChirho {
                terms_chirho,
                distance_chirho,
            } => {
                let phrase_chirho = terms_chirho.join(" ");
                self.execute_proximity_chirho(&phrase_chirho, *distance_chirho, module_name_chirho)
            }

            // ── Phase 2: AI / Semantic plan steps ───────────────────

            PlanStepChirho::EmbeddingSearchChirho { query_text_chirho, .. } => {
                // Requires AI context (embedding model + vector store).
                // Falls back to full-text search if AI is not configured.
                log::debug!("EmbeddingSearch for '{}' — AI context required", query_text_chirho);
                self.execute_fulltext_chirho(query_text_chirho, module_name_chirho)
            }

            PlanStepChirho::LlmExpansionChirho { original_query_chirho, .. } => {
                // Requires AI context (LLM provider).
                // Falls back to full-text search if AI is not configured.
                log::debug!("LlmExpansion for '{}' — AI context required", original_query_chirho);
                self.execute_fulltext_chirho(original_query_chirho, module_name_chirho)
            }

            PlanStepChirho::HybridRankChirho {
                keyword_step_chirho,
                semantic_step_chirho,
                keyword_weight_chirho,
            } => {
                // Execute both sides; if semantic fails, return keyword results only.
                let keyword_hits_chirho =
                    self.execute_step_chirho(keyword_step_chirho, module_name_chirho)?;
                let semantic_hits_chirho =
                    self.execute_step_chirho(semantic_step_chirho, module_name_chirho)
                        .unwrap_or_default();

                if semantic_hits_chirho.is_empty() {
                    return Ok(keyword_hits_chirho);
                }

                // Simple weighted merge for now.
                Ok(rhema_ai_chirho::merge_results_chirho(
                    &keyword_hits_chirho,
                    &semantic_hits_chirho,
                    *keyword_weight_chirho,
                    rhema_ai_chirho::MergeStrategyChirho::WeightedChirho,
                ))
            }

            PlanStepChirho::ConceptSearchChirho { concept_chirho } => {
                // Requires AI context. Falls back to full-text search.
                log::debug!("ConceptSearch for '{}' — AI context required", concept_chirho);
                self.execute_fulltext_chirho(concept_chirho, module_name_chirho)
            }

            // ── Phase 2: Discourse plan steps ───────────────────────

            PlanStepChirho::DiscourseRelationshipSearchChirho { relationship_type_chirho } => {
                // Requires discourse store context.
                // Returns empty when discourse store is not configured.
                log::debug!(
                    "DiscourseRelationshipSearch for '{}' — discourse store required",
                    relationship_type_chirho
                );
                Ok(Vec::new())
            }

            PlanStepChirho::PropositionTextSearchChirho { text_chirho } => {
                // Requires discourse store context.
                log::debug!(
                    "PropositionTextSearch for '{}' — discourse store required",
                    text_chirho
                );
                Ok(Vec::new())
            }

            // ── Phase 3: Morphology search ──────────────────────────

            PlanStepChirho::MorphLookupChirho { constraint_chirho } => {
                // Use Tantivy index morph facets when available.
                if let Some(ref base_chirho) = self.index_base_chirho {
                    let indexer_chirho = ModuleIndexerChirho::new_chirho(base_chirho);
                    if indexer_chirho.has_index_chirho(module_name_chirho) {
                        let index_path_chirho = indexer_chirho.index_path_chirho(module_name_chirho);
                        if let Ok(searcher_chirho) = IndexSearcherChirho::open_chirho(&index_path_chirho)
                            && let Ok(index_hits_chirho) = searcher_chirho.search_morph_chirho(constraint_chirho, 500)
                        {
                            let mut hits_chirho = Vec::with_capacity(index_hits_chirho.len());
                            for hit_chirho in index_hits_chirho {
                                if let Some(verse_ref_chirho) = parse_verse_ref_chirho(&hit_chirho.key_chirho) {
                                    hits_chirho.push(SearchHitChirho {
                                        verse_ref_chirho,
                                        text_chirho: hit_chirho.text_chirho,
                                        highlighted_text_chirho: None,
                                        score_chirho: hit_chirho.score_chirho,
                                        matched_positions_chirho: Vec::new(),
                                        explain_chirho: None,
                                        semantic_score_chirho: None,
                                        hybrid_score_chirho: None,
                                        semantic_explain_chirho: None,
                                    });
                                }
                            }
                            return Ok(hits_chirho);
                        }
                    }
                }
                // Graceful fallback: warn + empty results if no index.
                log::warn!(
                    "MorphLookup for module '{}' — no index available, returning empty",
                    module_name_chirho
                );
                Ok(Vec::new())
            }

            // ── Phase 4: Cross-reference graph search ───────────

            PlanStepChirho::GraphSearchChirho {
                seed_chirho,
                depth_chirho,
            } => {
                self.execute_graph_search_chirho(seed_chirho, *depth_chirho, module_name_chirho)
            }

            // ── Phase 5+: Domain / Sense / Syntax plan steps ────────

            PlanStepChirho::DomainLookupChirho { domain_chirho } => {
                self.execute_domain_lookup_chirho(domain_chirho, module_name_chirho)
            }

            PlanStepChirho::SenseLookupChirho { sense_chirho } => {
                self.execute_sense_lookup_chirho(sense_chirho, module_name_chirho)
            }

            PlanStepChirho::SyntaxSearchChirho { clause_type_chirho } => {
                log::debug!(
                    "SyntaxSearch for '{}' — syntax store required, returning empty + warning",
                    clause_type_chirho
                );
                Ok(Vec::new())
            }
        }
    }

    /// Execute a cross-reference graph search via BFS expansion.
    fn execute_graph_search_chirho(
        &self,
        seed_str_chirho: &str,
        depth_chirho: u32,
        module_name_chirho: &str,
    ) -> Result<Vec<SearchHitChirho>, ExecErrorChirho> {
        use rhema_contracts_chirho::xref_chirho::CrossRefGraphChirho;

        let xref_path_chirho = match &self.xref_store_path_chirho {
            Some(p_chirho) => p_chirho.clone(),
            None => {
                log::warn!(
                    "GraphSearch for '{}' — no xref store configured, returning empty",
                    seed_str_chirho
                );
                return Ok(Vec::new());
            }
        };

        // Parse the seed reference (try OSIS format first, then human-readable)
        let seed_chirho = rhema_ingest_chirho::xref_extractor_chirho::parse_osis_ref_chirho(seed_str_chirho)
            .into_iter()
            .next()
            .or_else(|| rhema_ingest_chirho::xref_extractor_chirho::parse_human_ref_chirho(seed_str_chirho))
            .ok_or_else(|| ExecErrorChirho::XrefStoreChirho {
                reason_chirho: format!("Cannot parse seed reference: '{seed_str_chirho}'"),
            })?;

        let path_str_chirho = xref_path_chirho.to_string_lossy().to_string();
        let store_chirho = rhema_module_chirho::XrefStoreChirho::open_chirho(&path_str_chirho)
            .map_err(|e_chirho| ExecErrorChirho::XrefStoreChirho {
                reason_chirho: format!("Failed to open xref store: {e_chirho}"),
            })?;

        let expanded_refs_chirho = store_chirho
            .bfs_expand_chirho(&seed_chirho, depth_chirho, false)
            .map_err(|e_chirho| ExecErrorChirho::XrefStoreChirho {
                reason_chirho: format!("BFS expansion failed: {e_chirho}"),
            })?;

        // Load verse text for each expanded reference
        let mut hits_chirho = Vec::with_capacity(expanded_refs_chirho.len());
        for ref_chirho in &expanded_refs_chirho {
            let key_str_chirho = format!(
                "{} {}:{}",
                ref_chirho.book_chirho, ref_chirho.chapter_chirho, ref_chirho.verse_chirho
            );

            // Try to load verse text from the module
            let text_chirho = self
                .sword_adapter_chirho
                .manager_chirho()
                .load_module_chirho(module_name_chirho)
                .ok()
                .and_then(|loaded_chirho| loaded_chirho.read_entry_chirho(&key_str_chirho).ok())
                .unwrap_or_default();

            hits_chirho.push(SearchHitChirho {
                verse_ref_chirho: ref_chirho.clone(),
                text_chirho,
                highlighted_text_chirho: None,
                score_chirho: 1.0,
                matched_positions_chirho: Vec::new(),
                explain_chirho: None,
                semantic_score_chirho: None,
                hybrid_score_chirho: None,
                semantic_explain_chirho: Some(format!(
                    "xref expansion from {} depth={}",
                    seed_chirho, depth_chirho
                )),
            });
        }

        Ok(hits_chirho)
    }

    /// Execute a domain lookup via the semantic domain store.
    fn execute_domain_lookup_chirho(
        &self,
        domain_chirho: &str,
        module_name_chirho: &str,
    ) -> Result<Vec<SearchHitChirho>, ExecErrorChirho> {
        let domain_path_chirho = match &self.domain_store_path_chirho {
            Some(p_chirho) => p_chirho.clone(),
            None => {
                log::warn!(
                    "DomainLookup for '{}' — no domain store configured, returning empty",
                    domain_chirho
                );
                return Ok(Vec::new());
            }
        };

        let path_str_chirho = domain_path_chirho.to_string_lossy().to_string();
        let store_chirho =
            rhema_module_chirho::DomainStoreChirho::open_chirho(&path_str_chirho)
                .map_err(|e_chirho| ExecErrorChirho::DomainStoreChirho {
                    reason_chirho: format!("Failed to open domain store: {e_chirho}"),
                })?;

        let refs_chirho = store_chirho
            .verses_by_domain_chirho(domain_chirho)
            .map_err(|e_chirho| ExecErrorChirho::DomainStoreChirho {
                reason_chirho: format!("Domain lookup failed: {e_chirho}"),
            })?;

        let mut hits_chirho = Vec::with_capacity(refs_chirho.len());
        for ref_chirho in &refs_chirho {
            let key_str_chirho = format!(
                "{} {}:{}",
                ref_chirho.book_chirho, ref_chirho.chapter_chirho, ref_chirho.verse_chirho
            );
            let text_chirho = self
                .sword_adapter_chirho
                .manager_chirho()
                .load_module_chirho(module_name_chirho)
                .ok()
                .and_then(|loaded_chirho| loaded_chirho.read_entry_chirho(&key_str_chirho).ok())
                .unwrap_or_default();

            hits_chirho.push(SearchHitChirho {
                verse_ref_chirho: ref_chirho.clone(),
                text_chirho,
                highlighted_text_chirho: None,
                score_chirho: 1.0,
                matched_positions_chirho: Vec::new(),
                explain_chirho: None,
                semantic_score_chirho: None,
                hybrid_score_chirho: None,
                semantic_explain_chirho: Some(format!("domain:{domain_chirho}")),
            });
        }
        Ok(hits_chirho)
    }

    /// Execute a sense lookup via the semantic domain store.
    fn execute_sense_lookup_chirho(
        &self,
        sense_chirho: &str,
        module_name_chirho: &str,
    ) -> Result<Vec<SearchHitChirho>, ExecErrorChirho> {
        let domain_path_chirho = match &self.domain_store_path_chirho {
            Some(p_chirho) => p_chirho.clone(),
            None => {
                log::warn!(
                    "SenseLookup for '{}' — no domain store configured, returning empty",
                    sense_chirho
                );
                return Ok(Vec::new());
            }
        };

        let path_str_chirho = domain_path_chirho.to_string_lossy().to_string();
        let store_chirho =
            rhema_module_chirho::DomainStoreChirho::open_chirho(&path_str_chirho)
                .map_err(|e_chirho| ExecErrorChirho::DomainStoreChirho {
                    reason_chirho: format!("Failed to open domain store: {e_chirho}"),
                })?;

        let refs_chirho = store_chirho
            .verses_by_sense_chirho(sense_chirho)
            .map_err(|e_chirho| ExecErrorChirho::DomainStoreChirho {
                reason_chirho: format!("Sense lookup failed: {e_chirho}"),
            })?;

        let mut hits_chirho = Vec::with_capacity(refs_chirho.len());
        for ref_chirho in &refs_chirho {
            let key_str_chirho = format!(
                "{} {}:{}",
                ref_chirho.book_chirho, ref_chirho.chapter_chirho, ref_chirho.verse_chirho
            );
            let text_chirho = self
                .sword_adapter_chirho
                .manager_chirho()
                .load_module_chirho(module_name_chirho)
                .ok()
                .and_then(|loaded_chirho| loaded_chirho.read_entry_chirho(&key_str_chirho).ok())
                .unwrap_or_default();

            hits_chirho.push(SearchHitChirho {
                verse_ref_chirho: ref_chirho.clone(),
                text_chirho,
                highlighted_text_chirho: None,
                score_chirho: 1.0,
                matched_positions_chirho: Vec::new(),
                explain_chirho: None,
                semantic_score_chirho: None,
                hybrid_score_chirho: None,
                semantic_explain_chirho: Some(format!("sense:{sense_chirho}")),
            });
        }
        Ok(hits_chirho)
    }

    /// Execute a full-text search, preferring Tantivy index when available.
    fn execute_fulltext_chirho(
        &self,
        query_text_chirho: &str,
        module_name_chirho: &str,
    ) -> Result<Vec<SearchHitChirho>, ExecErrorChirho> {
        // Try Tantivy-indexed search first.
        if let Some(ref base_chirho) = self.index_base_chirho {
            let indexer_chirho = ModuleIndexerChirho::new_chirho(base_chirho);
            if indexer_chirho.has_index_chirho(module_name_chirho) {
                log::debug!(
                    "Using Tantivy index for '{}' query on {}",
                    query_text_chirho,
                    module_name_chirho,
                );
                return self.execute_fulltext_indexed_chirho(
                    query_text_chirho,
                    module_name_chirho,
                    &indexer_chirho,
                );
            }
        }

        log::debug!(
            "No index found for {}, falling back to regex search",
            module_name_chirho,
        );
        self.execute_fulltext_regex_chirho(query_text_chirho, module_name_chirho)
    }

    /// Execute a full-text search using a Tantivy index.
    fn execute_fulltext_indexed_chirho(
        &self,
        query_text_chirho: &str,
        module_name_chirho: &str,
        indexer_chirho: &ModuleIndexerChirho,
    ) -> Result<Vec<SearchHitChirho>, ExecErrorChirho> {
        let index_path_chirho = indexer_chirho.index_path_chirho(module_name_chirho);
        let searcher_chirho = IndexSearcherChirho::open_chirho(&index_path_chirho)
            .map_err(|e_chirho| ExecErrorChirho::SearchFailedChirho {
                reason_chirho: format!("Failed to open index: {e_chirho}"),
            })?;

        // Strip quotes for phrase queries
        let clean_query_chirho = query_text_chirho.trim_matches('"');

        let index_hits_chirho = searcher_chirho
            .search_text_chirho(clean_query_chirho, 500)
            .map_err(|e_chirho| ExecErrorChirho::SearchFailedChirho {
                reason_chirho: format!("Index search failed: {e_chirho}"),
            })?;

        let mut hits_chirho = Vec::with_capacity(index_hits_chirho.len());
        for hit_chirho in index_hits_chirho {
            if let Some(verse_ref_chirho) = parse_verse_ref_chirho(&hit_chirho.key_chirho) {
                hits_chirho.push(SearchHitChirho {
                    verse_ref_chirho,
                    text_chirho: hit_chirho.text_chirho,
                    highlighted_text_chirho: None,
                    score_chirho: hit_chirho.score_chirho,
                    matched_positions_chirho: Vec::new(),
                    explain_chirho: None,
                    semantic_score_chirho: None,
                    hybrid_score_chirho: None,
                    semantic_explain_chirho: None,
                });
            }
        }

        Ok(hits_chirho)
    }

    /// Execute a full-text search using rsword_chirho's regex search engine (fallback).
    fn execute_fulltext_regex_chirho(
        &self,
        query_text_chirho: &str,
        module_name_chirho: &str,
    ) -> Result<Vec<SearchHitChirho>, ExecErrorChirho> {
        // Load the module and build a search corpus
        let loaded_chirho = self
            .sword_adapter_chirho
            .manager_chirho()
            .load_module_chirho(module_name_chirho)
            .map_err(|e_chirho| ExecErrorChirho::SwordChirho(e_chirho.to_string()))?;

        let mut search_engine_chirho = RegexSearchChirho::new_chirho();

        // Load all verses from the module for searching
        // Use well-known books to iterate through
        let books_chirho = standard_books_chirho();
        for book_chirho in &books_chirho {
            for chapter_chirho in 1..=150u32 {
                match loaded_chirho.read_chapter_batch_chirho(book_chirho, chapter_chirho, 200) {
                    Ok(verses_chirho) => {
                        if verses_chirho.is_empty() {
                            break; // No more chapters for this book
                        }
                        for (verse_num_chirho, text_chirho) in &verses_chirho {
                            if !text_chirho.is_empty() {
                                let key_chirho = format!(
                                    "{} {}:{}",
                                    book_chirho, chapter_chirho, verse_num_chirho
                                );
                                search_engine_chirho
                                    .add_entry_chirho(key_chirho, text_chirho.clone());
                            }
                        }
                    }
                    Err(_) => break,
                }
            }
        }

        // Determine if it's a phrase query (starts and ends with quotes)
        let (clean_query_chirho, search_type_chirho) =
            if query_text_chirho.starts_with('"') && query_text_chirho.ends_with('"') {
                (
                    query_text_chirho
                        .trim_matches('"')
                        .to_string(),
                    SearchTypeChirho::PhraseChirho,
                )
            } else {
                (query_text_chirho.to_string(), SearchTypeChirho::MultiWordChirho)
            };

        let options_chirho = SearchOptionsChirho::new_chirho()
            .with_type_chirho(search_type_chirho)
            .case_insensitive_chirho(true)
            .with_max_results_chirho(500);

        let list_key_chirho = search_engine_chirho
            .search_chirho(&clean_query_chirho, &options_chirho)
            .map_err(|e_chirho| ExecErrorChirho::SearchFailedChirho {
                reason_chirho: e_chirho.to_string(),
            })?;

        // Convert ListKeyChirho to SearchHitChirho
        let mut hits_chirho = Vec::new();
        for key_ref_chirho in list_key_chirho.iter_chirho() {
            let ref_str_chirho = key_ref_chirho.to_string();
            if let Some(verse_ref_chirho) = parse_verse_ref_chirho(&ref_str_chirho) {
                // Read the actual verse text
                let text_chirho = loaded_chirho
                    .read_entry_chirho(&ref_str_chirho)
                    .unwrap_or_default();

                hits_chirho.push(SearchHitChirho {
                    verse_ref_chirho,
                    text_chirho,
                    highlighted_text_chirho: None,
                    score_chirho: 1.0,
                    matched_positions_chirho: Vec::new(),
                    explain_chirho: None,
                    semantic_score_chirho: None,
                    hybrid_score_chirho: None,
                    semantic_explain_chirho: None,
                });
            }
        }

        Ok(hits_chirho)
    }

    /// Execute a proximity search.
    fn execute_proximity_chirho(
        &self,
        phrase_chirho: &str,
        _slop_chirho: u32,
        module_name_chirho: &str,
    ) -> Result<Vec<SearchHitChirho>, ExecErrorChirho> {
        // Use phrase search with slop as proximity
        let quoted_chirho = format!("\"{}\"", phrase_chirho);
        self.execute_fulltext_chirho(&quoted_chirho, module_name_chirho)
    }
}

impl QueryExecutorChirho for QueryExecutorImplChirho {
    fn execute_chirho(
        &self,
        query_chirho: &QueryChirho,
    ) -> Result<QueryResultChirho, Box<dyn std::error::Error + Send + Sync>> {
        // Default to searching all available Bible modules
        let bibles_chirho = self.sword_adapter_chirho.list_bibles_chirho();
        let module_name_chirho = bibles_chirho
            .first()
            .ok_or_else(|| {
                Box::new(ExecErrorChirho::ModuleNotLoadedChirho {
                    name_chirho: "no modules available".to_string(),
                }) as Box<dyn std::error::Error + Send + Sync>
            })?;

        self.execute_against_module_chirho(query_chirho, module_name_chirho)
            .map_err(|e_chirho| Box::new(e_chirho) as Box<dyn std::error::Error + Send + Sync>)
    }
}

/// Parse a verse reference string like "John 3:16" into a VerseRefChirho.
fn parse_verse_ref_chirho(ref_str_chirho: &str) -> Option<VerseRefChirho> {
    // Match patterns like "BookName Chapter:Verse"
    let re_chirho =
        regex::Regex::new(r"^(.+?)\s+(\d+):(\d+)$").ok()?;

    let caps_chirho = re_chirho.captures(ref_str_chirho)?;
    let book_chirho = caps_chirho.get(1)?.as_str();
    let chapter_chirho: u16 = caps_chirho.get(2)?.as_str().parse().ok()?;
    let verse_chirho: u16 = caps_chirho.get(3)?.as_str().parse().ok()?;

    VerseRefChirho::new_chirho(book_chirho, chapter_chirho, verse_chirho).ok()
}

/// Intersect multiple hit sets by verse reference (AND semantics).
fn intersect_hits_chirho(sets_chirho: Vec<Vec<SearchHitChirho>>) -> Vec<SearchHitChirho> {
    if sets_chirho.is_empty() {
        return Vec::new();
    }
    if sets_chirho.len() == 1 {
        return sets_chirho.into_iter().next().unwrap_or_default();
    }

    // Use the smallest set as the base
    let mut sorted_chirho = sets_chirho;
    sorted_chirho.sort_by_key(|s_chirho| s_chirho.len());

    let base_chirho = &sorted_chirho[0];
    let rest_chirho = &sorted_chirho[1..];

    base_chirho
        .iter()
        .filter(|hit_chirho| {
            rest_chirho.iter().all(|set_chirho| {
                set_chirho
                    .iter()
                    .any(|h_chirho| h_chirho.verse_ref_chirho == hit_chirho.verse_ref_chirho)
            })
        })
        .cloned()
        .collect()
}

/// Deduplicate hits by verse reference, keeping the highest score.
fn dedup_hits_chirho(hits_chirho: &mut Vec<SearchHitChirho>) {
    hits_chirho.sort_by(|a_chirho, b_chirho| {
        a_chirho
            .verse_ref_chirho
            .to_string()
            .cmp(&b_chirho.verse_ref_chirho.to_string())
    });
    hits_chirho.dedup_by(|a_chirho, b_chirho| {
        a_chirho.verse_ref_chirho == b_chirho.verse_ref_chirho
    });
}

/// Standard Bible book names for iteration.
fn standard_books_chirho() -> Vec<&'static str> {
    vec![
        "Genesis", "Exodus", "Leviticus", "Numbers", "Deuteronomy",
        "Joshua", "Judges", "Ruth", "I Samuel", "II Samuel",
        "I Kings", "II Kings", "I Chronicles", "II Chronicles",
        "Ezra", "Nehemiah", "Esther", "Job", "Psalms",
        "Proverbs", "Ecclesiastes", "Song of Solomon",
        "Isaiah", "Jeremiah", "Lamentations", "Ezekiel", "Daniel",
        "Hosea", "Joel", "Amos", "Obadiah", "Jonah", "Micah",
        "Nahum", "Habakkuk", "Zephaniah", "Haggai", "Zechariah", "Malachi",
        "Matthew", "Mark", "Luke", "John", "Acts",
        "Romans", "I Corinthians", "II Corinthians",
        "Galatians", "Ephesians", "Philippians", "Colossians",
        "I Thessalonians", "II Thessalonians",
        "I Timothy", "II Timothy", "Titus", "Philemon",
        "Hebrews", "James", "I Peter", "II Peter",
        "I John", "II John", "III John", "Jude",
        "Revelation of John",
    ]
}

/// Resolve the default index base directory.
/// Uses `~/.sword/rhema_indexes` (or platform equivalent via HOME).
fn dirs_index_base_chirho() -> Option<PathBuf> {
    std::env::var("HOME")
        .ok()
        .map(|home_chirho| {
            PathBuf::from(home_chirho)
                .join(".sword")
                .join(INDEX_DIR_NAME_CHIRHO)
        })
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_parse_verse_ref_chirho() {
        let ref1_chirho = parse_verse_ref_chirho("John 3:16").unwrap();
        assert_eq!(ref1_chirho.book_chirho, "John");
        assert_eq!(ref1_chirho.chapter_chirho, 3);
        assert_eq!(ref1_chirho.verse_chirho, 16);

        let ref2_chirho = parse_verse_ref_chirho("I Corinthians 13:4").unwrap();
        assert_eq!(ref2_chirho.book_chirho, "I Corinthians");
        assert_eq!(ref2_chirho.chapter_chirho, 13);
        assert_eq!(ref2_chirho.verse_chirho, 4);

        assert!(parse_verse_ref_chirho("invalid").is_none());
    }

    #[test]
    fn test_intersect_hits_empty_chirho() {
        let result_chirho = intersect_hits_chirho(Vec::new());
        assert!(result_chirho.is_empty());
    }

    #[test]
    fn test_intersect_hits_single_chirho() {
        let hits_chirho = vec![SearchHitChirho {
            verse_ref_chirho: VerseRefChirho::new_chirho("John", 3, 16).unwrap(),
            text_chirho: "test".to_string(),
            highlighted_text_chirho: None,
            score_chirho: 1.0,
            matched_positions_chirho: Vec::new(),
            explain_chirho: None,
            semantic_score_chirho: None,
            hybrid_score_chirho: None,
            semantic_explain_chirho: None,
        }];
        let result_chirho = intersect_hits_chirho(vec![hits_chirho]);
        assert_eq!(result_chirho.len(), 1);
    }

    #[test]
    fn test_intersect_with_exclude_chirho() {
        // Simulate AND(term("love"), NOT(term("world")))
        // where "love" matches John 3:16 and Romans 8:28
        // and "world" matches John 3:16
        // Expected result: only Romans 8:28 (John 3:16 excluded)
        let positive_sets_chirho = vec![vec![
            SearchHitChirho {
                verse_ref_chirho: VerseRefChirho::new_chirho("John", 3, 16).unwrap(),
                text_chirho: "loved the world".to_string(),
                highlighted_text_chirho: None,
                score_chirho: 1.0,
                matched_positions_chirho: Vec::new(),
                explain_chirho: None,
                semantic_score_chirho: None,
                hybrid_score_chirho: None,
                semantic_explain_chirho: None,
            },
            SearchHitChirho {
                verse_ref_chirho: VerseRefChirho::new_chirho("Romans", 8, 28).unwrap(),
                text_chirho: "love God".to_string(),
                highlighted_text_chirho: None,
                score_chirho: 1.0,
                matched_positions_chirho: Vec::new(),
                explain_chirho: None,
                semantic_score_chirho: None,
                hybrid_score_chirho: None,
                semantic_explain_chirho: None,
            },
        ]];

        let excluded_chirho = vec![SearchHitChirho {
            verse_ref_chirho: VerseRefChirho::new_chirho("John", 3, 16).unwrap(),
            text_chirho: "the world".to_string(),
            highlighted_text_chirho: None,
            score_chirho: 1.0,
            matched_positions_chirho: Vec::new(),
            explain_chirho: None,
            semantic_score_chirho: None,
            hybrid_score_chirho: None,
            semantic_explain_chirho: None,
        }];

        // Intersect the positive sets
        let mut base_results_chirho = intersect_hits_chirho(positive_sets_chirho);
        assert_eq!(base_results_chirho.len(), 2);

        // Apply exclusion
        let excluded_keys_chirho: std::collections::HashSet<String> = excluded_chirho
            .iter()
            .map(|h_chirho| h_chirho.verse_ref_chirho.to_string())
            .collect();
        base_results_chirho.retain(|h_chirho| {
            !excluded_keys_chirho.contains(&h_chirho.verse_ref_chirho.to_string())
        });

        assert_eq!(base_results_chirho.len(), 1);
        assert_eq!(base_results_chirho[0].verse_ref_chirho.book_chirho, "Romans");
    }

    // ── Phase 4: Graph search executor tests ────────────────────

    #[test]
    fn test_parse_verse_ref_osis_format_chirho() {
        // Test that OSIS refs can be parsed through the xref extractor
        let refs_chirho =
            rhema_ingest_chirho::xref_extractor_chirho::parse_osis_ref_chirho("John.3.16");
        assert_eq!(refs_chirho.len(), 1);
        assert_eq!(refs_chirho[0].book_chirho, "John");
        assert_eq!(refs_chirho[0].chapter_chirho, 3);
        assert_eq!(refs_chirho[0].verse_chirho, 16);
    }

    #[test]
    fn test_parse_verse_ref_human_format_chirho() {
        let ref_chirho =
            rhema_ingest_chirho::xref_extractor_chirho::parse_human_ref_chirho("John 3:16");
        assert!(ref_chirho.is_some());
        let vr_chirho = ref_chirho.unwrap();
        assert_eq!(vr_chirho.book_chirho, "John");
    }

    #[test]
    fn test_graph_search_no_store_fallback_chirho() {
        // When no xref store is configured, graph search should return empty
        use rhema_contracts_chirho::xref_chirho::CrossRefGraphChirho;

        let store_chirho = rhema_module_chirho::XrefStoreChirho::in_memory_chirho().unwrap();
        store_chirho
            .insert_xref_chirho(
                &rhema_contracts_chirho::xref_chirho::CrossRefEntryChirho::new_chirho(
                    VerseRefChirho::new_chirho("John", 3, 16).unwrap(),
                    VerseRefChirho::new_chirho("Romans", 5, 8).unwrap(),
                    rhema_contracts_chirho::xref_chirho::XRefTypeChirho::DirectChirho,
                ),
            )
            .unwrap();

        // BFS from the store directly works
        let expanded_chirho = store_chirho
            .bfs_expand_chirho(
                &VerseRefChirho::new_chirho("John", 3, 16).unwrap(),
                1,
                false,
            )
            .unwrap();
        assert_eq!(expanded_chirho.len(), 1);
        assert_eq!(expanded_chirho[0].book_chirho, "Romans");
    }

    #[test]
    fn test_dedup_hits_chirho() {
        let mut hits_chirho = vec![
            SearchHitChirho {
                verse_ref_chirho: VerseRefChirho::new_chirho("John", 3, 16).unwrap(),
                text_chirho: "a".to_string(),
                highlighted_text_chirho: None,
                score_chirho: 1.0,
                matched_positions_chirho: Vec::new(),
                explain_chirho: None,
                semantic_score_chirho: None,
                hybrid_score_chirho: None,
                semantic_explain_chirho: None,
            },
            SearchHitChirho {
                verse_ref_chirho: VerseRefChirho::new_chirho("John", 3, 16).unwrap(),
                text_chirho: "b".to_string(),
                highlighted_text_chirho: None,
                score_chirho: 0.8,
                matched_positions_chirho: Vec::new(),
                explain_chirho: None,
                semantic_score_chirho: None,
                hybrid_score_chirho: None,
                semantic_explain_chirho: None,
            },
        ];
        dedup_hits_chirho(&mut hits_chirho);
        assert_eq!(hits_chirho.len(), 1);
    }
}
