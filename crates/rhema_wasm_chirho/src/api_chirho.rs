// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! WASM-safe API functions.
//!
//! These functions are pure Rust (no Tantivy, no file I/O, no exec) and
//! are safe to compile to `wasm32-unknown-unknown`.
//!
//! On WASM targets, `#[wasm_bindgen]` attributes are applied.
//! On native targets, they're plain functions usable for testing.

use rhema_query_chirho::QueryParserChirho;
use rhema_query_chirho::planner_chirho::QueryPlannerChirho;

/// Parse a query string and return the IR as JSON.
///
/// Returns a JSON object with `root_chirho`, `scope_chirho`, `max_results_chirho`,
/// and `explain_chirho` fields.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub fn parse_query_chirho(input_chirho: &str) -> Result<String, String> {
    let parsed_chirho = QueryParserChirho::parse_chirho(input_chirho)
        .map_err(|e_chirho| e_chirho.to_string())?;

    serde_json::to_string(&serde_json::json!({
        "root_chirho": format!("{:?}", parsed_chirho.root_chirho),
        "scope_chirho": format!("{:?}", parsed_chirho.scope_chirho),
        "max_results_chirho": parsed_chirho.max_results_chirho,
        "explain_chirho": parsed_chirho.explain_chirho,
    }))
    .map_err(|e_chirho| e_chirho.to_string())
}

/// Validate a query string — returns true if it parses successfully.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub fn validate_query_chirho(input_chirho: &str) -> bool {
    QueryParserChirho::parse_chirho(input_chirho).is_ok()
}

/// Parse a query and return the execution plan as JSON.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub fn query_to_plan_chirho(input_chirho: &str) -> Result<String, String> {
    let parsed_chirho = QueryParserChirho::parse_chirho(input_chirho)
        .map_err(|e_chirho| e_chirho.to_string())?;
    let plan_chirho = QueryPlannerChirho::plan_chirho(&parsed_chirho);

    serde_json::to_string(&serde_json::json!({
        "plan_chirho": format!("{:?}", plan_chirho.root_step_chirho),
        "estimated_cost_chirho": plan_chirho.estimated_cost_chirho,
        "backend_chirho": plan_chirho.backend_chirho,
    }))
    .map_err(|e_chirho| e_chirho.to_string())
}

/// Return a JSON array of all supported query prefixes.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub fn supported_prefixes_chirho() -> String {
    let prefixes_chirho = vec![
        "strong:", "lemma:", "semantic:", "~", "concept:", "expand:",
        "xref:", "rel:", "prop:", "morph:", "pos:", "tense:", "voice:",
        "mood:", "case:", "number:", "gender:", "person:", "domain:",
        "sense:", "syntax:", "clause:",
    ];
    serde_json::to_string(&prefixes_chirho).unwrap_or_else(|_| "[]".to_string())
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_parse_query_chirho() {
        let result_chirho = parse_query_chirho("love AND world").unwrap();
        assert!(result_chirho.contains("root_chirho"));
        assert!(result_chirho.contains("max_results_chirho"));
    }

    #[test]
    fn test_parse_query_error_chirho() {
        let result_chirho = parse_query_chirho("");
        assert!(result_chirho.is_err());
    }

    #[test]
    fn test_validate_query_chirho() {
        assert!(validate_query_chirho("love"));
        assert!(validate_query_chirho("strong:G26"));
        assert!(!validate_query_chirho(""));
    }

    #[test]
    fn test_query_to_plan_chirho() {
        let result_chirho = query_to_plan_chirho("love AND world").unwrap();
        assert!(result_chirho.contains("estimated_cost_chirho"));
        assert!(result_chirho.contains("backend_chirho"));
    }

    #[test]
    fn test_query_to_plan_error_chirho() {
        let result_chirho = query_to_plan_chirho("");
        assert!(result_chirho.is_err());
    }

    #[test]
    fn test_supported_prefixes_chirho() {
        let json_chirho = supported_prefixes_chirho();
        let prefixes_chirho: Vec<String> = serde_json::from_str(&json_chirho).unwrap();
        assert!(prefixes_chirho.contains(&"strong:".to_string()));
        assert!(prefixes_chirho.contains(&"domain:".to_string()));
        assert!(prefixes_chirho.len() >= 20);
    }

    #[test]
    fn test_parse_with_scope_chirho() {
        let result_chirho = parse_query_chirho("[John] love").unwrap();
        assert!(result_chirho.contains("scope_chirho"));
    }

    #[test]
    fn test_plan_strong_query_chirho() {
        let result_chirho = query_to_plan_chirho("strong:G26").unwrap();
        assert!(result_chirho.contains("StrongLookup"));
    }
}
