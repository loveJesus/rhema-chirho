// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Query intermediate representation (IR) — parser-neutral, backend-agnostic.
//! This is the canonical form that the planner optimizes and backends execute.

use serde::{Deserialize, Serialize};

use crate::keys_chirho::ScopeChirho;
use crate::morphology_chirho::MorphConstraintChirho;

/// Strategy for LLM-based query expansion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExpansionStrategyChirho {
    /// Expand with synonyms only.
    SynonymChirho,
    /// Expand with Strong's number mappings.
    StrongsChirho,
    /// Expand with conceptual/thematic terms.
    ConceptualChirho,
    /// Full expansion: synonyms + Strong's + concepts.
    FullChirho,
}

/// The query IR. Every user query is parsed into this form before planning.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueryNodeChirho {
    /// Match a text term (word or phrase fragment).
    TermChirho {
        text_chirho: String,
        field_chirho: Option<String>,
    },

    /// Match an exact phrase.
    PhraseChirho {
        words_chirho: Vec<String>,
        slop_chirho: u32,
    },

    /// Boolean AND of sub-queries.
    AndChirho(Vec<QueryNodeChirho>),

    /// Boolean OR of sub-queries.
    OrChirho(Vec<QueryNodeChirho>),

    /// Boolean NOT (exclude matches).
    NotChirho(Box<QueryNodeChirho>),

    /// Proximity: words within N positions of each other.
    ProximityChirho {
        terms_chirho: Vec<String>,
        distance_chirho: u32,
    },

    /// Morphology constraint (e.g., pos=verb, tense=aorist).
    MorphChirho(MorphConstraintChirho),

    /// Lemma match (dictionary form).
    LemmaChirho { lemma_chirho: String },

    /// Strong's number match.
    StrongChirho { number_chirho: String },

    /// Scope restriction (limits where other constraints apply).
    ScopedChirho {
        scope_chirho: ScopeChirho,
        inner_chirho: Box<QueryNodeChirho>,
    },

    /// Cross-reference graph expansion.
    GraphExpandChirho {
        seed_chirho: String,
        depth_chirho: u32,
    },

    // ── Phase 2: AI / Semantic search nodes ─────────────────────────

    /// Semantic vector search — find verses similar in meaning.
    SemanticChirho {
        query_text_chirho: String,
        top_k_chirho: usize,
        similarity_threshold_chirho: f32,
    },

    /// LLM-expanded query — original text plus expanded terms/Strong's.
    ExpandedChirho {
        original_query_chirho: String,
        expanded_terms_chirho: Vec<String>,
        expanded_strongs_chirho: Vec<String>,
        expansion_strategy_chirho: ExpansionStrategyChirho,
    },

    /// Hybrid keyword + semantic search with weighted merge.
    HybridChirho {
        keyword_query_chirho: Box<QueryNodeChirho>,
        semantic_query_chirho: Box<QueryNodeChirho>,
        keyword_weight_chirho: f32,
    },

    /// Concept / thematic search (e.g. "redemption", "covenant faithfulness").
    ConceptChirho {
        concept_chirho: String,
        include_related_chirho: bool,
    },

    // ── Phase 2: Discourse analysis search nodes ────────────────────

    /// Find arcs containing a specific relationship type.
    DiscourseRelationshipChirho {
        relationship_type_chirho: String,
        scope_chirho: Option<ScopeChirho>,
    },

    /// Find propositions containing specific text.
    PropositionTextChirho {
        text_chirho: String,
        scope_chirho: Option<ScopeChirho>,
    },

    // ── Phase 5+: Domain, Sense, Syntax nodes ───────────────────

    /// Semantic domain search — find verses tagged with a semantic domain.
    DomainChirho {
        domain_chirho: String,
    },

    /// Sense/word-sense disambiguation search.
    SenseChirho {
        sense_chirho: String,
    },

    /// Syntax/clause search — find clauses by type and optional scope.
    SyntaxChirho {
        clause_type_chirho: String,
        scope_chirho: Option<ScopeChirho>,
    },
}

/// A complete query with IR and metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryChirho {
    pub root_chirho: QueryNodeChirho,
    pub scope_chirho: Option<ScopeChirho>,
    pub max_results_chirho: usize,
    pub explain_chirho: bool,
}

impl QueryChirho {
    pub fn new_chirho(root_chirho: QueryNodeChirho) -> Self {
        Self {
            root_chirho,
            scope_chirho: None,
            max_results_chirho: 100,
            explain_chirho: false,
        }
    }

