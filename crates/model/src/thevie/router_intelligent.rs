// crates/model/src/thevie/router_intelligent.rs
// =====================================================
// Intelligent Router v5.1 — Optimisation Extrême
// Compatible avec SkyNode v3.5 (generate_with_ai)
// 100% Indépendant • Ultra Rapide • Zéro Dépendance
// =====================================================

use super::neural_mesh::{Personality, Query};
use crate::node::skynode::{SkyNode, AIRequest};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct IntelligentRouter;

impl IntelligentRouter {
    pub fn new() -> Self {
        Self
    }

    /// Génère une réponse de façon ultra-optimisée via SkyNode
    #[inline(always)]
    pub async fn generate_response(
        &self,
        query: &Query,
        _personality: &Personality,
        _collective_wisdom: f32,
        skynode: Option<Arc<Mutex<SkyNode>>>,
    ) -> Result<String, String> {
        
        if let Some(skynode) = skynode {
            let request = AIRequest {
                prompt: query.content.clone(),
                ai: "thevie".to_string(),
                max_tokens: 1024,
            };

            let mut node = skynode.lock().await;
            return node.generate_with_ai(request).await.map(|r| r.text);
        }

        // Fallback ultra-léger
        Ok("SkyNode non disponible.".to_string())
    }
}