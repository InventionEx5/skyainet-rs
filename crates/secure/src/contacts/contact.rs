// crates/secure/src/contacts/contact.rs
// =====================================================
// Contact v6.1 — Structure de Contact Intelligente + DID
// SkyAInet × Nikola T369 — Réputation + Vérification Multi-Niveaux + QR + DID
// Compatible avec Messaging, Groupes et ContactManager
// =====================================================

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use tracing::debug;

use crate::identity::did::Did;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Contact {
    pub node_id: [u8; 32],
    pub name: String,
    pub public_key: Vec<u8>,
    pub signature: Vec<u8>,
    pub did: Option<Did>,                    // ← Identité décentralisée
    pub reputation_score: i32,
    pub verification_level: u8,
    pub qr_code_hash: Option<String>,
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
    ) -> Self {
        let now = Utc::now();

        Self {
            node_id,
            name,
            public_key,
            signature: Vec::new(),
            did: None,
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

    /// Associe un DID à ce contact (décentralisation de l'identité)
    pub fn set_did(&mut self, did: Did) {
        self.did = Some(did);
        if self.verification_level < 2 {
            self.verification_level = 2;
        }
        debug!("[Contact] DID associé à {}", self.name);
    }

    /// Retourne le DID sous forme de chaîne courte
    pub fn get_did_string(&self) -> Option<String> {
        self.did.as_ref().map(|d| d.to_short_string())
    }

    /// Vérifie si le contact a une identité décentralisée forte
    pub fn has_decentralized_identity(&self) -> bool {
        self.did.is_some() && self.verification_level >= 2
    }

    /// Met à jour la réputation
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
        )
    }
}