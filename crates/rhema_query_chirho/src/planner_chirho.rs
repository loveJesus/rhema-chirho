// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Query planner — transforms a [`QueryNodeChirho`] IR into an execution plan.
//!
//! The planner analyzes the query IR, selects the best backend(s), and
//! produces an execution plan that the executor can run.

use rhema_contracts_chirho::morphology_chirho::MorphConstraintChirho;
use rhema_contracts_chirho::query_chirho::{QueryChirho, QueryNodeChirho};

/// An execution plan step.
#[derive(Debug, Clone)]
pub enum PlanStepChirho {
    /// Full-text search on the given backend.
    FullTextSearchChirho {
        backend_chirho: String,
        query_text_chirho: String,
    },
    /// Index lookup for Strong's number.
    StrongLookupChirho { number_chirho: String },
    /// Index lookup for lemma.
    LemmaLookupChirho { lemma_chirho: String },
    /// Intersection of sub-plan results (AND).
    IntersectChirho(Vec<PlanStepChirho>),
    /// Union of sub-plan results (OR).
    UnionChirho(Vec<PlanStepChirho>),
    /// Negation filter.
    ExcludeChirho(Box<PlanStepChirho>),
    /// Proximity filter on previous results.
    ProximityFilterChirho {
        terms_chirho: Vec<String>,
        distance_chirho: u32,
    },

    // ── Phase 2: AI / Semantic plan steps ───────────────────────────

    /// Embedding-based vector similarity search.
    EmbeddingSearchChirho {
        query_text_chirho: String,
        top_k_chirho: usize,
        similarity_threshold_chirho: f32,
    },

    /// LLM-powered query expansion step.
    LlmExpansionChirho {
        original_query_chirho: String,
        expansion_type_chirho: String,
    },

    /// Hybrid merge of keyword + semantic results.
    HybridRankChirho {
        keyword_step_chirho: Box<PlanStepChirho>,
        semantic_step_chirho: Box<PlanStepChirho>,
        keyword_weight_chirho: f32,
    },

    /// Concept-based search via embedding.
    ConceptSearchChirho {
        concept_chirho: String,
    },

    // ── Phase 2: Discourse plan steps ───────────────────────────────

    /// Search discourse store by relationship type.
    DiscourseRelationshipSearchChirho {
        relationship_type_chirho: String,
    },

    /// Search discourse store by proposition text.
    PropositionTextSearchChirho {
        text_chirho: String,
    },

    /// Phase 3: Morphology lookup against Tantivy facet fields.
    MorphLookupChirho {
        constraint_chirho: MorphConstraintChirho,
    },

    /// Phase 4: Cross-reference graph expansion via BFS.
    GraphSearchChirho {
        seed_chirho: String,
        depth_chirho: u32,
    },

    /// Phase 5+: Semantic domain lookup.
    DomainLookupChirho {
        domain_chirho: String,
    },

    /// Phase 5+: Word sense lookup.
    SenseLookupChirho {
        sense_chirho: String,
    },

    /// Phase 5+: Syntax/clause search.
    SyntaxSearchChirho {
        clause_type_chirho: String,
    },
}

/// The complete execution plan.
#[derive(Debug, Clone)]
pub struct QueryPlanChirho {
    pub root_step_chirho: PlanStepChirho,
    pub estimated_cost_chirho: f64,
    pub backend_chirho: String,
}

/// The query planner.
pub struct QueryPlannerChirho;

impl QueryPlannerChirho {
    /// Plan a query for execution.
    pub fn plan_chirho(query_chirho: &QueryChirho) -> QueryPlanChirho {
        let root_step_chirho = plan_node_chirho(&query_chirho.root_chirho);
        let estimated_cost_chirho = estimate_cost_chirho(&root_step_chirho);

        QueryPlanChirho {
            root_step_chirho,
            estimated_cost_chirho,
            backend_chirho: "cpu".to_string(),
        }
    }

    /// Generate a human-readable explanation of the plan.
    pub fn explain_chirho(plan_chirho: &QueryPlanChirho) -> String {
        format!(
            "Backend: {}, Estimated cost: {:.2}\nPlan: {:?}",
            plan_chirho.backend_chirho, plan_chirho.estimated_cost_chirho, plan_chirho.root_step_chirho
        )
    }
}

