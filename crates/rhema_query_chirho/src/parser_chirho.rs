// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Query parser — converts text queries into [`QueryNodeChirho`] IR.
//!
//! Supports:
//! - Simple terms: `love`
//! - Quoted phrases: `"God so loved"`
//! - Boolean operators: `AND`, `OR`, `NOT`
//! - Strong's numbers: `strong:G26`
//! - Lemma search: `lemma:agape`
//! - Proximity: `love NEAR/5 world`
//! - Scope restrictions: `[John]` or `[Genesis-Deuteronomy]`

use rhema_contracts_chirho::keys_chirho::ScopeChirho;
use rhema_contracts_chirho::morph_parser_chirho::{
    parse_case_name_chirho, parse_gender_name_chirho, parse_mood_name_chirho,
    parse_number_name_chirho, parse_person_name_chirho, parse_pos_name_chirho,
    parse_tense_name_chirho, parse_voice_name_chirho, MorphParserChirho,
};
use rhema_contracts_chirho::morphology_chirho::MorphConstraintChirho;
use rhema_contracts_chirho::query_chirho::{QueryChirho, QueryNodeChirho};

use crate::error_chirho::QueryErrorChirho;

/// The query parser.
pub struct QueryParserChirho;

impl QueryParserChirho {
    /// Parse a query string into a [`QueryChirho`].
    pub fn parse_chirho(input_chirho: &str) -> Result<QueryChirho, QueryErrorChirho> {
        let trimmed_chirho = input_chirho.trim();
        if trimmed_chirho.is_empty() {
            return Err(QueryErrorChirho::EmptyQueryChirho);
        }

        let (scope_chirho, query_text_chirho) = extract_scope_chirho(trimmed_chirho)?;
        let root_chirho = parse_expression_chirho(query_text_chirho.trim())?;

        let mut query_chirho = QueryChirho::new_chirho(root_chirho);
        if let Some(scope_chirho) = scope_chirho {
            query_chirho = query_chirho.with_scope_chirho(scope_chirho);
        }

        Ok(query_chirho)
    }
}

/// Extract an optional scope prefix like `[John]` or `[Genesis]` from the query.
fn extract_scope_chirho(
    input_chirho: &str,
) -> Result<(Option<ScopeChirho>, &str), QueryErrorChirho> {
    if !input_chirho.starts_with('[') {
        return Ok((None, input_chirho));
    }

    let close_chirho = input_chirho.find(']').ok_or(
        QueryErrorChirho::UnmatchedBracketChirho {
            position_chirho: 0,
        },
    )?;

    let scope_text_chirho = &input_chirho[1..close_chirho];
    let remainder_chirho = &input_chirho[close_chirho + 1..];

    let scope_chirho = parse_scope_text_chirho(scope_text_chirho);

    Ok((Some(scope_chirho), remainder_chirho))
}

/// Parse scope text into a ScopeChirho.
fn parse_scope_text_chirho(text_chirho: &str) -> ScopeChirho {
    let trimmed_chirho = text_chirho.trim();

    match trimmed_chirho.to_lowercase().as_str() {
        "ot" | "old testament" => {
            ScopeChirho::TestamentChirho(rhema_contracts_chirho::keys_chirho::TestamentChirho::OldTestamentChirho)
        }
        "nt" | "new testament" => {
            ScopeChirho::TestamentChirho(rhema_contracts_chirho::keys_chirho::TestamentChirho::NewTestamentChirho)
        }
        _ => {
            // Could be a single book or comma-separated list
            if trimmed_chirho.contains(',') {
                let books_chirho: Vec<String> = trimmed_chirho
                    .split(',')
                    .map(|b_chirho| b_chirho.trim().to_string())
                    .collect();
                ScopeChirho::BooksChirho(books_chirho)
            } else {
                ScopeChirho::BookChirho(trimmed_chirho.to_string())
            }
        }
    }
}

/// Parse a query expression with boolean operators.
fn parse_expression_chirho(input_chirho: &str) -> Result<QueryNodeChirho, QueryErrorChirho> {
    if input_chirho.is_empty() {
        return Err(QueryErrorChirho::EmptyQueryChirho);
    }

    // Split by OR first (lowest precedence)
    let or_parts_chirho = split_top_level_chirho(input_chirho, "OR");
    if or_parts_chirho.len() > 1 {
        let mut children_chirho = Vec::new();
        for part_chirho in or_parts_chirho {
            children_chirho.push(parse_expression_chirho(part_chirho.trim())?);
        }
        return Ok(QueryNodeChirho::OrChirho(children_chirho));
    }

    // Split by AND (higher precedence than OR)
    let and_parts_chirho = split_top_level_chirho(input_chirho, "AND");
    if and_parts_chirho.len() > 1 {
        let mut children_chirho = Vec::new();
        for part_chirho in and_parts_chirho {
            children_chirho.push(parse_expression_chirho(part_chirho.trim())?);
        }
        return Ok(QueryNodeChirho::AndChirho(children_chirho));
    }

    // Check for NOT prefix
    if let Some(rest_chirho) = input_chirho.strip_prefix("NOT ") {
        let inner_chirho = parse_expression_chirho(rest_chirho.trim())?;
        return Ok(QueryNodeChirho::NotChirho(Box::new(inner_chirho)));
    }

    // Check for XREF/N graph expansion: `XREF/2 John 3:16`
    if let Some(xref_chirho) = try_parse_xref_operator_chirho(input_chirho)? {
        return Ok(xref_chirho);
    }

    // Check for NEAR/N proximity
    if let Some(prox_chirho) = try_parse_proximity_chirho(input_chirho)? {
        return Ok(prox_chirho);
    }

    // Check for prefix operators
    if let Some(node_chirho) = try_parse_prefix_chirho(input_chirho)? {
        return Ok(node_chirho);
    }

    // Check for quoted phrase
    if input_chirho.starts_with('"') {
        return parse_phrase_chirho(input_chirho);
    }

    // Multi-word without explicit operator → implicit AND
    // Handle inline NOT: "love NOT hate" → AND(Term("love"), NOT(Term("hate")))
    let words_chirho: Vec<&str> = input_chirho.split_whitespace().collect();
    if words_chirho.len() > 1 {
        let mut children_chirho = Vec::new();
        let mut i_chirho = 0;
        while i_chirho < words_chirho.len() {
            if words_chirho[i_chirho] == "NOT" && i_chirho + 1 < words_chirho.len() {
                // Collect the rest after NOT as the negated expression
                let rest_chirho = words_chirho[i_chirho + 1..].join(" ");
                let inner_chirho = parse_expression_chirho(&rest_chirho)?;
                children_chirho.push(QueryNodeChirho::NotChirho(Box::new(inner_chirho)));
                break; // NOT consumes the rest
            } else {
                children_chirho.push(parse_expression_chirho(words_chirho[i_chirho])?);
            }
            i_chirho += 1;
        }
        return Ok(QueryNodeChirho::AndChirho(children_chirho));
    }

    // Single term
    Ok(QueryNodeChirho::TermChirho {
        text_chirho: input_chirho.to_string(),
        field_chirho: None,
    })
}

