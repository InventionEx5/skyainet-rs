// crates/api/src/mod.rs
// =====================================================
// SkyAInet API Module
// REST + GraphQL + WebSocket APIs
// =====================================================

pub mod rest;
pub mod graphql;
pub mod websocket;

// =====================================================
// RÉ-EXPORTS PUBLICS
// =====================================================

pub use rest::create_rest_router;
pub use graphql::{create_schema, create_graphql_router, AppSchema};
pub use websocket::create_websocket_router;

// =====================================================
// VERSION DU MODULE
// =====================================================

pub const MODULE_VERSION: &str = "6.7.0";