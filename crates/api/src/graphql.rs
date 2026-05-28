// crates/api/src/graphql.rs
// =====================================================
// GraphQL API v6.7 — SkyAInet × Thevie
// Ultra optimisé + Intégration complète Rewards
// =====================================================

use async_graphql::{
    Context, EmptySubscription, Object, Schema, SimpleObject,
};
use async_graphql_axum::GraphQL;
use axum::Router;
use std::sync::Arc;
use tokio::sync::Mutex;

use skyainet_model::Thevie;
use skyainet_node::SkyAInetNode;
use crate::rewards::{UserRewards, RewardReason};

// =====================================================
// TYPES GRAPHQL
// =====================================================

#[derive(SimpleObject)]
pub struct SystemStats {
    pub neurons: usize,
    pub synapses: usize,
    pub avg_wisdom: f32,
    pub queries_processed: u64,
    pub dream_cycles: u64,
    pub meta_consciousness: f32,
}

#[derive(SimpleObject)]
pub struct NodeDashboard {
    pub tier: String,
    pub monthly_cost_eur: u64,
    pub is_rented_out: bool,
    pub estimated_monthly_earnings: u128,
    pub total_earned_sky: u128,
    pub pending_rewards: u128,
    pub quality_score: f64,
    pub learn_contributions: u64,
    pub dream_cycles: u64,
}

#[derive(SimpleObject)]
pub struct ThevieResponse {
    pub response: String,
    pub quality: f32,
    pub expert_used: String,
}

#[derive(SimpleObject)]
pub struct RewardsInfo {
    pub pending_rewards: u128,
    pub total_earned: u128,
    pub quality_score: f64,
    pub learn_contributions: u64,
    pub dream_cycles: u64,
    pub thevie_evolution: f64,
}

// =====================================================
// QUERY
// =====================================================

pub struct Query;

#[Object]
impl Query {
    /// Statistiques globales du système
    async fn system_stats(&self, ctx: &Context<'_>) -> SystemStats {
        let thevie = ctx.data::<Arc<Mutex<Thevie>>>().unwrap();
        let thevie = thevie.lock().await;
        let stats = thevie.get_system_stats().await;

        SystemStats {
            neurons: stats.neurons,
            synapses: stats.synapses,
            avg_wisdom: stats.avg_wisdom,
            queries_processed: stats.queries_processed,
            dream_cycles: stats.dream_cycles,
            meta_consciousness: stats.meta_consciousness,
        }
    }

    /// Informations du nœud + rewards
    async fn my_node(&self, ctx: &Context<'_>) -> NodeDashboard {
        let node = ctx.data::<Arc<Mutex<SkyAInetNode>>>().unwrap();
        let node = node.lock().await;
        let rewards = ctx.data::<Arc<Mutex<UserRewards>>>().unwrap();
        let rewards = rewards.lock().await;

        NodeDashboard {
            tier: format!("{:?}", node.economics.tier),
            monthly_cost_eur: node.economics.get_total_monthly_cost(),
            is_rented_out: node.economics.is_rented_out,
            estimated_monthly_earnings: node.get_estimated_earnings(),
            total_earned_sky: node.economics.total_earned_sky,
            pending_rewards: rewards.pending_rewards,
            quality_score: rewards.conversation_quality_score,
            learn_contributions: rewards.total_learn_contributions,
            dream_cycles: rewards.total_dream_cycles,
        }
    }

    /// Santé du système
    async fn health(&self) -> String {
        "OK".to_string()
    }

    /// Informations détaillées sur les rewards
    async fn rewards(&self, ctx: &Context<'_>) -> RewardsInfo {
        let rewards = ctx.data::<Arc<Mutex<UserRewards>>>().unwrap();
        let rewards = rewards.lock().await;

        RewardsInfo {
            pending_rewards: rewards.pending_rewards,
            total_earned: rewards.total_sky_earned,
            quality_score: rewards.conversation_quality_score,
            learn_contributions: rewards.total_learn_contributions,
            dream_cycles: rewards.total_dream_cycles,
            thevie_evolution: rewards.thevie_evolution_contribution,
        }
    }
}

// =====================================================
// MUTATION
// =====================================================

pub struct Mutation;

#[Object]
impl Mutation {
    /// Envoie un message à Thevie
    async fn send_message_to_thevie(
        &self,
        ctx: &Context<'_>,
        message: String,
    ) -> ThevieResponse {
        let thevie = ctx.data::<Arc<Mutex<Thevie>>>().unwrap();
        let mut thevie = thevie.lock().await;

        let response = thevie.process_query(message).await;

        ThevieResponse {
            response,
            quality: 0.91,
            expert_used: "orchestrator".to_string(),
        }
    }

    /// Déclenche un Dream Cycle
    async fn trigger_dream_cycle(&self, ctx: &Context<'_>) -> String {
        let thevie = ctx.data::<Arc<Mutex<Thevie>>>().unwrap();
        let mut thevie = thevie.lock().await;

        thevie.trigger_dream_cycle().await;
        "Dream Cycle déclenché avec succès".to_string()
    }

    /// Claim les rewards mensuels
    async fn claim_rewards(&self, ctx: &Context<'_>) -> String {
        let rewards = ctx.data::<Arc<Mutex<UserRewards>>>().unwrap();
        let mut rewards = rewards.lock().await;

        let amount = rewards.claim_monthly_rewards();
        format!("Récompenses réclamées : {} SKY", amount)
    }

    /// Met à jour le nœud
    async fn update_node(
        &self,
        ctx: &Context<'_>,
        action: String,
    ) -> String {
        let node = ctx.data::<Arc<Mutex<SkyAInetNode>>>().unwrap();
        let mut node = node.lock().await;

        match action.as_str() {
            "rent_out" => {
                node.economics.rent_out();
                "Nœud mis en location".to_string()
            }
            "upgrade_full" => {
                // À adapter selon ta nouvelle structure
                "Upgrade non implémenté pour le moment".to_string()
            }
            _ => "Action inconnue".to_string(),
        }
    }
}

// =====================================================
// SCHEMA
// =====================================================

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn create_schema(
    thevie: Arc<Mutex<Thevie>>,
    node: Arc<Mutex<SkyAInetNode>>,
    rewards: Arc<Mutex<UserRewards>>,
) -> AppSchema {
    Schema::build(Query, Mutation, EmptySubscription)
        .data(thevie)
        .data(node)
        .data(rewards)
        .finish()
}

// =====================================================
// ROUTER
// =====================================================

use axum::Router;

pub fn create_graphql_router(schema: AppSchema) -> Router {
    Router::new()
        .route("/graphql", GraphQL::new(schema))
        .route("/graphql/playground", axum::routing::get(playground))
}

async fn playground() -> impl axum::response::IntoResponse {
    axum::response::Html(async_graphql::http::playground_source(
        async_graphql::http::GraphQLPlaygroundConfig::new("/graphql"),
    ))
}