// crates/node/src/marketplace.rs
// =====================================================
// ComputeMarketplace v5.0 — Marché de Puissance de Calcul Décentralisé
// Location sécurisée + Réputation + Paiements intelligents + Intégration Rewards
// =====================================================

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use tracing::{info, warn, debug};
use chrono::{DateTime, Utc};

use skyainet_secure_transport::crypto::hybrid::HybridTransport;
use crate::rewards::UserRewards;
use crate::skyainet_node::SkyAInetNode;

/// Offre de location de puissance de calcul
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RentalOffer {
    pub offer_id: String,
    pub node_id: [u8; 32],
    pub owner: String,
    pub price_per_hour: u64,           // en SKY
    pub available_hours: u32,
    pub min_duration_hours: u32,
    pub reputation_required: f32,      // Score minimum de réputation du locataire
    pub created_at: DateTime<Utc>,
    pub is_active: bool,
}

/// Location active
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveRental {
    pub rental_id: String,
    pub node_id: [u8; 32],
    pub renter: String,
    pub owner: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub total_price: u128,
    pub status: RentalStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RentalStatus {
    Active,
    Completed,
    Cancelled,
    Disputed,
}

pub struct ComputeMarketplace {
    pub offers: HashMap<String, RentalOffer>,
    pub active_rentals: HashMap<String, ActiveRental>,
    pub total_volume_sky: u128,
    pub hybrid: HybridTransport,
}

impl ComputeMarketplace {
    pub fn new() -> Self {
        Self {
            offers: HashMap::new(),
            active_rentals: HashMap::new(),
            total_volume_sky: 0,
            hybrid: HybridTransport::new(true),
        }
    }

    /// Publie une nouvelle offre de location
    pub fn publish_offer(
        &mut self,
        node_id: [u8; 32],
        owner: String,
        price_per_hour: u64,
        available_hours: u32,
        reputation_required: f32,
    ) -> Result<String, String> {
        let offer_id = format!("offer-{}", uuid::Uuid::new_v4());

        let offer = RentalOffer {
            offer_id: offer_id.clone(),
            node_id,
            owner,
            price_per_hour,
            available_hours,
            min_duration_hours: 2,
            reputation_required: reputation_required.clamp(0.4, 0.95),
            created_at: Utc::now(),
            is_active: true,
        };

        self.offers.insert(offer_id.clone(), offer);

        info!("[Marketplace] Nouvelle offre publiée → ID: {}", offer_id);
        Ok(offer_id)
    }

    /// Loue un nœud avec vérification de réputation
    pub async fn rent_node(
        &mut self,
        offer_id: &str,
        renter: String,
        renter_reputation: f32,
        duration_hours: u32,
        rewards: &mut UserRewards,
    ) -> Result<ActiveRental, String> {
        let offer = self.offers.get_mut(offer_id).ok_or("Offre introuvable")?;

        if !offer.is_active || offer.available_hours < duration_hours {
            return Err("Offre non disponible ou durée insuffisante".to_string());
        }

        if renter_reputation < offer.reputation_required {
            return Err(format!(
                "Réputation insuffisante (requis: {:.2}, actuel: {:.2})",
                offer.reputation_required, renter_reputation
            ));
        }

        let total_price = offer.price_per_hour as u128 * duration_hours as u128;

        let rental = ActiveRental {
            rental_id: format!("rental-{}", uuid::Uuid::new_v4()),
            node_id: offer.node_id,
            renter: renter.clone(),
            owner: offer.owner.clone(),
            start_time: Utc::now(),
            end_time: Utc::now() + chrono::Duration::hours(duration_hours as i64),
            total_price,
            status: RentalStatus::Active,
        };

        self.active_rentals.insert(rental.rental_id.clone(), rental.clone());
        offer.available_hours -= duration_hours;
        self.total_volume_sky += total_price;

        // Récompense pour le propriétaire
        rewards.add_reward(crate::rewards::RewardReason::RentalIncome, (total_price as f64 * 0.92) as u128);

        info!("[Marketplace] Location confirmée : {} SKY pour {} heures", total_price, duration_hours);

        Ok(rental)
    }

    /// Termine une location et distribue les rewards
    pub async fn complete_rental(
        &mut self,
        rental_id: &str,
        rewards: &mut UserRewards,
    ) -> Result<u128, String> {
        let rental = self.active_rentals.get_mut(rental_id).ok_or("Location introuvable")?;

        if rental.status != RentalStatus::Active {
            return Err("Location déjà terminée".to_string());
        }

        rental.status = RentalStatus::Completed;

        // Paiement final au propriétaire
        let owner_reward = (rental.total_price as f64 * 0.93) as u128; // 7% de frais plateforme

        rewards.add_reward(crate::rewards::RewardReason::RentalIncome, owner_reward);

        info!("[Marketplace] Location terminée → {} SKY payés au propriétaire", owner_reward);

        Ok(owner_reward)
    }

    pub fn get_available_offers(&self) -> Vec<&RentalOffer> {
        self.offers.values()
            .filter(|o| o.is_active && o.available_hours > 0)
            .collect()
    }

    pub fn get_active_rentals(&self) -> Vec<&ActiveRental> {
        self.active_rentals.values().collect()
    }

    pub fn get_market_stats(&self) -> (u128, usize, usize) {
        (self.total_volume_sky, self.offers.len(), self.active_rentals.len())
    }
}