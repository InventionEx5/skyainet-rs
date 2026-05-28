// crates/sentinel/src/auto_healing.rs
// =====================================================
// Sentinel v4.0 — Auto-Healing & Self-Defense Intelligent
// Détection avancée + Actions autonomes + Intégration Thevie & Rewards
// =====================================================

use tracing::{info, warn, debug};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use crate::rewards::UserRewards;
use crate::skyainet_node::SkyAInetNode;

/// Niveau de gravité d'un problème
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IssueSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Problème détecté
#[derive(Debug, Clone)]
pub struct DetectedIssue {
    pub message: String,
    pub severity: IssueSeverity,
    pub timestamp: DateTime<Utc>,
    pub affected_node: Option<String>,
}

/// Actions de guérison
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HealingAction {
    TriggerFlashGematria,
    EnterLowPowerMode,
    WakeNode,
    BoostReputation,
    RebalanceCollective,
    PruneOldData,
    StartDreamCycle,
}

pub struct Sentinel {
    pub issues_detected: u64,
    pub heals_performed: u64,
    pub last_healing: Option<DateTime<Utc>>,
    pub healing_history: Vec<(HealingAction, DateTime<Utc>)>,
}

impl Sentinel {
    pub fn new() -> Self {
        Self {
            issues_detected: 0,
            heals_performed: 0,
            last_healing: None,
            healing_history: Vec::new(),
        }
    }

    /// Détection avancée des problèmes
    pub fn detect_issues(&mut self, node: &SkyAInetNode) -> Vec<DetectedIssue> {
        let mut issues = Vec::new();

        // Sagesse collective
        if node.metadata.reputation_score < 0.55 {
            issues.push(DetectedIssue {
                message: "Sagesse collective trop basse".to_string(),
                severity: IssueSeverity::High,
                timestamp: Utc::now(),
                affected_node: Some(node.metadata.peer_id.clone()),
            });
        }

        // Réputation
        if node.metadata.reputation_score < 0.45 {
            issues.push(DetectedIssue {
                message: "Réputation du nœud critique".to_string(),
                severity: IssueSeverity::Critical,
                timestamp: Utc::now(),
                affected_node: Some(node.metadata.peer_id.clone()),
            });
        }

        // Activité
        if node.state == crate::node_types::NodeState::Sleeping {
            issues.push(DetectedIssue {
                message: "Nœud inactif depuis longtemps".to_string(),
                severity: IssueSeverity::Medium,
                timestamp: Utc::now(),
                affected_node: Some(node.metadata.peer_id.clone()),
            });
        }

        if !issues.is_empty() {
            self.issues_detected += issues.len() as u64;
            warn!("[Sentinel] {} problèmes détectés", issues.len());
        }

        issues
    }

    /// Exécute les actions de guérison adaptatives
    pub async fn trigger_healing(&mut self, issues: &[DetectedIssue], node: &mut SkyAInetNode, rewards: &mut UserRewards) {
        for issue in issues {
            let action = self.choose_healing_action(issue);

            match action {
                HealingAction::TriggerFlashGematria => {
                    node.trigger_flash_gematria().await;
                    rewards.add_reward(crate::rewards::RewardReason::HealingContribution, 25);
                }
                HealingAction::EnterLowPowerMode => {
                    node.enter_low_power_mode().await;
                }
                HealingAction::WakeNode => {
                    node.wake().await;
                }
                HealingAction::BoostReputation => {
                    node.metadata.reputation_score = (node.metadata.reputation_score + 0.12).min(1.0);
                }
                HealingAction::RebalanceCollective => {
                    debug!("[Sentinel] Rebalancing collective wisdom");
                }
                HealingAction::PruneOldData => {
                    if let Some(zip) = &node.zip_memory {
                        let mut z = zip.lock().await;
                        let _ = z.compress_inactive_data().await;
                    }
                }
                HealingAction::StartDreamCycle => {
                    node.run_evolution_cycle().await;
                }
            }

            self.healing_history.push((action, Utc::now()));
            self.heals_performed += 1;
            self.last_healing = Some(Utc::now());
        }

        info!("[Sentinel] Healing completed: {} actions performed", issues.len());
    }

    fn choose_healing_action(&self, issue: &DetectedIssue) -> HealingAction {
        match issue.severity {
            IssueSeverity::Critical => HealingAction::TriggerFlashGematria,
            IssueSeverity::High => HealingAction::StartDreamCycle,
            IssueSeverity::Medium => HealingAction::EnterLowPowerMode,
            IssueSeverity::Low => HealingAction::PruneOldData,
        }
    }

    pub fn summary(&self) -> String {
        format!(
            "Sentinel | Issues: {} | Heals: {} | Last healing: {:?}",
            self.issues_detected, self.heals_performed, self.last_healing
        )
    }
}