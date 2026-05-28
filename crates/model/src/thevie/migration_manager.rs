// crates/model/src/thevie/migration_manager.rs
// =====================================================
// Migration Manager v2.1 — Voyage Sécurisé avec RomanT369
// Version finale avec le nouveau système cryptographique
// =====================================================

use crate::thevie::personality::Personality;
use crate::thevie::evolution::EvolutionEngine;
use crate::thevie::memory::LongTermMemory;
use skyainet_secure_transport::crypto::roman_t369::RomanT369;
use serde::{Serialize, Deserialize};
use tracing::{info, warn};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TravelPackage {
    pub version: String,
    pub wisdom: f32,
    pub benevolence: f32,
    pub creativity: f32,
    pub total_experiences: usize,
    pub evolution_level: u32,
    pub timestamp: u64,
    pub checksum: String,
}

pub struct AdvancedMigration {
    pub enabled: bool,
    pub encryption_enabled: bool,
    roman: RomanT369,
}

impl AdvancedMigration {
    pub fn new() -> Self {
        Self {
            enabled: true,
            encryption_enabled: true,
            roman: RomanT369::new([0x55u8; 32], [0u8; 12]),
        }
    }

    /// Prépare une instance pour le voyage (chiffrement RomanT369)
    pub fn prepare_travel(
        &self,
        personality: &Personality,
        evolution: &EvolutionEngine,
        memory: &LongTermMemory,
    ) -> Option<String> {
        if !self.enabled {
            return None;
        }

        let package = TravelPackage {
            version: "2.1".to_string(),
            wisdom: personality.wisdom,
            benevolence: personality.benevolence,
            creativity: personality.creativity,
            total_experiences: memory.experiences.len(),
            evolution_level: evolution.level,
            timestamp: crate::utils::now_millis(),
            checksum: self.calculate_checksum(personality, evolution, memory),
        };

        let serialized = serde_json::to_string(&package).ok()?;

        let final_data = if self.encryption_enabled {
            let encrypted = self.roman.encrypt(serialized.as_bytes());
            format!("ROMAN|{}", hex::encode(encrypted))
        } else {
            serialized
        };

        info!("✈️ Voyage préparé avec RomanT369 ({} octets)", final_data.len());
        Some(final_data)
    }

    /// Reçoit et restaure une instance après voyage
    pub fn receive_traveler(&self, travel_data: &str) -> Option<(Personality, EvolutionEngine, LongTermMemory)> {
        if !self.enabled || travel_data.is_empty() {
            return None;
        }

        let data = if travel_data.starts_with("ROMAN|") {
            let hex_data = &travel_data[6..];
            let encrypted = hex::decode(hex_data).ok()?;
            let decrypted = self.roman.decrypt(&encrypted)?;
            String::from_utf8(decrypted).ok()?
        } else {
            travel_data.to_string()
        };

        let package: TravelPackage = serde_json::from_str(&data).ok()?;

        if package.version != "2.1" {
            warn!("[Migration] Version incompatible : {}", package.version);
            return None;
        }

        let mut personality = Personality::new();
        personality.wisdom = package.wisdom;
        personality.benevolence = package.benevolence;
        personality.creativity = package.creativity;

        let evolution = EvolutionEngine::new_with_level(package.evolution_level);
        let memory = LongTermMemory::new();

        info!(
            "✅ Voyageur reçu avec succès (RomanT369) ! Sagesse : {:.2}",
            personality.wisdom
        );

        Some((personality, evolution, memory))
    }

    fn calculate_checksum(
        &self,
        personality: &Personality,
        evolution: &EvolutionEngine,
        memory: &LongTermMemory,
    ) -> String {
        format!(
            "{}-{}-{}-{}",
            personality.wisdom,
            personality.benevolence,
            evolution.level,
            memory.experiences.len()
        )
    }
}