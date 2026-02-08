// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Golden test harness — compares actual engine output against known-good snapshots.
//!
//! The harness loads fixtures, runs them against the engine, and reports
//! pass/fail with detailed diffs on mismatch.

use crate::fixtures_chirho::{QueryFixtureChirho, VerseFixtureChirho};

/// Result of a single golden test.
#[derive(Debug)]
pub struct GoldenResultChirho {
    pub label_chirho: String,
    pub passed_chirho: bool,
    pub detail_chirho: String,
}

/// The golden test harness.
pub struct GoldenHarnessChirho {
    verse_results_chirho: Vec<GoldenResultChirho>,
    query_results_chirho: Vec<GoldenResultChirho>,
}

impl GoldenHarnessChirho {
    /// Create an empty harness.
    pub fn new_chirho() -> Self {
        Self {
            verse_results_chirho: Vec::new(),
            query_results_chirho: Vec::new(),
        }
    }

    /// Record a verse fixture test result.
    pub fn record_verse_chirho(
        &mut self,
        fixture_chirho: &VerseFixtureChirho,
        actual_text_chirho: Option<&str>,
    ) {
        let (passed_chirho, detail_chirho) = match actual_text_chirho {
            Some(text_chirho) => {
                if text_chirho.contains(&fixture_chirho.expected_substring_chirho) {
                    (true, "OK".to_string())
                } else {
                    (
                        false,
                        format!(
                            "Expected substring '{}' not found in: '{}'",
                            fixture_chirho.expected_substring_chirho,
                            if text_chirho.len() > 100 {
                                &text_chirho[..100]
                            } else {
                                text_chirho
                            }
                        ),
                    )
                }
            }
            None => (false, "No text returned".to_string()),
        };

        self.verse_results_chirho.push(GoldenResultChirho {
            label_chirho: fixture_chirho.label_chirho.clone(),
            passed_chirho,
            detail_chirho,
        });
    }

    /// Record a query fixture test result.
    pub fn record_query_chirho(
        &mut self,
        fixture_chirho: &QueryFixtureChirho,
        hit_count_chirho: u64,
        hit_refs_chirho: &[rhema_contracts_chirho::keys_chirho::VerseRefChirho],
    ) {
        let mut issues_chirho = Vec::new();

        if hit_count_chirho < fixture_chirho.min_hits_chirho {
            issues_chirho.push(format!(
                "Too few hits: {} < {}",
                hit_count_chirho, fixture_chirho.min_hits_chirho
            ));
        }

        if let Some(max_chirho) = fixture_chirho.max_hits_chirho {
            if hit_count_chirho > max_chirho {
                issues_chirho.push(format!(
                    "Too many hits: {} > {}",
                    hit_count_chirho, max_chirho
                ));
            }
        }

        for required_chirho in &fixture_chirho.must_contain_chirho {
            if !hit_refs_chirho.contains(required_chirho) {
                issues_chirho.push(format!(
                    "Missing required result: {}",
                    required_chirho
                ));
            }
        }

        let passed_chirho = issues_chirho.is_empty();
        let detail_chirho = if passed_chirho {
            format!("OK ({} hits)", hit_count_chirho)
        } else {
            issues_chirho.join("; ")
        };

        self.query_results_chirho.push(GoldenResultChirho {
            label_chirho: fixture_chirho.label_chirho.clone(),
            passed_chirho,
            detail_chirho,
        });
    }

    /// Get all verse test results.
    pub fn verse_results_chirho(&self) -> &[GoldenResultChirho] {
        &self.verse_results_chirho
    }

    /// Get all query test results.
    pub fn query_results_chirho(&self) -> &[GoldenResultChirho] {
        &self.query_results_chirho
    }

    /// Count total passed tests.
    pub fn passed_count_chirho(&self) -> usize {
        self.verse_results_chirho
            .iter()
            .chain(self.query_results_chirho.iter())
            .filter(|r_chirho| r_chirho.passed_chirho)
            .count()
    }

    /// Count total tests.
    pub fn total_count_chirho(&self) -> usize {
        self.verse_results_chirho.len() + self.query_results_chirho.len()
    }

    /// Generate a summary report.
    pub fn report_chirho(&self) -> String {
        let mut lines_chirho = Vec::new();
        lines_chirho.push(format!(
            "Golden Test Report: {}/{} passed",
            self.passed_count_chirho(),
            self.total_count_chirho()
        ));
        lines_chirho.push(String::new());

        if !self.verse_results_chirho.is_empty() {
            lines_chirho.push("=== Verse Fixtures ===".to_string());
            for result_chirho in &self.verse_results_chirho {
                let status_chirho = if result_chirho.passed_chirho {
                    "PASS"
                } else {
                    "FAIL"
                };
                lines_chirho.push(format!(
                    "  [{}] {} — {}",
                    status_chirho, result_chirho.label_chirho, result_chirho.detail_chirho
                ));
            }
            lines_chirho.push(String::new());
        }

        if !self.query_results_chirho.is_empty() {
            lines_chirho.push("=== Query Fixtures ===".to_string());
            for result_chirho in &self.query_results_chirho {
                let status_chirho = if result_chirho.passed_chirho {
                    "PASS"
                } else {
                    "FAIL"
                };
                lines_chirho.push(format!(
                    "  [{}] {} — {}",
                    status_chirho, result_chirho.label_chirho, result_chirho.detail_chirho
                ));
            }
        }

        lines_chirho.join("\n")
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use crate::fixtures_chirho::VerseFixtureChirho;
    use rhema_contracts_chirho::keys_chirho::VerseRefChirho;

    #[test]
    fn test_golden_harness_verse_pass_chirho() {
        let mut harness_chirho = GoldenHarnessChirho::new_chirho();
        let fixture_chirho = VerseFixtureChirho {
            label_chirho: "Test verse".to_string(),
            verse_ref_chirho: VerseRefChirho::new_chirho("John", 3, 16).unwrap(),
            expected_substring_chirho: "God so loved".to_string(),
            module_chirho: "KJV".to_string(),
        };

        harness_chirho.record_verse_chirho(
            &fixture_chirho,
            Some("For God so loved the world"),
        );

        assert_eq!(harness_chirho.passed_count_chirho(), 1);
        assert_eq!(harness_chirho.total_count_chirho(), 1);
    }

    #[test]
    fn test_golden_harness_verse_fail_chirho() {
        let mut harness_chirho = GoldenHarnessChirho::new_chirho();
        let fixture_chirho = VerseFixtureChirho {
            label_chirho: "Test verse".to_string(),
            verse_ref_chirho: VerseRefChirho::new_chirho("John", 3, 16).unwrap(),
            expected_substring_chirho: "God so loved".to_string(),
            module_chirho: "KJV".to_string(),
        };

        harness_chirho.record_verse_chirho(
            &fixture_chirho,
            Some("Wrong text here"),
        );

        assert_eq!(harness_chirho.passed_count_chirho(), 0);
        let report_chirho = harness_chirho.report_chirho();
        assert!(report_chirho.contains("FAIL"));
    }

    #[test]
    fn test_golden_harness_report_chirho() {
        let harness_chirho = GoldenHarnessChirho::new_chirho();
        let report_chirho = harness_chirho.report_chirho();
        assert!(report_chirho.contains("0/0 passed"));
    }
}