/// Convert a QueryNodeChirho into a PlanStepChirho.
fn plan_node_chirho(node_chirho: &QueryNodeChirho) -> PlanStepChirho {
    match node_chirho {
        QueryNodeChirho::TermChirho { text_chirho, .. } => PlanStepChirho::FullTextSearchChirho {
            backend_chirho: "tantivy".to_string(),
            query_text_chirho: text_chirho.clone(),
        },

        QueryNodeChirho::PhraseChirho {
            words_chirho, ..
        } => PlanStepChirho::FullTextSearchChirho {
            backend_chirho: "tantivy".to_string(),
            query_text_chirho: format!("\"{}\"", words_chirho.join(" ")),
        },

        QueryNodeChirho::AndChirho(children_chirho) => {
            let steps_chirho: Vec<PlanStepChirho> =
                children_chirho.iter().map(plan_node_chirho).collect();
            PlanStepChirho::IntersectChirho(steps_chirho)
        }

        QueryNodeChirho::OrChirho(children_chirho) => {
            let steps_chirho: Vec<PlanStepChirho> =
                children_chirho.iter().map(plan_node_chirho).collect();
            PlanStepChirho::UnionChirho(steps_chirho)
        }

        QueryNodeChirho::NotChirho(inner_chirho) => {
            PlanStepChirho::ExcludeChirho(Box::new(plan_node_chirho(inner_chirho)))
        }

        QueryNodeChirho::ProximityChirho {
            terms_chirho,
            distance_chirho,
        } => PlanStepChirho::ProximityFilterChirho {
            terms_chirho: terms_chirho.clone(),
            distance_chirho: *distance_chirho,
        },

        QueryNodeChirho::StrongChirho { number_chirho } => PlanStepChirho::StrongLookupChirho {
            number_chirho: number_chirho.clone(),
        },

        QueryNodeChirho::LemmaChirho { lemma_chirho } => PlanStepChirho::LemmaLookupChirho {
            lemma_chirho: lemma_chirho.clone(),
        },

        QueryNodeChirho::MorphChirho(constraint_chirho) => {
            PlanStepChirho::MorphLookupChirho {
                constraint_chirho: constraint_chirho.clone(),
            }
        }

        QueryNodeChirho::ScopedChirho { inner_chirho, .. } => {
            // Plan the inner query; scope filtering happens at execution time
            plan_node_chirho(inner_chirho)
        }

        QueryNodeChirho::GraphExpandChirho {
            seed_chirho,
            depth_chirho,
        } => PlanStepChirho::GraphSearchChirho {
            seed_chirho: seed_chirho.clone(),
            depth_chirho: *depth_chirho,
        },

        // ── Phase 2: AI / Semantic nodes ────────────────────────────

        QueryNodeChirho::SemanticChirho {
            query_text_chirho,
            top_k_chirho,
            similarity_threshold_chirho,
        } => PlanStepChirho::EmbeddingSearchChirho {
            query_text_chirho: query_text_chirho.clone(),
            top_k_chirho: *top_k_chirho,
            similarity_threshold_chirho: *similarity_threshold_chirho,
        },

        QueryNodeChirho::ExpandedChirho {
            original_query_chirho,
            expansion_strategy_chirho,
            ..
        } => PlanStepChirho::LlmExpansionChirho {
            original_query_chirho: original_query_chirho.clone(),
            expansion_type_chirho: format!("{expansion_strategy_chirho:?}"),
        },

        QueryNodeChirho::HybridChirho {
            keyword_query_chirho,
            semantic_query_chirho,
            keyword_weight_chirho,
        } => PlanStepChirho::HybridRankChirho {
            keyword_step_chirho: Box::new(plan_node_chirho(keyword_query_chirho)),
            semantic_step_chirho: Box::new(plan_node_chirho(semantic_query_chirho)),
            keyword_weight_chirho: *keyword_weight_chirho,
        },

        QueryNodeChirho::ConceptChirho {
            concept_chirho, ..
        } => PlanStepChirho::ConceptSearchChirho {
            concept_chirho: concept_chirho.clone(),
        },

        // ── Phase 2: Discourse nodes ────────────────────────────────

        QueryNodeChirho::DiscourseRelationshipChirho {
            relationship_type_chirho,
            ..
        } => PlanStepChirho::DiscourseRelationshipSearchChirho {
            relationship_type_chirho: relationship_type_chirho.clone(),
        },

        QueryNodeChirho::PropositionTextChirho {
            text_chirho, ..
        } => PlanStepChirho::PropositionTextSearchChirho {
            text_chirho: text_chirho.clone(),
        },

        // ── Phase 5+: Domain, Sense, Syntax nodes ──────────────────

        QueryNodeChirho::DomainChirho { domain_chirho } => PlanStepChirho::DomainLookupChirho {
            domain_chirho: domain_chirho.clone(),
        },

        QueryNodeChirho::SenseChirho { sense_chirho } => PlanStepChirho::SenseLookupChirho {
            sense_chirho: sense_chirho.clone(),
        },

        QueryNodeChirho::SyntaxChirho {
            clause_type_chirho, ..
        } => PlanStepChirho::SyntaxSearchChirho {
            clause_type_chirho: clause_type_chirho.clone(),
        },
    }
}

