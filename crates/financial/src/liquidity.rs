// crates/financial/src/liquidity.rs
// =====================================================
// LiquidityManager v5.0 — Gestion Avancée de Liquidité Uniswap V4
// Intégré avec Treasury, Rewards, Hybrid Crypto & Risk Management
// =====================================================

use alloy::primitives::{Address, U256, FixedBytes};
use alloy::providers::Provider;
use alloy::sol;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, warn, debug, error};
use thiserror::Error;
use chrono::{DateTime, Utc};

use skyainet_secure_transport::crypto::hybrid::HybridTransport;
use crate::rewards::UserRewards;
use crate::treasury::TreasuryManager;

sol! {
    #[sol(rpc)]
    interface IUniswapV4Router {
        function addLiquidity(
            address pool,
            uint256 amount0Desired,
            uint256 amount1Desired,
            uint256 amount0Min,
            uint256 amount1Min,
            address recipient,
            uint256 deadline
        ) external returns (uint256 amount0, uint256 amount1, uint256 liquidity);
        
        function removeLiquidity(
            address pool,
            uint256 liquidity,
            uint256 amount0Min,
            uint256 amount1Min,
            address recipient,
            uint256 deadline
        ) external returns (uint256 amount0, uint256 amount1);
    }
}

#[derive(Error, Debug)]
pub enum LiquidityError {
    #[error("Provider error: {0}")]
    ProviderError(String),
    #[error("Transaction failed: {0}")]
    TxFailed(String),
    #[error("Insufficient liquidity")]
    InsufficientLiquidity,
    #[error("Slippage tolerance exceeded")]
    SlippageExceeded,
    #[error("Invalid amount")]
    InvalidAmount,
}

pub struct LiquidityManager {
    pub provider: Arc<dyn Provider>,
    pub router_address: Address,
    pub hybrid: HybridTransport,
    pub treasury: Option<Arc<Mutex<TreasuryManager>>>,

    pub slippage_tolerance: f64,     // ex: 0.005 = 0.5%
    pub default_deadline_minutes: u64,
    
    pub last_add_liquidity: Mutex<DateTime<Utc>>,
    pub total_liquidity_provided: Mutex<U256>,
}

impl LiquidityManager {
    pub fn new(
        provider: Arc<dyn Provider>,
        router_address: Address,
        treasury: Option<Arc<Mutex<TreasuryManager>>>,
    ) -> Self {
        Self {
            provider,
            router_address,
            hybrid: HybridTransport::new(true),
            treasury,
            slippage_tolerance: 0.005,
            default_deadline_minutes: 30,
            last_add_liquidity: Mutex::new(Utc::now()),
            total_liquidity_provided: Mutex::new(U256::ZERO),
        }
    }

    /// Ajoute de la liquidité avec protection slippage + logging sécurisé
    pub async fn add_liquidity(
        &self,
        pool: Address,
        amount0: U256,
        amount1: U256,
        recipient: Address,
        rewards: &mut UserRewards,
    ) -> Result<(U256, U256, U256), LiquidityError> {
        if amount0.is_zero() || amount1.is_zero() {
            return Err(LiquidityError::InvalidAmount);
        }

        let router = IUniswapV4Router::new(self.router_address, self.provider.clone());

        let amount0_min = self.apply_slippage(amount0);
        let amount1_min = self.apply_slippage(amount1);

        let deadline = (Utc::now() + chrono::Duration::minutes(self.default_deadline_minutes as i64))
            .timestamp() as u64;

        info!(
            "[Liquidity] Adding liquidity to pool {:?} | Amount0: {} | Amount1: {}",
            pool, amount0, amount1
        );

        let result = router
            .addLiquidity(pool, amount0, amount1, amount0_min, amount1_min, recipient, U256::from(deadline))
            .send()
            .await
            .map_err(|e| LiquidityError::TxFailed(e.to_string()))?;

        let receipt = result.get_receipt().await
            .map_err(|e| LiquidityError::TxFailed(e.to_string()))?;

        let liquidity = U256::from(0); // À remplacer par parsing d'événement dans une vraie impl

        // Mise à jour stats
        *self.total_liquidity_provided.lock().await += liquidity;
        *self.last_add_liquidity.lock().await = Utc::now();

        // Récompense pour contribution liquidité
        rewards.add_reward(crate::rewards::RewardReason::LiquidityProvision, 85);

        info!("[Liquidity] Liquidity added successfully | Tx: {:?}", receipt.transaction_hash);

        Ok((amount0, amount1, liquidity))
    }

    fn apply_slippage(&self, amount: U256) -> U256 {
        let slippage_factor = U256::from((10000.0 - (self.slippage_tolerance * 10000.0)) as u64);
        amount * slippage_factor / U256::from(10000)
    }

    pub async fn remove_liquidity(
        &self,
        pool: Address,
        liquidity: U256,
        recipient: Address,
    ) -> Result<(U256, U256), LiquidityError> {
        let router = IUniswapV4Router::new(self.router_address, self.provider.clone());

        let deadline = (Utc::now() + chrono::Duration::minutes(self.default_deadline_minutes as i64))
            .timestamp() as u64;

        let amount0_min = U256::ZERO; // À calculer selon prix oracle dans une vraie version
        let amount1_min = U256::ZERO;

        let result = router
            .removeLiquidity(pool, liquidity, amount0_min, amount1_min, recipient, U256::from(deadline))
            .send()
            .await
            .map_err(|e| LiquidityError::TxFailed(e.to_string()))?;

        info!("[Liquidity] Liquidity removed successfully | Tx: {:?}", result.tx_hash());

        Ok((amount0_min, amount1_min))
    }

    pub fn set_slippage_tolerance(&mut self, tolerance: f64) {
        self.slippage_tolerance = tolerance.clamp(0.001, 0.08); // 0.1% à 8%
        info!("[Liquidity] Slippage tolerance updated to {:.2}%", tolerance * 100.0);
    }

    pub async fn get_total_liquidity_provided(&self) -> U256 {
        *self.total_liquidity_provided.lock().await
    }
}