// crates/secure/src/contacts/contact.rs
// =====================================================
// Contact v6.0 — Structure de Contact Intelligente
// SkyAInet × Nikola T369 — Réputation + Vérification Multi-Niveaux + QR + ZipMemory Ready
// Version Ultra Améliorée + Compatible avec Messaging & Manager
// =====================================================

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use tracing::debug;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Contact {
    pub node_id: [u8; 32],
    pub name: String,                    // Nom d'affichage principal
    pub public_key: Vec<u8>,
    pub signature: Vec<u8>,              // Signature Dilithium de la clé publique
    pub did: String,
    pub reputation_score: i32,           // 0 → 100
    pub verification_level: u8,          // 0=None, 1=Signature, 2=Signature+QR, 3=Full Trust
    pub qr_code_hash: Option<String>,    // Hash du QR Air-Gap signé
    pub interaction_count: u32,
    pub last_interaction: Option<DateTime<Utc>>,
    pub notes: Option<String>,
    pub is_favorite: bool,
    pub revoked: bool,
    pub created_at: DateTime<Utc>,
}

impl Contact {
    /// Crée un nouveau contact
    pub fn new(
        node_id: [u8; 32],
        name: String,
        public_key: Vec<u8>,
        did: String,
    ) -> Self {
        let now = Utc::now();

        Self {
            node_id,
            name,
            public_key,
            signature: Vec::new(),
            did,
            reputation_score: 50,
            verification_level: 0,
            qr_code_hash: None,
            interaction_count: 0,
            last_interaction: Some(now),
            notes: None,
            is_favorite: false,
            revoked: false,
            created_at: now,
        }
    }

    /// Met à jour la réputation (avec clamp)
    pub fn update_reputation(&mut self, delta: i32) {
        self.reputation_score = (self.reputation_score + delta).clamp(0, 100);
        debug!(
            "[Contact] Réputation mise à jour pour {} → {}",
            self.name, self.reputation_score
        );
    }

    /// Enregistre une interaction
    pub fn touch(&mut self) {
        self.last_interaction = Some(Utc::now());
        self.interaction_count += 1;
    }

    /// Incrémente le compteur d'interactions
    pub fn increment_interaction(&mut self) {
        self.interaction_count += 1;
        self.last_interaction = Some(Utc::now());
    }

    /// Définit la signature Dilithium
    pub fn set_signature(&mut self, signature: Vec<u8>) {
        self.signature = signature;
    }

    /// Définit le hash du QR Air-Gap
    pub fn set_qr_hash(&mut self, hash: String) {
        self.qr_code_hash = Some(hash);
        if self.verification_level < 2 {
            self.verification_level = 2;
        }
    }

    /// Révoque le contact
    pub fn revoke(&mut self, reason: Option<String>) {
        self.revoked = true;
        if let Some(r) = reason {
            self.notes = Some(format!("Révoqué: {}", r));
        }
        debug!("[Contact] Contact révoqué : {}", self.name);
    }

    /// Vérifie si le contact est considéré comme fiable
    pub fn is_trusted(&self) -> bool {
        self.verification_level >= 2 && !self.revoked && self.reputation_score >= 60
    }

    /// Vérifie si le contact peut être utilisé pour des opérations sensibles
    pub fn can_use_for_sensitive_operations(&self) -> bool {
        self.verification_level >= 2 && !self.revoked
    }

    /// Retourne le badge de vérification visuel
    pub fn verification_badge(&self) -> &'static str {
        match self.verification_level {
            0 => "⚠️ Non vérifié",
            1 => "🔐 Signature valide",
            2 => "📱 Vérifié (QR Air-Gap)",
            3 => "✅ Confiance élevée",
            _ => "❓ Inconnu",
        }
    }

    /// Âge du contact en jours
    pub fn age_days(&self) -> i64 {
        (Utc::now() - self.created_at).num_days()
    }

    /// Dernière interaction en jours
    pub fn days_since_last_interaction(&self) -> Option<i64> {
        self.last_interaction.map(|last| (Utc::now() - last).num_days())
    }

    /// Vérifie si le contact est récemment actif (< 7 jours)
    pub fn is_recently_active(&self) -> bool {
        match self.last_interaction {
            Some(last) => (Utc::now() - last).num_days() < 7,
            None => false,
        }
    }

    /// Retourne un résumé court du contact
    pub fn summary(&self) -> String {
        format!(
            "{} | {} | Rep: {}/100 | {}",
            self.name,
            self.verification_badge(),
            self.reputation_score,
            if self.is_favorite { "★" } else { "" }
        )
    }
}

impl Default for Contact {
    fn default() -> Self {
        Self::new(
            [0u8; 32],
            "Unknown Contact".to_string(),
            vec![],
            "did:t369:unknown".to_string(),
        )
    }
}