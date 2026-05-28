// crates/financial/src/lib.rs
// =====================================================
// SkyAInet Financial Crate v5.0
// Treasury Management + Liquidity + Financial Operations
// =====================================================

#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

pub mod treasury;
pub mod liquidity;

// =====================================================
// RÉ-EXPORTS PUBLICS
// =====================================================

pub use treasury::TreasuryManager;
pub use liquidity::LiquidityManager;

// =====================================================
// VERSION DU CRATE
// =====================================================

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

pub fn version_info() -> String {
    format!("{} v{}", CRATE_NAME, VERSION)
}

// =====================================================
// FONCTION D'INITIALISATION GLOBALE
// =====================================================

/// Crée un système financier complet (Treasury + Liquidity)
pub async fn create_full_financial_system(
    rpc_url: &str,
    treasury_address: alloy::primitives::Address,
    uniswap_router: alloy::primitives::Address,
    chain_id: u64,
) -> Result<(TreasuryManager, LiquidityManager), String> {
    let treasury = TreasuryManager::new(rpc_url, treasury_address, chain_id, None).await?;
    let liquidity = LiquidityManager::new(
        treasury.provider.clone(),
        uniswap_router,
        Some(std::sync::Arc::new(tokio::sync::Mutex::new(treasury.clone()))),
    );

    info!("[Financial] Full financial system initialized successfully");
    Ok((treasury, liquidity))
}