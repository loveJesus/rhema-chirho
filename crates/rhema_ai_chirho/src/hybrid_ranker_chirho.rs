// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Hybrid ranker — merges keyword and semantic search results.

use std::collections::HashMap;

use rhema_contracts_chirho::result_chirho::SearchHitChirho;

/// Strategy for merging keyword and semantic results.
#[derive(Debug, Clone, Copy)]
pub enum MergeStrategyChirho {
    /// Weighted linear combination of scores.
    WeightedChirho,
    /// Reciprocal Rank Fusion.
    RrfChirho,
}

/// Merge keyword and semantic search results into a unified ranking.
pub fn merge_results_chirho(
    keyword_hits_chirho: &[SearchHitChirho],
    semantic_hits_chirho: &[SearchHitChirho],
    keyword_weight_chirho: f32,
    strategy_chirho: MergeStrategyChirho,
) -> Vec<SearchHitChirho> {
    match strategy_chirho {
        MergeStrategyChirho::WeightedChirho => {
            merge_weighted_chirho(keyword_hits_chirho, semantic_hits_chirho, keyword_weight_chirho)
        }
        MergeStrategyChirho::RrfChirho => {
            merge_rrf_chirho(keyword_hits_chirho, semantic_hits_chirho)
        }
    }
}

/// Weighted linear combination of keyword and semantic scores.
fn merge_weighted_chirho(
    keyword_hits_chirho: &[SearchHitChirho],
    semantic_hits_chirho: &[SearchHitChirho],
    keyword_weight_chirho: f32,
) -> Vec<SearchHitChirho> {
    let semantic_weight_chirho = 1.0 - keyword_weight_chirho;
    let mut merged_chirho: HashMap<String, SearchHitChirho> = HashMap::new();

    // Add keyword hits.
    for hit_chirho in keyword_hits_chirho {
        let key_chirho = hit_chirho.verse_ref_chirho.to_string();
        let mut merged_hit_chirho = hit_chirho.clone();
        merged_hit_chirho.hybrid_score_chirho =
            Some(hit_chirho.score_chirho * keyword_weight_chirho);
        merged_chirho.insert(key_chirho, merged_hit_chirho);
    }

    // Merge semantic hits.
    for hit_chirho in semantic_hits_chirho {
        let key_chirho = hit_chirho.verse_ref_chirho.to_string();
        let semantic_contribution_chirho =
            hit_chirho.semantic_score_chirho.unwrap_or(hit_chirho.score_chirho)
                * semantic_weight_chirho;

        if let Some(existing_chirho) = merged_chirho.get_mut(&key_chirho) {
            let current_hybrid_chirho =
                existing_chirho.hybrid_score_chirho.unwrap_or(0.0);
            existing_chirho.hybrid_score_chirho =
                Some(current_hybrid_chirho + semantic_contribution_chirho);
            existing_chirho.semantic_score_chirho = hit_chirho.semantic_score_chirho;
            if existing_chirho.semantic_explain_chirho.is_none() {
                existing_chirho.semantic_explain_chirho =
                    hit_chirho.semantic_explain_chirho.clone();
            }
        } else {
            let mut merged_hit_chirho = hit_chirho.clone();
            merged_hit_chirho.hybrid_score_chirho = Some(semantic_contribution_chirho);
            merged_chirho.insert(key_chirho, merged_hit_chirho);
        }
    }

    let mut results_chirho: Vec<SearchHitChirho> = merged_chirho.into_values().collect();
    results_chirho.sort_by(|a_chirho, b_chirho| {
        let sa_chirho = a_chirho.hybrid_score_chirho.unwrap_or(0.0);
        let sb_chirho = b_chirho.hybrid_score_chirho.unwrap_or(0.0);
        sb_chirho
            .partial_cmp(&sa_chirho)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    results_chirho
}

/// Reciprocal Rank Fusion (RRF) merge.
///
/// RRF score = sum(1 / (k + rank)) across all result lists.
/// Uses k=60 as is standard.
fn merge_rrf_chirho(
    keyword_hits_chirho: &[SearchHitChirho],
    semantic_hits_chirho: &[SearchHitChirho],
) -> Vec<SearchHitChirho> {
    const K_CHIRHO: f32 = 60.0;
    let mut rrf_scores_chirho: HashMap<String, (f32, SearchHitChirho)> = HashMap::new();

    // Score from keyword ranking.
    for (rank_chirho, hit_chirho) in keyword_hits_chirho.iter().enumerate() {
        let key_chirho = hit_chirho.verse_ref_chirho.to_string();
        let rrf_chirho = 1.0 / (K_CHIRHO + rank_chirho as f32 + 1.0);
        let entry_chirho = rrf_scores_chirho
            .entry(key_chirho)
            .or_insert((0.0, hit_chirho.clone()));
        entry_chirho.0 += rrf_chirho;
    }

    // Score from semantic ranking.
    for (rank_chirho, hit_chirho) in semantic_hits_chirho.iter().enumerate() {
        let key_chirho = hit_chirho.verse_ref_chirho.to_string();
        let rrf_chirho = 1.0 / (K_CHIRHO + rank_chirho as f32 + 1.0);
        let entry_chirho = rrf_scores_chirho
            .entry(key_chirho)
            .or_insert((0.0, hit_chirho.clone()));
        entry_chirho.0 += rrf_chirho;
        entry_chirho.1.semantic_score_chirho = hit_chirho.semantic_score_chirho;
    }

    let mut results_chirho: Vec<SearchHitChirho> = rrf_scores_chirho
        .into_values()
        .map(|(score_chirho, mut hit_chirho)| {
            hit_chirho.hybrid_score_chirho = Some(score_chirho);
            hit_chirho
        })
        .collect();

    results_chirho.sort_by(|a_chirho, b_chirho| {
        let sa_chirho = a_chirho.hybrid_score_chirho.unwrap_or(0.0);
        let sb_chirho = b_chirho.hybrid_score_chirho.unwrap_or(0.0);
        sb_chirho
            .partial_cmp(&sa_chirho)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    results_chirho
}

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use rhema_contracts_chirho::ids_chirho::TokenPositionChirho;
    use rhema_contracts_chirho::keys_chirho::VerseRefChirho;

    fn make_hit_chirho(
        book_chirho: &str,
        ch_chirho: u16,
        v_chirho: u16,
        score_chirho: f32,
    ) -> SearchHitChirho {
        SearchHitChirho {
            verse_ref_chirho: VerseRefChirho::new_chirho(book_chirho, ch_chirho, v_chirho).unwrap(),
            text_chirho: format!("{} {}:{}", book_chirho, ch_chirho, v_chirho),
            highlighted_text_chirho: None,
            score_chirho,
            matched_positions_chirho: Vec::new(),
            explain_chirho: None,
            semantic_score_chirho: None,
            hybrid_score_chirho: None,
            semantic_explain_chirho: None,
        }
    }

    fn make_semantic_hit_chirho(
        book_chirho: &str,
        ch_chirho: u16,
        v_chirho: u16,
        semantic_score_chirho: f32,
    ) -> SearchHitChirho {
        let mut hit_chirho = make_hit_chirho(book_chirho, ch_chirho, v_chirho, 0.0);
        hit_chirho.semantic_score_chirho = Some(semantic_score_chirho);
        hit_chirho
    }

    #[test]
    fn test_weighted_merge_chirho() {
        let keyword_chirho = vec![
            make_hit_chirho("John", 3, 16, 1.0),
            make_hit_chirho("Rom", 8, 28, 0.8),
        ];
        let semantic_chirho = vec![
            make_semantic_hit_chirho("John", 3, 16, 0.95),
            make_semantic_hit_chirho("Ps", 23, 1, 0.90),
        ];

        let results_chirho = merge_results_chirho(
            &keyword_chirho,
            &semantic_chirho,
            0.7,
            MergeStrategyChirho::WeightedChirho,
        );

        assert!(!results_chirho.is_empty());
        // John 3:16 appears in both, should have highest hybrid score.
        assert_eq!(results_chirho[0].verse_ref_chirho.book_chirho, "John");
        assert!(results_chirho[0].hybrid_score_chirho.unwrap() > 0.0);
    }

    #[test]
    fn test_rrf_merge_chirho() {
        let keyword_chirho = vec![
            make_hit_chirho("John", 3, 16, 1.0),
            make_hit_chirho("Rom", 8, 28, 0.8),
        ];
        let semantic_chirho = vec![
            make_semantic_hit_chirho("John", 3, 16, 0.95),
            make_semantic_hit_chirho("Gen", 1, 1, 0.85),
        ];

        let results_chirho = merge_results_chirho(
            &keyword_chirho,
            &semantic_chirho,
            0.5,
            MergeStrategyChirho::RrfChirho,
        );

        assert!(!results_chirho.is_empty());
        // John 3:16 appears in both lists, should rank highest.
        assert_eq!(results_chirho[0].verse_ref_chirho.book_chirho, "John");
    }

    #[test]
    fn test_merge_empty_lists_chirho() {
        let results_chirho = merge_results_chirho(
            &[],
            &[],
            0.5,
            MergeStrategyChirho::WeightedChirho,
        );
        assert!(results_chirho.is_empty());
    }

    #[test]
    fn test_merge_keyword_only_chirho() {
        let keyword_chirho = vec![make_hit_chirho("John", 3, 16, 1.0)];
        let results_chirho = merge_results_chirho(
            &keyword_chirho,
            &[],
            0.7,
            MergeStrategyChirho::WeightedChirho,
        );
        assert_eq!(results_chirho.len(), 1);
    }
}