/// Estimate the cost of executing a plan step (simple heuristic).
fn estimate_cost_chirho(step_chirho: &PlanStepChirho) -> f64 {
    match step_chirho {
        PlanStepChirho::FullTextSearchChirho { .. } => 1.0,
        PlanStepChirho::StrongLookupChirho { .. } => 0.5,
        PlanStepChirho::LemmaLookupChirho { .. } => 0.5,
        PlanStepChirho::IntersectChirho(children_chirho) => {
            children_chirho.iter().map(estimate_cost_chirho).sum::<f64>() * 0.8
        }
        PlanStepChirho::UnionChirho(children_chirho) => {
            children_chirho.iter().map(estimate_cost_chirho).sum::<f64>() * 1.2
        }
        PlanStepChirho::ExcludeChirho(inner_chirho) => estimate_cost_chirho(inner_chirho) + 0.3,
        PlanStepChirho::ProximityFilterChirho { .. } => 1.5,
        PlanStepChirho::EmbeddingSearchChirho { .. } => 3.0,
        PlanStepChirho::LlmExpansionChirho { .. } => 5.0,
        PlanStepChirho::HybridRankChirho {
            keyword_step_chirho,
            semantic_step_chirho,
            ..
        } => estimate_cost_chirho(keyword_step_chirho) + estimate_cost_chirho(semantic_step_chirho),
        PlanStepChirho::ConceptSearchChirho { .. } => 4.0,
        PlanStepChirho::DiscourseRelationshipSearchChirho { .. } => 0.5,
        PlanStepChirho::PropositionTextSearchChirho { .. } => 0.5,
        PlanStepChirho::MorphLookupChirho { .. } => 0.7,
        PlanStepChirho::GraphSearchChirho { depth_chirho, .. } => 1.0 + *depth_chirho as f64 * 0.5,
        PlanStepChirho::DomainLookupChirho { .. } => 0.6,
        PlanStepChirho::SenseLookupChirho { .. } => 0.6,
        PlanStepChirho::SyntaxSearchChirho { .. } => 0.8,
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use crate::parser_chirho::QueryParserChirho;

    #[test]
    fn test_plan_simple_term_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("love").unwrap();
        let plan_chirho = QueryPlannerChirho::plan_chirho(&query_chirho);

        match &plan_chirho.root_step_chirho {
            PlanStepChirho::FullTextSearchChirho {
                query_text_chirho, ..
            } => {
                assert_eq!(query_text_chirho, "love");
            }
            other_chirho => panic!("Expected FullTextSearchChirho, got {:?}", other_chirho),
        }
    }

    #[test]
    fn test_plan_boolean_and_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("love AND world").unwrap();
        let plan_chirho = QueryPlannerChirho::plan_chirho(&query_chirho);

        match &plan_chirho.root_step_chirho {
            PlanStepChirho::IntersectChirho(steps_chirho) => {
                assert_eq!(steps_chirho.len(), 2);
            }
            other_chirho => panic!("Expected IntersectChirho, got {:?}", other_chirho),
        }
    }

    #[test]
    fn test_plan_strong_lookup_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("strong:G26").unwrap();
        let plan_chirho = QueryPlannerChirho::plan_chirho(&query_chirho);

        match &plan_chirho.root_step_chirho {
            PlanStepChirho::StrongLookupChirho { number_chirho } => {
                assert_eq!(number_chirho, "G26");
            }
            other_chirho => panic!("Expected StrongLookupChirho, got {:?}", other_chirho),
        }
    }

    #[test]
    fn test_explain_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("love AND world").unwrap();
        let plan_chirho = QueryPlannerChirho::plan_chirho(&query_chirho);
        let explain_chirho = QueryPlannerChirho::explain_chirho(&plan_chirho);

        assert!(explain_chirho.contains("Backend: cpu"));
        assert!(explain_chirho.contains("Estimated cost"));
    }

    #[test]
    fn test_plan_morph_lookup_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("morph:V-AAI-3S").unwrap();
        let plan_chirho = QueryPlannerChirho::plan_chirho(&query_chirho);

        match &plan_chirho.root_step_chirho {
            PlanStepChirho::MorphLookupChirho { constraint_chirho } => {
                assert!(constraint_chirho.part_of_speech_chirho.is_some());
            }
            other_chirho => panic!("Expected MorphLookupChirho, got {:?}", other_chirho),
        }
    }

    #[test]
    fn test_plan_morph_cost_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("pos:verb").unwrap();
        let plan_chirho = QueryPlannerChirho::plan_chirho(&query_chirho);
        assert!((plan_chirho.estimated_cost_chirho - 0.7).abs() < 0.01);
    }

    // ── Phase 4: Graph search plan tests ────────────────────────

    #[test]
    fn test_plan_graph_search_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("xref:John.3.16").unwrap();
        let plan_chirho = QueryPlannerChirho::plan_chirho(&query_chirho);

        match &plan_chirho.root_step_chirho {
            PlanStepChirho::GraphSearchChirho {
                seed_chirho,
                depth_chirho,
            } => {
                assert_eq!(seed_chirho, "John.3.16");
                assert_eq!(*depth_chirho, 1);
            }
            other_chirho => panic!("Expected GraphSearchChirho, got {:?}", other_chirho),
        }
    }

    #[test]
    fn test_plan_domain_lookup_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("domain:love").unwrap();
        let plan_chirho = QueryPlannerChirho::plan_chirho(&query_chirho);
        match &plan_chirho.root_step_chirho {
            PlanStepChirho::DomainLookupChirho { domain_chirho } => {
                assert_eq!(domain_chirho, "love");
            }
            other_chirho => panic!("Expected DomainLookupChirho, got {:?}", other_chirho),
        }
        assert!((plan_chirho.estimated_cost_chirho - 0.6).abs() < 0.01);
    }

    #[test]
    fn test_plan_sense_lookup_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("sense:love.01").unwrap();
        let plan_chirho = QueryPlannerChirho::plan_chirho(&query_chirho);
        match &plan_chirho.root_step_chirho {
            PlanStepChirho::SenseLookupChirho { sense_chirho } => {
                assert_eq!(sense_chirho, "love.01");
            }
            other_chirho => panic!("Expected SenseLookupChirho, got {:?}", other_chirho),
        }
    }

    #[test]
    fn test_plan_syntax_search_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("syntax:relative").unwrap();
        let plan_chirho = QueryPlannerChirho::plan_chirho(&query_chirho);
        match &plan_chirho.root_step_chirho {
            PlanStepChirho::SyntaxSearchChirho { clause_type_chirho } => {
                assert_eq!(clause_type_chirho, "relative");
            }
            other_chirho => panic!("Expected SyntaxSearchChirho, got {:?}", other_chirho),
        }
        assert!((plan_chirho.estimated_cost_chirho - 0.8).abs() < 0.01);
    }

    #[test]
    fn test_plan_graph_search_cost_chirho() {
        // Depth 1: cost = 1.0 + 1*0.5 = 1.5
        let query_chirho = QueryParserChirho::parse_chirho("xref:John.3.16").unwrap();
        let plan_chirho = QueryPlannerChirho::plan_chirho(&query_chirho);
        assert!((plan_chirho.estimated_cost_chirho - 1.5).abs() < 0.01);

        // Depth 3: cost = 1.0 + 3*0.5 = 2.5
        let query2_chirho = QueryParserChirho::parse_chirho("XREF/3 John 3:16").unwrap();
        let plan2_chirho = QueryPlannerChirho::plan_chirho(&query2_chirho);
        assert!((plan2_chirho.estimated_cost_chirho - 2.5).abs() < 0.01);
    }
}
