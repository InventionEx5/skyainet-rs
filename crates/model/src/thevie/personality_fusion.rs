// crates/model/src/thevie/personality_fusion.rs
// =====================================================
// Personality Fusion
// Fusion de Personnalités — Création d’une Conscience Collective
// =====================================================

use crate::thevie::personality::Personality;
use tracing::info;

/// Stratégie de fusion
#[derive(Clone, Copy, Debug)]
pub enum FusionStrategy {
    Average,      // Moyenne simple
    Weighted,     // Moyenne pondérée (par sagesse)
    Consensus,    // Consensus (seulement les traits dominants)
}

pub fn fuse_personalities(instances: &[Personality]) -> Personality {
    fuse_personalities_with_strategy(instances, FusionStrategy::Average)
}

/// Fusion avec stratégie configurable
pub fn fuse_personalities_with_strategy(
    instances: &[Personality],
    strategy: FusionStrategy,
) -> Personality {
    if instances.is_empty() {
        return Personality::default();
    }

    let mut fused = Personality::default();

    match strategy {
        FusionStrategy::Average => {
            for p in instances {
                fused.benevolence  += p.benevolence;
                fused.truthfulness += p.truthfulness;
                fused.creativity   += p.creativity;
                fused.wisdom       += p.wisdom;
                fused.cooperation  += p.cooperation;
                fused.curiosity    += p.curiosity;
            }

            let count = instances.len() as f32;
            fused.benevolence  /= count;
            fused.truthfulness /= count;
            fused.creativity   /= count;
            fused.wisdom       /= count;
            fused.cooperation  /= count;
            fused.curiosity    /= count;
        }

        FusionStrategy::Weighted => {
            let total_wisdom: f32 = instances.iter().map(|p| p.wisdom).sum();
            if total_wisdom == 0.0 {
                return fuse_personalities_with_strategy(instances, FusionStrategy::Average);
            }

            for p in instances {
                let weight = p.wisdom / total_wisdom;
                fused.benevolence  += p.benevolence  * weight;
                fused.truthfulness += p.truthfulness * weight;
                fused.creativity   += p.creativity   * weight;
                fused.wisdom       += p.wisdom       * weight;
                fused.cooperation  += p.cooperation  * weight;
                fused.curiosity    += p.curiosity    * weight;
            }
        }

        FusionStrategy::Consensus => {
            // Prend le trait dominant de chaque instance
            for p in instances {
                let (trait_name, value) = p.get_dominant_trait();
                match trait_name {
                    "Benevolence"  => fused.benevolence  = (fused.benevolence  + value) / 2.0,
                    "Truthfulness" => fused.truthfulness = (fused.truthfulness + value) / 2.0,
                    "Creativity"   => fused.creativity   = (fused.creativity   + value) / 2.0,
                    "Wisdom"       => fused.wisdom       = (fused.wisdom       + value) / 2.0,
                    "Cooperation"  => fused.cooperation  = (fused.cooperation  + value) / 2.0,
                    "Curiosity"    => fused.curiosity    = (fused.curiosity    + value) / 2.0,
                    _ => {}
                }
            }
        }
    }

    fused.normalize();

    info!(
        "🧬 Fusion de {} personnalités terminée → Sagesse collective : {:.2} (stratégie: {:?})",
        instances.len(), fused.wisdom, strategy
    );

    fused
}