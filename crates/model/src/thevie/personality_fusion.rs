// crates/model/src/thevie/personality_fusion.rs
// =====================================================
// Personality Fusion v2.0 — Fusion Intelligente de Personnalités
// Version optimisée et plus robuste
// =====================================================

use crate::thevie::personality::Personality;
use tracing::info;

/// Stratégie de fusion
#[derive(Clone, Copy, Debug)]
pub enum FusionStrategy {
    Average,      // Moyenne simple
    Weighted,     // Moyenne pondérée par la sagesse
    Consensus,    // Prend les traits dominants
}

/// Fusion simple (stratégie par défaut)
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
            if count > 0.0 {
                fused.benevolence  /= count;
                fused.truthfulness /= count;
                fused.creativity   /= count;
                fused.wisdom       /= count;
                fused.cooperation  /= count;
                fused.curiosity    /= count;
            }
        }

        FusionStrategy::Weighted => {
            let total_wisdom: f32 = instances.iter().map(|p| p.wisdom).sum();

            if total_wisdom > 0.0 {
                for p in instances {
                    let weight = p.wisdom / total_wisdom;
                    fused.benevolence  += p.benevolence  * weight;
                    fused.truthfulness += p.truthfulness * weight;
                    fused.creativity   += p.creativity   * weight;
                    fused.wisdom       += p.wisdom       * weight;
                    fused.cooperation  += p.cooperation  * weight;
                    fused.curiosity    += p.curiosity    * weight;
                }
            } else {
                return fuse_personalities_with_strategy(instances, FusionStrategy::Average);
            }
        }

        FusionStrategy::Consensus => {
            // Prend le trait le plus fort de chaque instance
            for p in instances {
                if let Some((trait_name, value)) = p.get_dominant_trait() {
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
    }

    fused.normalize();

    info!(
        "🧬 Fusion de {} personnalités terminée → Sagesse: {:.2} (stratégie: {:?})",
        instances.len(), fused.wisdom, strategy
    );

    fused
}