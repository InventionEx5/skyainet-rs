// crates/financial/src/treasury.rs
// =====================================================
// TreasuryManager v6.6 — Gestion du Trésor + Distribution Globale
// 15% Burn | 55% Users | 25% DAO | 5% Dev Team
// Intégré avec UserRewards + alloy-rs
// =====================================================

use alloy::primitives::{Address, U256, FixedBytes};
use alloy::providers::{Provider, ProviderBuilder, WsConnect};
use alloy::sol;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, warn, error};
use thiserror::Error;
use chrono::{DateTime, Utc};

use crate::rewards::{UserRewards, RewardReason};

sol! {
    #[sol(rpc)]
    contract TreasuryVault {
        function getBalance() external view returns (uint256);
        function triggerRebalance() external;
        function logSecureTransfer(bytes32 gematriaSessionId, uint256 amount) external;
        function recordEthicalScore(bytes32 nodeId, uint256 score) external;

        event RevenueDistributed(uint256 burned, uint256 rewarded, uint256 daoReserve, uint256 devTeam, uint256 timestamp);
        event TokensBurned(uint256 amount, uint256 timestamp);
        event RewardsDistributed(uint256 amount, uint256 timestamp);
        event DaoShareSent(uint256 amount, uint256 timestamp);
    }
}

#[derive(Error, Debug)]
pub enum TreasuryError {
    #[error("Provider error: {0}")]
    ProviderError(String),
    #[error("Contract call failed: {0}")]
    ContractError(String),
}

pub struct TreasuryManager {
    pub contract_address: Address,
    pub provider: Arc<dyn Provider>,
    pub chain_id: u64,

    pub last_rebalance: Mutex<DateTime<Utc>>,
    pub total_burned: Mutex<u128>,
    pub total_rewarded: Mutex<u128>,
    pub total_dao_reserve: Mutex<u128>,
    pub total_dev_team: Mutex<u128>,
}

impl TreasuryManager {
    pub async fn new(
        rpc_url: &str,
        contract_address: Address,
        chain_id: u64,
    ) -> Result<Self, TreasuryError> {
        let provider = if rpc_url.starts_with("ws") {
            let ws = WsConnect::new(rpc_url);
            ProviderBuilder::new()
                .with_recommended_fillers()
                .on_ws(ws)
                .await
                .map_err(|e| TreasuryError::ProviderError(e.to_string()))?
        } else {
            ProviderBuilder::new()
                .with_recommended_fillers()
                .on_http(rpc_url.parse().map_err(|e| TreasuryError::ProviderError(e.to_string()))?)
        };

        Ok(Self {
            contract_address,
            provider: Arc::new(provider),
            chain_id,
            last_rebalance: Mutex::new(Utc::now()),
            total_burned: Mutex::new(0),
            total_rewarded: Mutex::new(0),
            total_dao_reserve: Mutex::new(0),
            total_dev_team: Mutex::new(0),
        })
    }

    /// Récupère le solde du Treasury
    pub async fn get_balance(&self) -> Result<U256, TreasuryError> {
        let contract = TreasuryVault::new(self.contract_address, self.provider.clone());
        contract.getBalance().call().await
            .map_err(|e| TreasuryError::ContractError(e.to_string()))
    }

    /// Distribution globale des rewards (15% Burn / 55% Users / 25% DAO / 5% Dev)
    pub async fn distribute_rewards(
        &self,
        total_amount: u128,
        user_rewards: &mut UserRewards,
    ) -> Result<(), TreasuryError> {
        if total_amount == 0 {
            return Ok(());
        }

        let burn_amount = (total_amount as f64 * 0.15) as u128;
        let users_amount = (total_amount as f64 * 0.55) as u128;
        let dao_amount = (total_amount as f64 * 0.25) as u128;
        let dev_amount = total_amount - burn_amount - users_amount - dao_amount;

        // Mise à jour des compteurs
        *self.total_burned.lock().await += burn_amount;
        *self.total_rewarded.lock().await += users_amount;
        *self.total_dao_reserve.lock().await += dao_amount;
        *self.total_dev_team.lock().await += dev_amount;

        // Ajout aux rewards utilisateurs
        user_rewards.total_sky_earned += users_amount;

        info!(
            "[Treasury] Distribution: Burn={} | Users={} | DAO={} | Dev={}",
            burn_amount, users_amount, dao_amount, dev_amount
        );

        // TODO: Appeler le smart contract pour exécuter réellement la distribution
        // (transfert on-chain, burn, etc.)

        Ok(())
    }

    /// Déclenche un rebalance du Treasury
    pub async fn trigger_rebalance(&self) -> Result<(), TreasuryError> {
        let contract = TreasuryVault::new(self.contract_address, self.provider.clone());

        let tx = contract.triggerRebalance().send().await
            .map_err(|e| TreasuryError::ContractError(e.to_string()))?;

        *self.last_rebalance.lock().await = Utc::now();

        info!("[Treasury] Rebalance executed | Tx: {:?}", tx.tx_hash());
        Ok(())
    }

    /// Enregistre un Ethical Score on-chain
    pub async fn record_ethical_score(&self, node_id: [u8; 32], score: u64) -> Result<(), TreasuryError> {
        let contract = TreasuryVault::new(self.contract_address, self.provider.clone());

        let tx = contract
            .recordEthicalScore(FixedBytes::from(node_id), U256::from(score))
            .send()
            .await
            .map_err(|e| TreasuryError::ContractError(e.to_string()))?;

        info!("[Treasury] Ethical Score {} recorded for node {:?}", score, node_id);
        Ok(())
    }

    pub async fn get_stats(&self) -> Result<(U256, u128, u128, u128, u128), TreasuryError> {
        let balance = self.get_balance().await?;
        let burned = *self.total_burned.lock().await;
        let rewarded = *self.total_rewarded.lock().await;
        let dao = *self.total_dao_reserve.lock().await;
        let dev = *self.total_dev_team.lock().await;

        Ok((balance, burned, rewarded, dao, dev))
    }
}