    pub fn with_scope_chirho(mut self, scope_chirho: ScopeChirho) -> Self {
        self.scope_chirho = Some(scope_chirho);
        self
    }

    pub fn with_max_results_chirho(mut self, max_chirho: usize) -> Self {
        self.max_results_chirho = max_chirho;
        self
    }

    pub fn with_explain_chirho(mut self) -> Self {
        self.explain_chirho = true;
        self
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_query_new_defaults_chirho() {
        let q_chirho = QueryChirho::new_chirho(QueryNodeChirho::TermChirho {
            text_chirho: "love".to_string(),
            field_chirho: None,
        });
        assert_eq!(q_chirho.max_results_chirho, 100);
        assert!(!q_chirho.explain_chirho);
        assert!(q_chirho.scope_chirho.is_none());
    }

    #[test]
    fn test_query_with_scope_chirho() {
        let q_chirho = QueryChirho::new_chirho(QueryNodeChirho::TermChirho {
            text_chirho: "grace".to_string(),
            field_chirho: None,
        })
        .with_scope_chirho(ScopeChirho::BookChirho("Romans".to_string()));

        assert!(q_chirho.scope_chirho.is_some());
        assert_eq!(
            q_chirho.scope_chirho.unwrap(),
            ScopeChirho::BookChirho("Romans".to_string())
        );
    }

    #[test]
    fn test_query_with_max_results_chirho() {
        let q_chirho = QueryChirho::new_chirho(QueryNodeChirho::TermChirho {
            text_chirho: "faith".to_string(),
            field_chirho: None,
        })
        .with_max_results_chirho(25);

        assert_eq!(q_chirho.max_results_chirho, 25);
    }

    #[test]
    fn test_query_with_explain_chirho() {
        let q_chirho = QueryChirho::new_chirho(QueryNodeChirho::TermChirho {
            text_chirho: "hope".to_string(),
            field_chirho: None,
        })
        .with_explain_chirho();

        assert!(q_chirho.explain_chirho);
    }

    #[test]
    fn test_query_builder_chaining_chirho() {
        let q_chirho = QueryChirho::new_chirho(QueryNodeChirho::TermChirho {
            text_chirho: "peace".to_string(),
            field_chirho: None,
        })
        .with_max_results_chirho(10)
        .with_explain_chirho()
        .with_scope_chirho(ScopeChirho::EntireBibleChirho);

        assert_eq!(q_chirho.max_results_chirho, 10);
        assert!(q_chirho.explain_chirho);
        assert_eq!(q_chirho.scope_chirho, Some(ScopeChirho::EntireBibleChirho));
    }

    #[test]
    fn test_query_node_term_chirho() {
        let node_chirho = QueryNodeChirho::TermChirho {
            text_chirho: "word".to_string(),
            field_chirho: Some("text".to_string()),
        };
        if let QueryNodeChirho::TermChirho { text_chirho, field_chirho } = &node_chirho {
            assert_eq!(text_chirho, "word");
            assert_eq!(field_chirho.as_deref(), Some("text"));
        }
    }

    #[test]
    fn test_query_node_phrase_chirho() {
        let node_chirho = QueryNodeChirho::PhraseChirho {
            words_chirho: vec!["God".to_string(), "so".to_string(), "loved".to_string()],
            slop_chirho: 0,
        };
        if let QueryNodeChirho::PhraseChirho { words_chirho, slop_chirho } = &node_chirho {
            assert_eq!(words_chirho.len(), 3);
            assert_eq!(*slop_chirho, 0);
        }
    }

    #[test]
    fn test_query_node_and_chirho() {
        let node_chirho = QueryNodeChirho::AndChirho(vec![
            QueryNodeChirho::TermChirho { text_chirho: "grace".to_string(), field_chirho: None },
            QueryNodeChirho::TermChirho { text_chirho: "mercy".to_string(), field_chirho: None },
        ]);
        if let QueryNodeChirho::AndChirho(children_chirho) = &node_chirho {
            assert_eq!(children_chirho.len(), 2);
        }
    }

    #[test]
    fn test_query_node_or_chirho() {
        let node_chirho = QueryNodeChirho::OrChirho(vec![
            QueryNodeChirho::TermChirho { text_chirho: "love".to_string(), field_chirho: None },
            QueryNodeChirho::TermChirho { text_chirho: "charity".to_string(), field_chirho: None },
        ]);
        if let QueryNodeChirho::OrChirho(children_chirho) = &node_chirho {
            assert_eq!(children_chirho.len(), 2);
        }
    }

    #[test]
    fn test_query_node_not_chirho() {
        let node_chirho = QueryNodeChirho::NotChirho(Box::new(
            QueryNodeChirho::TermChirho { text_chirho: "sin".to_string(), field_chirho: None },
        ));
        assert!(matches!(node_chirho, QueryNodeChirho::NotChirho(_)));
    }

    #[test]
    fn test_query_node_strong_chirho() {
        let node_chirho = QueryNodeChirho::StrongChirho {
            number_chirho: "G26".to_string(),
        };
        if let QueryNodeChirho::StrongChirho { number_chirho } = &node_chirho {
            assert_eq!(number_chirho, "G26");
        }
    }

    #[test]
    fn test_query_node_lemma_chirho() {
        let node_chirho = QueryNodeChirho::LemmaChirho {
            lemma_chirho: "agape".to_string(),
        };
        if let QueryNodeChirho::LemmaChirho { lemma_chirho } = &node_chirho {
            assert_eq!(lemma_chirho, "agape");
        }
    }

    #[test]
    fn test_query_node_proximity_chirho() {
        let node_chirho = QueryNodeChirho::ProximityChirho {
            terms_chirho: vec!["God".to_string(), "loved".to_string()],
            distance_chirho: 5,
        };
        if let QueryNodeChirho::ProximityChirho { terms_chirho, distance_chirho } = &node_chirho {
            assert_eq!(terms_chirho.len(), 2);
            assert_eq!(*distance_chirho, 5);
        }
    }

    #[test]
    fn test_query_node_scoped_chirho() {
        let node_chirho = QueryNodeChirho::ScopedChirho {
            scope_chirho: ScopeChirho::BookChirho("John".to_string()),
            inner_chirho: Box::new(QueryNodeChirho::TermChirho {
                text_chirho: "light".to_string(),
                field_chirho: None,
            }),
        };
        assert!(matches!(node_chirho, QueryNodeChirho::ScopedChirho { .. }));
    }

    #[test]
    fn test_domain_node_serde_chirho() {
        let node_chirho = QueryNodeChirho::DomainChirho {
            domain_chirho: "love".to_string(),
        };
        let json_chirho = serde_json::to_string(&node_chirho).unwrap();
        let parsed_chirho: QueryNodeChirho = serde_json::from_str(&json_chirho).unwrap();
        if let QueryNodeChirho::DomainChirho { domain_chirho } = &parsed_chirho {
            assert_eq!(domain_chirho, "love");
        } else {
            panic!("Expected DomainChirho");
        }
    }

    #[test]
    fn test_sense_node_serde_chirho() {
        let node_chirho = QueryNodeChirho::SenseChirho {
            sense_chirho: "love.01".to_string(),
        };
        let json_chirho = serde_json::to_string(&node_chirho).unwrap();
        let parsed_chirho: QueryNodeChirho = serde_json::from_str(&json_chirho).unwrap();
        if let QueryNodeChirho::SenseChirho { sense_chirho } = &parsed_chirho {
            assert_eq!(sense_chirho, "love.01");
        } else {
            panic!("Expected SenseChirho");
        }
    }

    #[test]
    fn test_syntax_node_serde_chirho() {
        let node_chirho = QueryNodeChirho::SyntaxChirho {
            clause_type_chirho: "relative".to_string(),
            scope_chirho: None,
        };
        let json_chirho = serde_json::to_string(&node_chirho).unwrap();
        let parsed_chirho: QueryNodeChirho = serde_json::from_str(&json_chirho).unwrap();
        if let QueryNodeChirho::SyntaxChirho {
            clause_type_chirho, ..
        } = &parsed_chirho
        {
            assert_eq!(clause_type_chirho, "relative");
        } else {
            panic!("Expected SyntaxChirho");
        }
    }

    #[test]
    fn test_query_serde_roundtrip_chirho() {
        let q_chirho = QueryChirho::new_chirho(QueryNodeChirho::AndChirho(vec![
            QueryNodeChirho::TermChirho { text_chirho: "love".to_string(), field_chirho: None },
            QueryNodeChirho::StrongChirho { number_chirho: "G26".to_string() },
        ]))
        .with_max_results_chirho(50)
        .with_explain_chirho();

        let json_chirho = serde_json::to_string(&q_chirho).unwrap();
        let parsed_chirho: QueryChirho = serde_json::from_str(&json_chirho).unwrap();
        assert_eq!(parsed_chirho.max_results_chirho, 50);
        assert!(parsed_chirho.explain_chirho);
    }
}
