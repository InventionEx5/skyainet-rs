// crates/api/src/lib.rs
// =====================================================
// SkyAInet API Crate v6.7
// REST + GraphQL + WebSocket APIs
// =====================================================

pub mod rest;
pub mod graphql;
pub mod websocket;

pub use rest::create_rest_router;
pub use graphql::{create_schema, create_graphql_router, AppSchema};
pub use websocket::create_websocket_router;

// =====================================================
// TYPES PARTAGÉS
// =====================================================

pub type SharedThevie = std::sync::Arc<tokio::sync::Mutex<skyainet_model::Thevie>>;
pub type SharedNode = std::sync::Arc<tokio::sync::Mutex<skyainet_node::SkyAInetNode>>;
pub type SharedRewards = std::sync::Arc<tokio::sync::Mutex<skyainet_core::rewards::UserRewards>>;

// =====================================================
// VERSION DU CRATE
// =====================================================

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

pub fn version_info() -> String {
    format!("{} v{}", CRATE_NAME, VERSION)
}

// =====================================================
// FONCTION D'INITIALISATION
// =====================================================

/// Crée un router complet (REST + GraphQL + WebSocket)
pub fn create_full_api_router(
    thevie: SharedThevie,
    node: SharedNode,
    rewards: SharedRewards,
) -> axum::Router {
    let graphql_schema = graphql::create_schema(thevie.clone(), node.clone(), rewards.clone());

    axum::Router::new()
        .merge(rest::create_rest_router((thevie.clone(), node.clone(), rewards.clone())))
        .merge(graphql::create_graphql_router(graphql_schema))
        .merge(websocket::create_websocket_router((thevie, node, rewards)))
}