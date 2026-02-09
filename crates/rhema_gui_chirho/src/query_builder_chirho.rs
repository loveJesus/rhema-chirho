// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Visual query builder model — converts GUI drag-and-drop elements
//! into query text and back.

use rhema_query_chirho::QueryParserChirho;

/// The type of a query element in the builder.
#[derive(Clone, Debug, PartialEq)]
pub enum QueryElementChirho {
    /// A simple term input — e.g., "love".
    TermInputChirho { text_chirho: String },
    /// A prefixed input — e.g., "strong:G26".
    PrefixInputChirho {
        prefix_chirho: String,
        value_chirho: String,
    },
    /// A boolean or proximity operator joining elements.
    OperatorChirho { op_chirho: OperatorTypeChirho },
    /// A scope filter — e.g., "[John]".
    ScopeFilterChirho { scope_chirho: String },
    /// A grouped sub-expression (parenthesized).
    GroupChirho {
        children_chirho: Vec<QueryElementChirho>,
    },
}

/// Operator types for the visual builder.
#[derive(Clone, Debug, PartialEq)]
pub enum OperatorTypeChirho {
    AndChirho,
    OrChirho,
    NotChirho,
    NearChirho { distance_chirho: u32 },
}

/// The query builder model — holds a list of query elements and
/// provides serialization / deserialization against query text.
#[derive(Clone, Debug, Default)]
pub struct QueryBuilderModelChirho {
    elements_chirho: Vec<QueryElementChirho>,
}

impl QueryBuilderModelChirho {
    /// Create a new empty builder.
    pub fn new_chirho() -> Self {
        Self {
            elements_chirho: Vec::new(),
        }
    }

    /// Add an element to the end of the builder.
    pub fn add_chirho(&mut self, element_chirho: QueryElementChirho) {
        self.elements_chirho.push(element_chirho);
    }

    /// Remove the element at `index_chirho`. Returns `None` if out of bounds.
    pub fn remove_chirho(&mut self, index_chirho: usize) -> Option<QueryElementChirho> {
        if index_chirho < self.elements_chirho.len() {
            Some(self.elements_chirho.remove(index_chirho))
        } else {
            None
        }
    }

    /// Return a read-only view of the current elements.
    pub fn elements_chirho(&self) -> &[QueryElementChirho] {
        &self.elements_chirho
    }

    /// Serialize the builder contents to a query string.
    pub fn to_query_text_chirho(&self) -> String {
        Self::elements_to_text_chirho(&self.elements_chirho)
    }

    /// Parse a query string into a builder model.
    ///
    /// This performs a best-effort tokenization of the query text.
    /// The round-trip is **not** lossless for complex nested expressions —
    /// it is intended to give users a starting point for visual editing.
    pub fn from_query_text_chirho(input_chirho: &str) -> Result<Self, String> {
        // Validate that the input parses at all.
        QueryParserChirho::parse_chirho(input_chirho)
            .map_err(|e_chirho| e_chirho.to_string())?;

        let mut model_chirho = Self::new_chirho();
        let trimmed_chirho = input_chirho.trim();

        // Extract leading scope: [Book]
        let remaining_chirho = if trimmed_chirho.starts_with('[') {
            if let Some(end_chirho) = trimmed_chirho.find(']') {
                let scope_chirho = &trimmed_chirho[1..end_chirho];
                model_chirho.add_chirho(QueryElementChirho::ScopeFilterChirho {
                    scope_chirho: scope_chirho.to_string(),
                });
                trimmed_chirho[end_chirho + 1..].trim()
            } else {
                trimmed_chirho
            }
        } else {
            trimmed_chirho
        };

        // Tokenize the remaining query text.
        let tokens_chirho = Self::tokenize_chirho(remaining_chirho);
        for token_chirho in &tokens_chirho {
            match token_chirho.as_str() {
                "AND" => model_chirho.add_chirho(QueryElementChirho::OperatorChirho {
                    op_chirho: OperatorTypeChirho::AndChirho,
                }),
                "OR" => model_chirho.add_chirho(QueryElementChirho::OperatorChirho {
                    op_chirho: OperatorTypeChirho::OrChirho,
                }),
                "NOT" => model_chirho.add_chirho(QueryElementChirho::OperatorChirho {
                    op_chirho: OperatorTypeChirho::NotChirho,
                }),
                t_chirho if t_chirho.starts_with("NEAR/") => {
                    let dist_chirho = t_chirho[5..]
                        .parse::<u32>()
                        .unwrap_or(5);
                    model_chirho.add_chirho(QueryElementChirho::OperatorChirho {
                        op_chirho: OperatorTypeChirho::NearChirho {
                            distance_chirho: dist_chirho,
                        },
                    });
                }
                t_chirho if t_chirho.contains(':') => {
                    let (prefix_chirho, value_chirho) =
                        t_chirho.split_once(':').unwrap();
                    model_chirho.add_chirho(QueryElementChirho::PrefixInputChirho {
                        prefix_chirho: format!("{prefix_chirho}:"),
                        value_chirho: value_chirho.to_string(),
                    });
                }
                t_chirho => {
                    model_chirho.add_chirho(QueryElementChirho::TermInputChirho {
                        text_chirho: t_chirho.to_string(),
                    });
                }
            }
        }

        Ok(model_chirho)
    }