/// Try to parse an XREF/N operator like `XREF/2 John 3:16`.
fn try_parse_xref_operator_chirho(
    input_chirho: &str,
) -> Result<Option<QueryNodeChirho>, QueryErrorChirho> {
    let xref_re_chirho = regex::Regex::new(r"(?i)^XREF/(\d+)\s+(.+)$").unwrap();

    if let Some(captures_chirho) = xref_re_chirho.captures(input_chirho) {
        let depth_chirho: u32 = captures_chirho
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .map_err(|_| QueryErrorChirho::InvalidXrefChirho {
                value_chirho: captures_chirho.get(1).unwrap().as_str().to_string(),
            })?;
        let seed_chirho = captures_chirho.get(2).unwrap().as_str().trim().to_string();

        return Ok(Some(QueryNodeChirho::GraphExpandChirho {
            seed_chirho,
            depth_chirho,
        }));
    }

    Ok(None)
}

/// Try to parse a proximity expression like `love NEAR/5 world`.
fn try_parse_proximity_chirho(
    input_chirho: &str,
) -> Result<Option<QueryNodeChirho>, QueryErrorChirho> {
    let near_re_chirho = regex::Regex::new(r"(?i)(.+)\s+NEAR/(\d+)\s+(.+)").unwrap();

    if let Some(captures_chirho) = near_re_chirho.captures(input_chirho) {
        let left_chirho = captures_chirho.get(1).unwrap().as_str().trim();
        let distance_chirho: u32 = captures_chirho
            .get(2)
            .unwrap()
            .as_str()
            .parse()
            .map_err(|_| QueryErrorChirho::InvalidProximityChirho {
                value_chirho: captures_chirho.get(2).unwrap().as_str().to_string(),
            })?;
        let right_chirho = captures_chirho.get(3).unwrap().as_str().trim();

        let terms_chirho = vec![left_chirho.to_string(), right_chirho.to_string()];

        return Ok(Some(QueryNodeChirho::ProximityChirho {
            terms_chirho,
            distance_chirho,
        }));
    }

    Ok(None)
}

