// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! REST API handlers.

use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};

use rhema_integration_chirho::RhemaLibraryChirho;

use crate::error_chirho::ApiErrorChirho;

/// Shared application state.
pub type AppStateChirho = Arc<RhemaLibraryChirho>;

/// Search query parameters.
#[derive(Debug, Deserialize)]
pub struct SearchParamsChirho {
    /// The query string.
    pub q_chirho: String,
    /// Module name.
    pub module_chirho: Option<String>,
    /// Maximum results.
    pub max_chirho: Option<usize>,
}

/// Lookup query parameters.
#[derive(Debug, Deserialize)]
pub struct LookupParamsChirho {
    /// Book name.
    pub book_chirho: String,
    /// Chapter number.
    pub chapter_chirho: u16,
    /// Verse number (optional — omit for full chapter).
    pub verse_chirho: Option<u16>,
    /// Module name.
    pub module_chirho: Option<String>,
}

/// Search result DTO.
#[derive(Debug, Serialize)]
pub struct SearchResponseChirho {
    pub query_chirho: String,
    pub total_chirho: usize,
    pub hits_chirho: Vec<HitDtoChirho>,
}

/// Single hit DTO.
#[derive(Debug, Serialize)]
pub struct HitDtoChirho {
    pub reference_chirho: String,
    pub text_chirho: String,
    pub score_chirho: f32,
}

/// Module info DTO.
#[derive(Debug, Serialize)]
pub struct ModuleDtoChirho {
    pub name_chirho: String,
    pub module_type_chirho: String,
    pub language_chirho: String,
    pub description_chirho: String,
}

/// GET /api-chirho/search-chirho — execute a search query.
pub async fn handle_search_chirho(
    State(lib_chirho): State<AppStateChirho>,
    Query(params_chirho): Query<SearchParamsChirho>,
) -> Result<Json<SearchResponseChirho>, ApiErrorChirho> {
    let query_chirho = rhema_query_chirho::QueryParserChirho::parse_chirho(&params_chirho.q_chirho)
        .map_err(|e_chirho| ApiErrorChirho::BadRequestChirho {
            message_chirho: e_chirho.to_string(),
        })?;

    let _ = lib_chirho.as_ref();
    let _ = params_chirho.module_chirho;
    let _ = params_chirho.max_chirho;

    // Return parsed query info (actual execution requires the executor crate)
    Ok(Json(SearchResponseChirho {
        query_chirho: format!("{:?}", query_chirho.root_chirho),
        total_chirho: 0,
        hits_chirho: Vec::new(),
    }))
}

/// GET /api-chirho/lookup-chirho — look up a verse or chapter.
pub async fn handle_lookup_chirho(
    State(lib_chirho): State<AppStateChirho>,
    Query(params_chirho): Query<LookupParamsChirho>,
) -> Result<Json<serde_json::Value>, ApiErrorChirho> {
    let module_chirho = params_chirho.module_chirho.as_deref().unwrap_or("KJV");

    if let Some(v_chirho) = params_chirho.verse_chirho {
        let result_chirho = lib_chirho
            .read_verse_chirho(
                module_chirho,
                &params_chirho.book_chirho,
                params_chirho.chapter_chirho,
                v_chirho,
            )
            .map_err(|e_chirho| ApiErrorChirho::NotFoundChirho {
                message_chirho: e_chirho.to_string(),
            })?;
        Ok(Json(serde_json::json!({
            "book_chirho": result_chirho.book_chirho,
            "chapter_chirho": result_chirho.chapter_chirho,
            "verse_chirho": result_chirho.verse_chirho,
            "text_chirho": result_chirho.text_chirho,
        })))
    } else {
        let chapter_chirho = lib_chirho
            .read_chapter_chirho(
                module_chirho,
                &params_chirho.book_chirho,
                params_chirho.chapter_chirho,
            )
            .map_err(|e_chirho| ApiErrorChirho::NotFoundChirho {
                message_chirho: e_chirho.to_string(),
            })?;
        Ok(Json(serde_json::json!({
            "book_chirho": chapter_chirho.book_chirho,
            "chapter_chirho": chapter_chirho.chapter_chirho,
            "verses_chirho": chapter_chirho.verses_chirho.len(),
        })))
    }
}