    /// Return all supported query prefixes.
    pub fn available_prefixes_chirho() -> Vec<&'static str> {
        vec![
            "strong:", "lemma:", "semantic:", "~", "concept:", "expand:",
            "xref:", "rel:", "prop:", "morph:", "pos:", "tense:", "voice:",
            "mood:", "case:", "number:", "gender:", "person:", "domain:",
            "sense:", "syntax:", "clause:",
        ]
    }

    // ── private helpers ────────────────────────────────────────

    fn elements_to_text_chirho(elements_chirho: &[QueryElementChirho]) -> String {
        let mut parts_chirho: Vec<String> = Vec::new();

        for elem_chirho in elements_chirho {
            match elem_chirho {
                QueryElementChirho::TermInputChirho { text_chirho } => {
                    parts_chirho.push(text_chirho.clone());
                }
                QueryElementChirho::PrefixInputChirho {
                    prefix_chirho,
                    value_chirho,
                } => {
                    parts_chirho.push(format!("{prefix_chirho}{value_chirho}"));
                }
                QueryElementChirho::OperatorChirho { op_chirho } => match op_chirho {
                    OperatorTypeChirho::AndChirho => parts_chirho.push("AND".to_string()),
                    OperatorTypeChirho::OrChirho => parts_chirho.push("OR".to_string()),
                    OperatorTypeChirho::NotChirho => parts_chirho.push("NOT".to_string()),
                    OperatorTypeChirho::NearChirho { distance_chirho } => {
                        parts_chirho.push(format!("NEAR/{distance_chirho}"));
                    }
                },
                QueryElementChirho::ScopeFilterChirho { scope_chirho } => {
                    parts_chirho.push(format!("[{scope_chirho}]"));
                }
                QueryElementChirho::GroupChirho { children_chirho } => {
                    let inner_chirho = Self::elements_to_text_chirho(children_chirho);
                    parts_chirho.push(format!("({inner_chirho})"));
                }
            }
        }

        parts_chirho.join(" ")
    }

    fn tokenize_chirho(input_chirho: &str) -> Vec<String> {
        let mut tokens_chirho = Vec::new();
        let mut chars_chirho = input_chirho.chars().peekable();
        let mut buf_chirho = String::new();

        while let Some(&ch_chirho) = chars_chirho.peek() {
            match ch_chirho {
                ' ' | '\t' | '\n' => {
                    if !buf_chirho.is_empty() {
                        tokens_chirho.push(std::mem::take(&mut buf_chirho));
                    }
                    chars_chirho.next();
                }
                '"' => {
                    // Consume quoted phrase as a single token.
                    chars_chirho.next();
                    let mut phrase_chirho = String::from("\"");
                    while let Some(&c_chirho) = chars_chirho.peek() {
                        chars_chirho.next();
                        phrase_chirho.push(c_chirho);
                        if c_chirho == '"' {
                            break;
                        }
                    }
                    if !buf_chirho.is_empty() {
                        tokens_chirho.push(std::mem::take(&mut buf_chirho));
                    }
                    tokens_chirho.push(phrase_chirho);
                }
                _ => {
                    chars_chirho.next();
                    buf_chirho.push(ch_chirho);
                }
            }
        }

        if !buf_chirho.is_empty() {
            tokens_chirho.push(buf_chirho);
        }

        tokens_chirho
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_empty_builder_chirho() {
        let model_chirho = QueryBuilderModelChirho::new_chirho();
        assert!(model_chirho.elements_chirho().is_empty());
        assert_eq!(model_chirho.to_query_text_chirho(), "");
    }

    #[test]
    fn test_add_term_chirho() {
        let mut model_chirho = QueryBuilderModelChirho::new_chirho();
        model_chirho.add_chirho(QueryElementChirho::TermInputChirho {
            text_chirho: "love".to_string(),
        });
        assert_eq!(model_chirho.elements_chirho().len(), 1);
        assert_eq!(model_chirho.to_query_text_chirho(), "love");
    }

    #[test]
    fn test_and_expression_chirho() {
        let mut model_chirho = QueryBuilderModelChirho::new_chirho();
        model_chirho.add_chirho(QueryElementChirho::TermInputChirho {
            text_chirho: "love".to_string(),
        });
        model_chirho.add_chirho(QueryElementChirho::OperatorChirho {
            op_chirho: OperatorTypeChirho::AndChirho,
        });
        model_chirho.add_chirho(QueryElementChirho::TermInputChirho {
            text_chirho: "world".to_string(),
        });
        assert_eq!(model_chirho.to_query_text_chirho(), "love AND world");
    }

    #[test]
    fn test_prefix_input_chirho() {
        let mut model_chirho = QueryBuilderModelChirho::new_chirho();
        model_chirho.add_chirho(QueryElementChirho::PrefixInputChirho {
            prefix_chirho: "strong:".to_string(),
            value_chirho: "G26".to_string(),
        });
        assert_eq!(model_chirho.to_query_text_chirho(), "strong:G26");
    }

    #[test]
    fn test_scope_filter_chirho() {
        let mut model_chirho = QueryBuilderModelChirho::new_chirho();
        model_chirho.add_chirho(QueryElementChirho::ScopeFilterChirho {
            scope_chirho: "John".to_string(),
        });
        model_chirho.add_chirho(QueryElementChirho::TermInputChirho {
            text_chirho: "love".to_string(),
        });
        assert_eq!(model_chirho.to_query_text_chirho(), "[John] love");
    }

    #[test]
    fn test_remove_element_chirho() {
        let mut model_chirho = QueryBuilderModelChirho::new_chirho();
        model_chirho.add_chirho(QueryElementChirho::TermInputChirho {
            text_chirho: "love".to_string(),
        });
        model_chirho.add_chirho(QueryElementChirho::TermInputChirho {
            text_chirho: "world".to_string(),
        });
        let removed_chirho = model_chirho.remove_chirho(0).unwrap();
        assert_eq!(
            removed_chirho,
            QueryElementChirho::TermInputChirho {
                text_chirho: "love".to_string(),
            }
        );
        assert_eq!(model_chirho.elements_chirho().len(), 1);

        // Out-of-bounds returns None.
        assert!(model_chirho.remove_chirho(99).is_none());
    }

    #[test]
    fn test_roundtrip_chirho() {
        let input_chirho = "love AND world";
        let model_chirho =
            QueryBuilderModelChirho::from_query_text_chirho(input_chirho).unwrap();
        let output_chirho = model_chirho.to_query_text_chirho();
        assert_eq!(output_chirho, "love AND world");
    }

    #[test]
    fn test_complex_query_chirho() {
        let mut model_chirho = QueryBuilderModelChirho::new_chirho();
        model_chirho.add_chirho(QueryElementChirho::ScopeFilterChirho {
            scope_chirho: "Romans".to_string(),
        });
        model_chirho.add_chirho(QueryElementChirho::PrefixInputChirho {
            prefix_chirho: "strong:".to_string(),
            value_chirho: "G26".to_string(),
        });
        model_chirho.add_chirho(QueryElementChirho::OperatorChirho {
            op_chirho: OperatorTypeChirho::AndChirho,
        });
        model_chirho.add_chirho(QueryElementChirho::TermInputChirho {
            text_chirho: "love".to_string(),
        });

        let text_chirho = model_chirho.to_query_text_chirho();
        assert_eq!(text_chirho, "[Romans] strong:G26 AND love");

        // Verify it parses.
        let reparsed_chirho =
            QueryBuilderModelChirho::from_query_text_chirho(&text_chirho).unwrap();
        assert_eq!(reparsed_chirho.elements_chirho().len(), 4);
    }

    #[test]
    fn test_available_prefixes_chirho() {
        let prefixes_chirho = QueryBuilderModelChirho::available_prefixes_chirho();
        assert!(prefixes_chirho.contains(&"strong:"));
        assert!(prefixes_chirho.contains(&"domain:"));
        assert!(prefixes_chirho.contains(&"syntax:"));
        assert!(prefixes_chirho.len() >= 20);
    }
}