/// Try to parse prefix operators like `strong:G26`, `lemma:agape`,
/// `semantic:query`, `~query`, `concept:redemption`, `expand:query`,
/// `rel:Ground`, `prop:text`.
fn try_parse_prefix_chirho(
    input_chirho: &str,
) -> Result<Option<QueryNodeChirho>, QueryErrorChirho> {
    if let Some(value_chirho) = input_chirho.strip_prefix("strong:") {
        return Ok(Some(QueryNodeChirho::StrongChirho {
            number_chirho: value_chirho.to_string(),
        }));
    }

    if let Some(value_chirho) = input_chirho.strip_prefix("lemma:") {
        return Ok(Some(QueryNodeChirho::LemmaChirho {
            lemma_chirho: value_chirho.to_string(),
        }));
    }

    // Phase 2: Semantic search — `semantic:query` or `~query`
    if let Some(value_chirho) = input_chirho.strip_prefix("semantic:") {
        return Ok(Some(QueryNodeChirho::SemanticChirho {
            query_text_chirho: value_chirho.to_string(),
            top_k_chirho: 20,
            similarity_threshold_chirho: 0.5,
        }));
    }

    if let Some(value_chirho) = input_chirho.strip_prefix('~') {
        return Ok(Some(QueryNodeChirho::SemanticChirho {
            query_text_chirho: value_chirho.to_string(),
            top_k_chirho: 20,
            similarity_threshold_chirho: 0.5,
        }));
    }

    // Phase 2: Concept search — `concept:redemption`
    if let Some(value_chirho) = input_chirho.strip_prefix("concept:") {
        return Ok(Some(QueryNodeChirho::ConceptChirho {
            concept_chirho: value_chirho.to_string(),
            include_related_chirho: true,
        }));
    }

    // Phase 2: Expanded search — `expand:God's faithfulness`
    if let Some(value_chirho) = input_chirho.strip_prefix("expand:") {
        return Ok(Some(QueryNodeChirho::ExpandedChirho {
            original_query_chirho: value_chirho.to_string(),
            expanded_terms_chirho: Vec::new(),
            expanded_strongs_chirho: Vec::new(),
            expansion_strategy_chirho:
                rhema_contracts_chirho::query_chirho::ExpansionStrategyChirho::FullChirho,
        }));
    }

    // Phase 4: Cross-reference graph search — `xref:John.3.16` or `xref:John 3:16`
    if let Some(value_chirho) = input_chirho.strip_prefix("xref:") {
        return Ok(Some(QueryNodeChirho::GraphExpandChirho {
            seed_chirho: value_chirho.to_string(),
            depth_chirho: 1,
        }));
    }

    // Phase 2: Discourse relationship search — `rel:Ground`
    if let Some(value_chirho) = input_chirho.strip_prefix("rel:") {
        return Ok(Some(QueryNodeChirho::DiscourseRelationshipChirho {
            relationship_type_chirho: value_chirho.to_string(),
            scope_chirho: None,
        }));
    }

    // Phase 2: Proposition text search — `prop:"resurrection"`
    if let Some(value_chirho) = input_chirho.strip_prefix("prop:") {
        let clean_chirho = value_chirho.trim_matches('"');
        return Ok(Some(QueryNodeChirho::PropositionTextChirho {
            text_chirho: clean_chirho.to_string(),
            scope_chirho: None,
        }));
    }

    // Phase 5+: Semantic domain search — `domain:love`
    if let Some(value_chirho) = input_chirho.strip_prefix("domain:") {
        return Ok(Some(QueryNodeChirho::DomainChirho {
            domain_chirho: value_chirho.to_string(),
        }));
    }

    // Phase 5+: Sense search — `sense:love.01`
    if let Some(value_chirho) = input_chirho.strip_prefix("sense:") {
        return Ok(Some(QueryNodeChirho::SenseChirho {
            sense_chirho: value_chirho.to_string(),
        }));
    }

    // Phase 5+: Syntax/clause search — `syntax:relative` or `clause:conditional`
    if let Some(value_chirho) = input_chirho.strip_prefix("syntax:") {
        return Ok(Some(QueryNodeChirho::SyntaxChirho {
            clause_type_chirho: value_chirho.to_string(),
            scope_chirho: None,
        }));
    }

    if let Some(value_chirho) = input_chirho.strip_prefix("clause:") {
        return Ok(Some(QueryNodeChirho::SyntaxChirho {
            clause_type_chirho: value_chirho.to_string(),
            scope_chirho: None,
        }));
    }

    // Phase 3: Morphology search — `morph:V-AAI-3S` (full Robinson/OSHM code)
    if let Some(value_chirho) = input_chirho.strip_prefix("morph:") {
        let constraint_chirho = MorphParserChirho::constraint_from_code_chirho(value_chirho)
            .map_err(|_| QueryErrorChirho::InvalidMorphSyntaxChirho {
                value_chirho: value_chirho.to_string(),
            })?;
        return Ok(Some(QueryNodeChirho::MorphChirho(constraint_chirho)));
    }

    // Phase 3: Individual morph facet prefixes
    if let Some(value_chirho) = input_chirho.strip_prefix("pos:") {
        let pos_chirho = parse_pos_name_chirho(value_chirho).ok_or(
            QueryErrorChirho::InvalidMorphSyntaxChirho {
                value_chirho: value_chirho.to_string(),
            },
        )?;
        return Ok(Some(QueryNodeChirho::MorphChirho(MorphConstraintChirho {
            part_of_speech_chirho: Some(pos_chirho),
            ..Default::default()
        })));
    }

    if let Some(value_chirho) = input_chirho.strip_prefix("tense:") {
        let tense_chirho = parse_tense_name_chirho(value_chirho).ok_or(
            QueryErrorChirho::InvalidMorphSyntaxChirho {
                value_chirho: value_chirho.to_string(),
            },
        )?;
        return Ok(Some(QueryNodeChirho::MorphChirho(MorphConstraintChirho {
            tense_chirho: Some(tense_chirho),
            ..Default::default()
        })));
    }

    if let Some(value_chirho) = input_chirho.strip_prefix("voice:") {
        let voice_chirho = parse_voice_name_chirho(value_chirho).ok_or(
            QueryErrorChirho::InvalidMorphSyntaxChirho {
                value_chirho: value_chirho.to_string(),
            },
        )?;
        return Ok(Some(QueryNodeChirho::MorphChirho(MorphConstraintChirho {
            voice_chirho: Some(voice_chirho),
            ..Default::default()
        })));
    }

    if let Some(value_chirho) = input_chirho.strip_prefix("mood:") {
        let mood_chirho = parse_mood_name_chirho(value_chirho).ok_or(
            QueryErrorChirho::InvalidMorphSyntaxChirho {
                value_chirho: value_chirho.to_string(),
            },
        )?;
        return Ok(Some(QueryNodeChirho::MorphChirho(MorphConstraintChirho {
            mood_chirho: Some(mood_chirho),
            ..Default::default()
        })));
    }

    if let Some(value_chirho) = input_chirho.strip_prefix("case:") {
        let case_chirho = parse_case_name_chirho(value_chirho).ok_or(
            QueryErrorChirho::InvalidMorphSyntaxChirho {
                value_chirho: value_chirho.to_string(),
            },
        )?;
        return Ok(Some(QueryNodeChirho::MorphChirho(MorphConstraintChirho {
            case_chirho: Some(case_chirho),
            ..Default::default()
        })));
    }

    if let Some(value_chirho) = input_chirho.strip_prefix("number:") {
        let number_chirho = parse_number_name_chirho(value_chirho).ok_or(
            QueryErrorChirho::InvalidMorphSyntaxChirho {
                value_chirho: value_chirho.to_string(),
            },
        )?;
        return Ok(Some(QueryNodeChirho::MorphChirho(MorphConstraintChirho {
            number_chirho: Some(number_chirho),
            ..Default::default()
        })));
    }

    if let Some(value_chirho) = input_chirho.strip_prefix("gender:") {
        let gender_chirho = parse_gender_name_chirho(value_chirho).ok_or(
            QueryErrorChirho::InvalidMorphSyntaxChirho {
                value_chirho: value_chirho.to_string(),
            },
        )?;
        return Ok(Some(QueryNodeChirho::MorphChirho(MorphConstraintChirho {
            gender_chirho: Some(gender_chirho),
            ..Default::default()
        })));
    }

    if let Some(value_chirho) = input_chirho.strip_prefix("person:") {
        let person_chirho = parse_person_name_chirho(value_chirho).ok_or(
            QueryErrorChirho::InvalidMorphSyntaxChirho {
                value_chirho: value_chirho.to_string(),
            },
        )?;
        return Ok(Some(QueryNodeChirho::MorphChirho(MorphConstraintChirho {
            person_chirho: Some(person_chirho),
            ..Default::default()
        })));
    }

    Ok(None)
}

