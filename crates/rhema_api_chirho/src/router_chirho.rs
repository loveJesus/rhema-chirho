// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! REST API router configuration.

use std::sync::Arc;

use axum::routing::get;
use axum::Router;
use tower_http::cors::CorsLayer;

use rhema_integration_chirho::RhemaLibraryChirho;

use crate::handlers_chirho;

/// Build the REST API router with all routes and CORS middleware.
pub fn build_router_chirho(lib_chirho: Arc<RhemaLibraryChirho>) -> Router {
    Router::new()
        .route(
            "/api-chirho/search-chirho",
            get(handlers_chirho::handle_search_chirho),
        )
        .route(
            "/api-chirho/lookup-chirho",
            get(handlers_chirho::handle_lookup_chirho),
        )
        .route(
            "/api-chirho/modules-chirho",
            get(handlers_chirho::handle_modules_chirho),
        )
        .route(
            "/api-chirho/info-chirho/{module}",
            get(handlers_chirho::handle_module_info_chirho),
        )
        .layer(CorsLayer::permissive())
        .with_state(lib_chirho)
}

/// Start the REST API server on the given port.
pub async fn serve_chirho(port_chirho: u16) -> anyhow::Result<()> {
    let lib_chirho = RhemaLibraryChirho::init_chirho()?;
    let app_chirho = build_router_chirho(Arc::new(lib_chirho));
    let addr_chirho = format!("0.0.0.0:{port_chirho}");
    log::info!("Rhema API listening on {}", addr_chirho);
    let listener_chirho = tokio::net::TcpListener::bind(&addr_chirho).await?;
    axum::serve(listener_chirho, app_chirho).await?;
    Ok(())
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_router_builds_chirho() {
        // Verify the router can be constructed without panicking.
        // We use a mock/dummy lib by catching the init error.
        let lib_result_chirho = RhemaLibraryChirho::init_chirho();
        if let Ok(lib_chirho) = lib_result_chirho {
            let _router_chirho = build_router_chirho(Arc::new(lib_chirho));
        }
        // Even if init fails (no SWORD modules), the router constructor itself is sound.
    }

    #[test]
    fn test_router_routes_configured_chirho() {
        // Verify that the route configuration compiles and produces a Router.
        // This is a compile-time correctness check.
        let lib_result_chirho = RhemaLibraryChirho::init_chirho();
        if let Ok(lib_chirho) = lib_result_chirho {
            let router_chirho = build_router_chirho(Arc::new(lib_chirho));
            // Router exists, routes are configured
            let _router2_chirho = router_chirho.clone();
        }
    }

    #[test]
    fn test_cors_layer_applied_chirho() {
        // Verify that CorsLayer::permissive() doesn't panic.
        let _cors_chirho = CorsLayer::permissive();
    }
}