/// GET /api-chirho/modules-chirho — list available modules.
pub async fn handle_modules_chirho(
    State(lib_chirho): State<AppStateChirho>,
) -> Json<Vec<ModuleDtoChirho>> {
    let modules_chirho = lib_chirho.list_all_modules_chirho();
    let dtos_chirho: Vec<ModuleDtoChirho> = modules_chirho
        .iter()
        .map(|m_chirho| ModuleDtoChirho {
            name_chirho: m_chirho.name_chirho.clone(),
            module_type_chirho: m_chirho.module_type_chirho.clone(),
            language_chirho: m_chirho.language_chirho.clone(),
            description_chirho: m_chirho.description_chirho.clone(),
        })
        .collect();
    Json(dtos_chirho)
}

/// GET /api-chirho/info-chirho/:module — get module info.
pub async fn handle_module_info_chirho(
    State(lib_chirho): State<AppStateChirho>,
    Path(module_name_chirho): Path<String>,
) -> Result<Json<ModuleDtoChirho>, ApiErrorChirho> {
    let info_chirho = lib_chirho
        .get_module_info_chirho(&module_name_chirho)
        .ok_or_else(|| ApiErrorChirho::NotFoundChirho {
            message_chirho: format!("Module '{}' not found", module_name_chirho),
        })?;

    Ok(Json(ModuleDtoChirho {
        name_chirho: info_chirho.name_chirho.clone(),
        module_type_chirho: info_chirho.module_type_chirho.clone(),
        language_chirho: info_chirho.language_chirho.clone(),
        description_chirho: info_chirho.description_chirho.clone(),
    }))
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_search_params_deserialize_chirho() {
        let json_chirho = r#"{"q_chirho": "love AND world"}"#;
        let params_chirho: SearchParamsChirho = serde_json::from_str(json_chirho).unwrap();
        assert_eq!(params_chirho.q_chirho, "love AND world");
        assert!(params_chirho.module_chirho.is_none());
    }

    #[test]
    fn test_search_response_serialize_chirho() {
        let response_chirho = SearchResponseChirho {
            query_chirho: "love".to_string(),
            total_chirho: 2,
            hits_chirho: vec![
                HitDtoChirho {
                    reference_chirho: "John 3:16".to_string(),
                    text_chirho: "For God so loved...".to_string(),
                    score_chirho: 1.0,
                },
            ],
        };
        let json_chirho = serde_json::to_string(&response_chirho).unwrap();
        assert!(json_chirho.contains("John 3:16"));
    }

    #[test]
    fn test_module_dto_serialize_chirho() {
        let dto_chirho = ModuleDtoChirho {
            name_chirho: "KJV".to_string(),
            module_type_chirho: "Bible".to_string(),
            language_chirho: "en".to_string(),
            description_chirho: "King James Version".to_string(),
        };
        let json_chirho = serde_json::to_string(&dto_chirho).unwrap();
        assert!(json_chirho.contains("KJV"));
    }

    #[test]
    fn test_lookup_params_deserialize_chirho() {
        let json_chirho = r#"{"book_chirho": "John", "chapter_chirho": 3, "verse_chirho": 16}"#;
        let params_chirho: LookupParamsChirho = serde_json::from_str(json_chirho).unwrap();
        assert_eq!(params_chirho.book_chirho, "John");
        assert_eq!(params_chirho.chapter_chirho, 3);
        assert_eq!(params_chirho.verse_chirho, Some(16));
    }

    #[test]
    fn test_hit_dto_serialize_chirho() {
        let hit_chirho = HitDtoChirho {
            reference_chirho: "Romans 8:28".to_string(),
            text_chirho: "And we know...".to_string(),
            score_chirho: 0.95,
        };
        let json_chirho = serde_json::to_string(&hit_chirho).unwrap();
        assert!(json_chirho.contains("0.95"));
    }
}