/// Parse a quoted phrase like `"God so loved"`.
fn parse_phrase_chirho(input_chirho: &str) -> Result<QueryNodeChirho, QueryErrorChirho> {
    let without_start_chirho = &input_chirho[1..];
    let end_chirho = without_start_chirho
        .find('"')
        .ok_or(QueryErrorChirho::UnmatchedQuoteChirho {
            position_chirho: 0,
        })?;

    let phrase_text_chirho = &without_start_chirho[..end_chirho];
    let words_chirho: Vec<String> = phrase_text_chirho
        .split_whitespace()
        .map(|w_chirho| w_chirho.to_string())
        .collect();

    Ok(QueryNodeChirho::PhraseChirho {
        words_chirho,
        slop_chirho: 0,
    })
}

/// Split a string at top-level occurrences of `operator` (respecting quotes).
fn split_top_level_chirho<'a>(input_chirho: &'a str, operator_chirho: &str) -> Vec<&'a str> {
    let mut parts_chirho = Vec::new();
    let mut depth_chirho: i32 = 0;
    let mut in_quote_chirho = false;
    let mut last_split_chirho = 0;
    let op_with_spaces_chirho = format!(" {} ", operator_chirho);
    let bytes_chirho = input_chirho.as_bytes();

    let mut i_chirho = 0;
    while i_chirho < bytes_chirho.len() {
        match bytes_chirho[i_chirho] {
            b'"' => in_quote_chirho = !in_quote_chirho,
            b'(' => depth_chirho += 1,
            b')' => depth_chirho -= 1,
            _ if !in_quote_chirho
                && depth_chirho == 0
                && input_chirho[i_chirho..].starts_with(&op_with_spaces_chirho) =>
            {
                parts_chirho.push(&input_chirho[last_split_chirho..i_chirho]);
                i_chirho += op_with_spaces_chirho.len();
                last_split_chirho = i_chirho;
                continue;
            }
            _ => {}
        }
        i_chirho += 1;
    }

    parts_chirho.push(&input_chirho[last_split_chirho..]);
    parts_chirho
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_simple_term_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("love").unwrap();
        match &query_chirho.root_chirho {
            QueryNodeChirho::TermChirho { text_chirho, .. } => {
                assert_eq!(text_chirho, "love");
            }
            other_chirho => panic!("Expected TermChirho, got {:?}", other_chirho),
        }
    }

    #[test]
    fn test_phrase_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("\"God so loved\"").unwrap();
        match &query_chirho.root_chirho {
            QueryNodeChirho::PhraseChirho {
                words_chirho,
                slop_chirho,
            } => {
                assert_eq!(words_chirho, &["God", "so", "loved"]);
                assert_eq!(*slop_chirho, 0);
            }
            other_chirho => panic!("Expected PhraseChirho, got {:?}", other_chirho),
        }
    }

    #[test]
    fn test_boolean_and_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("love AND world").unwrap();
        match &query_chirho.root_chirho {
            QueryNodeChirho::AndChirho(children_chirho) => {
                assert_eq!(children_chirho.len(), 2);
            }
            other_chirho => panic!("Expected AndChirho, got {:?}", other_chirho),
        }
    }

    #[test]
    fn test_boolean_or_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("grace OR mercy").unwrap();
        match &query_chirho.root_chirho {
            QueryNodeChirho::OrChirho(children_chirho) => {
                assert_eq!(children_chirho.len(), 2);
            }
            other_chirho => panic!("Expected OrChirho, got {:?}", other_chirho),
        }
    }

    #[test]
    fn test_boolean_not_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("NOT sin").unwrap();
        match &query_chirho.root_chirho {
            QueryNodeChirho::NotChirho(inner_chirho) => {
                match inner_chirho.as_ref() {
                    QueryNodeChirho::TermChirho { text_chirho, .. } => {
                        assert_eq!(text_chirho, "sin");
                    }
                    other_chirho => panic!("Expected TermChirho inside NOT, got {:?}", other_chirho),
                }
            }
            other_chirho => panic!("Expected NotChirho, got {:?}", other_chirho),
        }
    }

    #[test]
    fn test_strong_number_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("strong:G26").unwrap();
        match &query_chirho.root_chirho {
            QueryNodeChirho::StrongChirho { number_chirho } => {
                assert_eq!(number_chirho, "G26");
            }
            other_chirho => panic!("Expected StrongChirho, got {:?}", other_chirho),
        }
    }

    #[test]
    fn test_lemma_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("lemma:agape").unwrap();
        match &query_chirho.root_chirho {
            QueryNodeChirho::LemmaChirho { lemma_chirho } => {
                assert_eq!(lemma_chirho, "agape");
            }
            other_chirho => panic!("Expected LemmaChirho, got {:?}", other_chirho),
        }
    }

    #[test]
    fn test_proximity_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("love NEAR/5 world").unwrap();
        match &query_chirho.root_chirho {
            QueryNodeChirho::ProximityChirho {
                terms_chirho,
                distance_chirho,
            } => {
                assert_eq!(terms_chirho, &["love", "world"]);
                assert_eq!(*distance_chirho, 5);
            }
            other_chirho => panic!("Expected ProximityChirho, got {:?}", other_chirho),
        }
    }

    #[test]
    fn test_scope_book_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("[John] love").unwrap();
        assert!(query_chirho.scope_chirho.is_some());
        match &query_chirho.scope_chirho {
            Some(ScopeChirho::BookChirho(book_chirho)) => {
                assert_eq!(book_chirho, "John");
            }
            other_chirho => panic!("Expected BookChirho scope, got {:?}", other_chirho),
        }
    }

    #[test]
    fn test_scope_testament_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("[NT] grace").unwrap();
        match &query_chirho.scope_chirho {
            Some(ScopeChirho::TestamentChirho(t_chirho)) => {
                assert_eq!(
                    *t_chirho,
                    rhema_contracts_chirho::keys_chirho::TestamentChirho::NewTestamentChirho
                );
            }
            other_chirho => panic!("Expected TestamentChirho scope, got {:?}", other_chirho),
        }
    }

    #[test]
    fn test_empty_query_chirho() {
        assert!(QueryParserChirho::parse_chirho("").is_err());
        assert!(QueryParserChirho::parse_chirho("  ").is_err());
    }

    #[test]
    fn test_implicit_and_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("God love world").unwrap();
        match &query_chirho.root_chirho {
            QueryNodeChirho::AndChirho(children_chirho) => {
                assert_eq!(children_chirho.len(), 3);
            }
            other_chirho => panic!("Expected implicit AndChirho, got {:?}", other_chirho),
        }
    }

    // ── Edge case tests ───────────────────────────────────────────

    #[test]
    fn test_whitespace_trimming_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("   love   ").unwrap();
        match &query_chirho.root_chirho {
            QueryNodeChirho::TermChirho { text_chirho, .. } => {
                assert_eq!(text_chirho, "love");
            }
            other_chirho => panic!("Expected TermChirho, got {:?}", other_chirho),
        }
    }

    #[test]
    fn test_mixed_case_operators_chirho() {
        // AND should be case-sensitive (uppercase only)
        let query_chirho = QueryParserChirho::parse_chirho("love AND world").unwrap();
        assert!(matches!(query_chirho.root_chirho, QueryNodeChirho::AndChirho(_)));
    }

    #[test]
    fn test_strong_hebrew_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("strong:H430").unwrap();
        match &query_chirho.root_chirho {
            QueryNodeChirho::StrongChirho { number_chirho } => {
                assert_eq!(number_chirho, "H430");
            }
            other_chirho => panic!("Expected StrongChirho, got {:?}", other_chirho),
        }
    }

    #[test]
    fn test_strong_large_number_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("strong:G3056").unwrap();
        match &query_chirho.root_chirho {
            QueryNodeChirho::StrongChirho { number_chirho } => {
                assert_eq!(number_chirho, "G3056");
            }
            other_chirho => panic!("Expected StrongChirho, got {:?}", other_chirho),
        }
    }

    #[test]
    fn test_scope_ot_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("[OT] covenant").unwrap();
        match &query_chirho.scope_chirho {
            Some(ScopeChirho::TestamentChirho(t_chirho)) => {
                assert_eq!(
                    *t_chirho,
                    rhema_contracts_chirho::keys_chirho::TestamentChirho::OldTestamentChirho
                );
            }
            other_chirho => panic!("Expected OT scope, got {:?}", other_chirho),
        }
    }

    #[test]
    fn test_scope_with_and_query_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("[Romans] grace AND mercy").unwrap();
        assert!(query_chirho.scope_chirho.is_some());
        assert!(matches!(query_chirho.root_chirho, QueryNodeChirho::AndChirho(_)));
    }

    #[test]
    fn test_phrase_single_word_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("\"love\"").unwrap();
        match &query_chirho.root_chirho {
            QueryNodeChirho::PhraseChirho { words_chirho, .. } => {
                assert_eq!(words_chirho, &["love"]);
            }
            other_chirho => panic!("Expected PhraseChirho, got {:?}", other_chirho),
        }
    }

    #[test]
    fn test_two_word_implicit_and_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("grace mercy").unwrap();
        match &query_chirho.root_chirho {
            QueryNodeChirho::AndChirho(children_chirho) => {
                assert_eq!(children_chirho.len(), 2);
            }
            other_chirho => panic!("Expected implicit AndChirho, got {:?}", other_chirho),
        }
    }

    #[test]
    fn test_and_with_strong_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("love AND strong:G26").unwrap();
        match &query_chirho.root_chirho {
            QueryNodeChirho::AndChirho(children_chirho) => {
                assert_eq!(children_chirho.len(), 2);
                assert!(matches!(children_chirho[1], QueryNodeChirho::StrongChirho { .. }));
            }
            other_chirho => panic!("Expected AndChirho, got {:?}", other_chirho),
        }
    }

    #[test]
    fn test_or_multiple_terms_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("love OR charity OR grace").unwrap();
        match &query_chirho.root_chirho {
            QueryNodeChirho::OrChirho(children_chirho) => {
                assert!(children_chirho.len() >= 2);
            }
            other_chirho => panic!("Expected OrChirho, got {:?}", other_chirho),
        }
    }

    #[test]
    fn test_inline_not_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("love NOT hate").unwrap();
        match &query_chirho.root_chirho {
            QueryNodeChirho::AndChirho(children_chirho) => {
                assert_eq!(children_chirho.len(), 2);
                assert!(matches!(children_chirho[0], QueryNodeChirho::TermChirho { .. }));
                assert!(matches!(children_chirho[1], QueryNodeChirho::NotChirho(_)));
            }
            other_chirho => panic!("Expected AndChirho with NOT child, got {:?}", other_chirho),
        }
    }

    #[test]
    fn test_inline_not_multi_word_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("grace mercy NOT sin").unwrap();
        match &query_chirho.root_chirho {
            QueryNodeChirho::AndChirho(children_chirho) => {
                assert_eq!(children_chirho.len(), 3);
                assert!(matches!(children_chirho[0], QueryNodeChirho::TermChirho { .. }));
                assert!(matches!(children_chirho[1], QueryNodeChirho::TermChirho { .. }));
                assert!(matches!(children_chirho[2], QueryNodeChirho::NotChirho(_)));
            }
            other_chirho => panic!("Expected AndChirho with NOT child, got {:?}", other_chirho),
        }
    }

    #[test]
    fn test_max_results_default_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("love").unwrap();
        assert_eq!(query_chirho.max_results_chirho, 100);
    }

    #[test]
    fn test_explain_default_false_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("love").unwrap();
        assert!(!query_chirho.explain_chirho);
    }

    #[test]
    fn test_query_with_max_results_chirho() {
        let mut query_chirho = QueryParserChirho::parse_chirho("love").unwrap();
        query_chirho = query_chirho.with_max_results_chirho(10);
        assert_eq!(query_chirho.max_results_chirho, 10);
    }

    #[test]
    fn test_query_with_explain_chirho() {
        let mut query_chirho = QueryParserChirho::parse_chirho("love").unwrap();
        query_chirho = query_chirho.with_explain_chirho();
        assert!(query_chirho.explain_chirho);
    }

    #[test]
    fn test_proximity_near_1_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("God NEAR/1 loved").unwrap();
        match &query_chirho.root_chirho {
            QueryNodeChirho::ProximityChirho {
                distance_chirho, ..
            } => {
                assert_eq!(*distance_chirho, 1);
            }
            other_chirho => panic!("Expected ProximityChirho, got {:?}", other_chirho),
        }
    }

    #[test]
    fn test_long_phrase_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho(
            "\"the Lord is my shepherd I shall not want\""
        ).unwrap();
        match &query_chirho.root_chirho {
            QueryNodeChirho::PhraseChirho { words_chirho, .. } => {
                assert_eq!(words_chirho.len(), 9);
            }
            other_chirho => panic!("Expected PhraseChirho, got {:?}", other_chirho),
        }
    }

    // ── Phase 2: Semantic / AI prefix tests ─────────────────────────

    #[test]
    fn test_semantic_prefix_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("semantic:God's faithfulness").unwrap();
        match &query_chirho.root_chirho {
            QueryNodeChirho::SemanticChirho {
                query_text_chirho, top_k_chirho, ..
            } => {
                assert_eq!(query_text_chirho, "God's faithfulness");
                assert_eq!(*top_k_chirho, 20);
            }
            other_chirho => panic!("Expected SemanticChirho, got {:?}", other_chirho),
        }
    }

    #[test]
    fn test_tilde_semantic_prefix_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("~divine love").unwrap();
        match &query_chirho.root_chirho {
            QueryNodeChirho::SemanticChirho {
                query_text_chirho, ..
            } => {
                assert_eq!(query_text_chirho, "divine love");
            }
            other_chirho => panic!("Expected SemanticChirho, got {:?}", other_chirho),
        }
    }

    #[test]
    fn test_concept_prefix_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("concept:redemption").unwrap();
        match &query_chirho.root_chirho {
            QueryNodeChirho::ConceptChirho {
                concept_chirho,
                include_related_chirho,
            } => {
                assert_eq!(concept_chirho, "redemption");
                assert!(*include_related_chirho);
            }
            other_chirho => panic!("Expected ConceptChirho, got {:?}", other_chirho),
        }
    }

    #[test]
    fn test_expand_prefix_chirho() {
        let query_chirho =
            QueryParserChirho::parse_chirho("expand:covenant faithfulness").unwrap();
        match &query_chirho.root_chirho {
            QueryNodeChirho::ExpandedChirho {
                original_query_chirho,
                ..
            } => {
                assert_eq!(original_query_chirho, "covenant faithfulness");
            }
            other_chirho => panic!("Expected ExpandedChirho, got {:?}", other_chirho),
        }
    }

    // ── Phase 2: Discourse prefix tests ─────────────────────────────

    #[test]
    fn test_rel_prefix_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("rel:Ground").unwrap();
        match &query_chirho.root_chirho {
            QueryNodeChirho::DiscourseRelationshipChirho {
                relationship_type_chirho,
                ..
            } => {
                assert_eq!(relationship_type_chirho, "Ground");
            }
            other_chirho => panic!("Expected DiscourseRelationshipChirho, got {:?}", other_chirho),
        }
    }

    #[test]
    fn test_prop_prefix_chirho() {
        let query_chirho =
            QueryParserChirho::parse_chirho("prop:\"resurrection\"").unwrap();
        match &query_chirho.root_chirho {
            QueryNodeChirho::PropositionTextChirho { text_chirho, .. } => {
                assert_eq!(text_chirho, "resurrection");
            }
            other_chirho => panic!("Expected PropositionTextChirho, got {:?}", other_chirho),
        }
    }

    // ── Phase 5+: Domain / Sense / Syntax prefix tests ──────────

    #[test]
    fn test_domain_prefix_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("domain:love").unwrap();
        match &query_chirho.root_chirho {
            QueryNodeChirho::DomainChirho { domain_chirho } => {
                assert_eq!(domain_chirho, "love");
            }
            other_chirho => panic!("Expected DomainChirho, got {:?}", other_chirho),
        }
    }

    #[test]
    fn test_sense_prefix_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("sense:love.01").unwrap();
        match &query_chirho.root_chirho {
            QueryNodeChirho::SenseChirho { sense_chirho } => {
                assert_eq!(sense_chirho, "love.01");
            }
            other_chirho => panic!("Expected SenseChirho, got {:?}", other_chirho),
        }
    }

    #[test]
    fn test_syntax_prefix_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("syntax:relative").unwrap();
        match &query_chirho.root_chirho {
            QueryNodeChirho::SyntaxChirho {
                clause_type_chirho, ..
            } => {
                assert_eq!(clause_type_chirho, "relative");
            }
            other_chirho => panic!("Expected SyntaxChirho, got {:?}", other_chirho),
        }
    }

    #[test]
    fn test_clause_prefix_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("clause:conditional").unwrap();
        match &query_chirho.root_chirho {
            QueryNodeChirho::SyntaxChirho {
                clause_type_chirho, ..
            } => {
                assert_eq!(clause_type_chirho, "conditional");
            }
            other_chirho => panic!("Expected SyntaxChirho, got {:?}", other_chirho),
        }
    }

    #[test]
    fn test_domain_with_and_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("domain:love AND love").unwrap();
        match &query_chirho.root_chirho {
            QueryNodeChirho::AndChirho(children_chirho) => {
                assert_eq!(children_chirho.len(), 2);
                assert!(matches!(children_chirho[0], QueryNodeChirho::DomainChirho { .. }));
            }
            other_chirho => panic!("Expected AndChirho, got {:?}", other_chirho),
        }
    }

    // ── Phase 3: Morphology prefix tests ─────────────────────────

    #[test]
    fn test_morph_full_code_prefix_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("morph:V-AAI-3S").unwrap();
        match &query_chirho.root_chirho {
            QueryNodeChirho::MorphChirho(constraint_chirho) => {
                assert_eq!(
                    constraint_chirho.part_of_speech_chirho,
                    Some(rhema_contracts_chirho::morphology_chirho::PartOfSpeechChirho::VerbChirho)
                );
                assert_eq!(
                    constraint_chirho.tense_chirho,
                    Some(rhema_contracts_chirho::morphology_chirho::TenseChirho::AoristChirho)
                );
            }
            other_chirho => panic!("Expected MorphChirho, got {:?}", other_chirho),
        }
    }

    #[test]
    fn test_pos_prefix_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("pos:verb").unwrap();
        match &query_chirho.root_chirho {
            QueryNodeChirho::MorphChirho(constraint_chirho) => {
                assert_eq!(
                    constraint_chirho.part_of_speech_chirho,
                    Some(rhema_contracts_chirho::morphology_chirho::PartOfSpeechChirho::VerbChirho)
                );
            }
            other_chirho => panic!("Expected MorphChirho, got {:?}", other_chirho),
        }
    }

    #[test]
    fn test_tense_prefix_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("tense:aorist").unwrap();
        match &query_chirho.root_chirho {
            QueryNodeChirho::MorphChirho(constraint_chirho) => {
                assert_eq!(
                    constraint_chirho.tense_chirho,
                    Some(rhema_contracts_chirho::morphology_chirho::TenseChirho::AoristChirho)
                );
            }
            other_chirho => panic!("Expected MorphChirho, got {:?}", other_chirho),
        }
    }

    #[test]
    fn test_voice_prefix_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("voice:active").unwrap();
        match &query_chirho.root_chirho {
            QueryNodeChirho::MorphChirho(constraint_chirho) => {
                assert_eq!(
                    constraint_chirho.voice_chirho,
                    Some(rhema_contracts_chirho::morphology_chirho::VoiceChirho::ActiveChirho)
                );
            }
            other_chirho => panic!("Expected MorphChirho, got {:?}", other_chirho),
        }
    }

    #[test]
    fn test_mood_prefix_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("mood:indicative").unwrap();
        match &query_chirho.root_chirho {
            QueryNodeChirho::MorphChirho(constraint_chirho) => {
                assert_eq!(
                    constraint_chirho.mood_chirho,
                    Some(rhema_contracts_chirho::morphology_chirho::MoodChirho::IndicativeChirho)
                );
            }
            other_chirho => panic!("Expected MorphChirho, got {:?}", other_chirho),
        }
    }

    #[test]
    fn test_case_prefix_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("case:genitive").unwrap();
        match &query_chirho.root_chirho {
            QueryNodeChirho::MorphChirho(constraint_chirho) => {
                assert_eq!(
                    constraint_chirho.case_chirho,
                    Some(rhema_contracts_chirho::morphology_chirho::CaseChirho::GenitiveChirho)
                );
            }
            other_chirho => panic!("Expected MorphChirho, got {:?}", other_chirho),
        }
    }

    #[test]
    fn test_number_prefix_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("number:singular").unwrap();
        match &query_chirho.root_chirho {
            QueryNodeChirho::MorphChirho(constraint_chirho) => {
                assert_eq!(
                    constraint_chirho.number_chirho,
                    Some(rhema_contracts_chirho::morphology_chirho::GrammaticalNumberChirho::SingularChirho)
                );
            }
            other_chirho => panic!("Expected MorphChirho, got {:?}", other_chirho),
        }
    }

    #[test]
    fn test_gender_prefix_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("gender:masculine").unwrap();
        match &query_chirho.root_chirho {
            QueryNodeChirho::MorphChirho(constraint_chirho) => {
                assert_eq!(
                    constraint_chirho.gender_chirho,
                    Some(rhema_contracts_chirho::morphology_chirho::GenderChirho::MasculineChirho)
                );
            }
            other_chirho => panic!("Expected MorphChirho, got {:?}", other_chirho),
        }
    }

    #[test]
    fn test_person_prefix_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("person:3").unwrap();
        match &query_chirho.root_chirho {
            QueryNodeChirho::MorphChirho(constraint_chirho) => {
                assert_eq!(
                    constraint_chirho.person_chirho,
                    Some(rhema_contracts_chirho::morphology_chirho::PersonChirho::ThirdChirho)
                );
            }
            other_chirho => panic!("Expected MorphChirho, got {:?}", other_chirho),
        }
    }

    #[test]
    fn test_invalid_morph_prefix_chirho() {
        let result_chirho = QueryParserChirho::parse_chirho("morph:ZZZZZ");
        assert!(result_chirho.is_err());
    }

    #[test]
    fn test_invalid_pos_prefix_chirho() {
        let result_chirho = QueryParserChirho::parse_chirho("pos:xylophone");
        assert!(result_chirho.is_err());
    }

    #[test]
    fn test_morph_and_term_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("pos:verb AND love").unwrap();
        match &query_chirho.root_chirho {
            QueryNodeChirho::AndChirho(children_chirho) => {
                assert_eq!(children_chirho.len(), 2);
                assert!(matches!(children_chirho[0], QueryNodeChirho::MorphChirho(_)));
                assert!(matches!(children_chirho[1], QueryNodeChirho::TermChirho { .. }));
            }
            other_chirho => panic!("Expected AndChirho, got {:?}", other_chirho),
        }
    }

    #[test]
    fn test_morph_hebrew_code_prefix_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("morph:HNcmsa").unwrap();
        match &query_chirho.root_chirho {
            QueryNodeChirho::MorphChirho(constraint_chirho) => {
                assert_eq!(
                    constraint_chirho.part_of_speech_chirho,
                    Some(rhema_contracts_chirho::morphology_chirho::PartOfSpeechChirho::NounChirho)
                );
            }
            other_chirho => panic!("Expected MorphChirho, got {:?}", other_chirho),
        }
    }

    #[test]
    fn test_morph_noun_prefix_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("morph:N-GSF").unwrap();
        match &query_chirho.root_chirho {
            QueryNodeChirho::MorphChirho(constraint_chirho) => {
                assert_eq!(
                    constraint_chirho.part_of_speech_chirho,
                    Some(rhema_contracts_chirho::morphology_chirho::PartOfSpeechChirho::NounChirho)
                );
                assert_eq!(
                    constraint_chirho.case_chirho,
                    Some(rhema_contracts_chirho::morphology_chirho::CaseChirho::GenitiveChirho)
                );
            }
            other_chirho => panic!("Expected MorphChirho, got {:?}", other_chirho),
        }
    }

    // ── Phase 4: Cross-reference prefix tests ───────────────────

    #[test]
    fn test_xref_prefix_osis_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("xref:John.3.16").unwrap();
        match &query_chirho.root_chirho {
            QueryNodeChirho::GraphExpandChirho {
                seed_chirho,
                depth_chirho,
            } => {
                assert_eq!(seed_chirho, "John.3.16");
                assert_eq!(*depth_chirho, 1);
            }
            other_chirho => panic!("Expected GraphExpandChirho, got {:?}", other_chirho),
        }
    }

    #[test]
    fn test_xref_prefix_human_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("xref:John 3:16").unwrap();
        match &query_chirho.root_chirho {
            QueryNodeChirho::GraphExpandChirho {
                seed_chirho,
                depth_chirho,
            } => {
                assert_eq!(seed_chirho, "John 3:16");
                assert_eq!(*depth_chirho, 1);
            }
            other_chirho => panic!("Expected GraphExpandChirho, got {:?}", other_chirho),
        }
    }

    #[test]
    fn test_xref_operator_depth_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("XREF/2 John 3:16").unwrap();
        match &query_chirho.root_chirho {
            QueryNodeChirho::GraphExpandChirho {
                seed_chirho,
                depth_chirho,
            } => {
                assert_eq!(seed_chirho, "John 3:16");
                assert_eq!(*depth_chirho, 2);
            }
            other_chirho => panic!("Expected GraphExpandChirho, got {:?}", other_chirho),
        }
    }

    #[test]
    fn test_xref_operator_depth_3_chirho() {
        let query_chirho = QueryParserChirho::parse_chirho("XREF/3 Gen.1.1").unwrap();
        match &query_chirho.root_chirho {
            QueryNodeChirho::GraphExpandChirho {
                seed_chirho,
                depth_chirho,
            } => {
                assert_eq!(seed_chirho, "Gen.1.1");
                assert_eq!(*depth_chirho, 3);
            }
            other_chirho => panic!("Expected GraphExpandChirho, got {:?}", other_chirho),
        }
    }
}
