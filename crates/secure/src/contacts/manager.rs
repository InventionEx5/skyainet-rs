// crates/secure/src/contacts/manager.rs
// =====================================================
// ContactManager v6.3 — Version Finale (Best of Both + DID Bonus)
// SkyAInet × Nikola T369
// =====================================================

use super::contact::Contact;
use super::verification::ContactVerification;
use crate::identity::did::{Did, DidError};
use std::collections::HashMap;
use tracing::{info, debug, warn};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContactManagerError {
    #[error("Contact not found")]
    ContactNotFound,
    #[error("Contact already exists")]
    ContactAlreadyExists,
    #[error("Maximum number of contacts reached")]
    MaxContactsReached,
    #[error("Invalid DID")]
    InvalidDid,
    #[error("Contact is revoked")]
    ContactRevoked,
}

pub struct ContactManager {
    contacts: HashMap<[u8; 32], Contact>,
    favorites: Vec<[u8; 32]>,
    verification: ContactVerification,
    max_contacts: usize,
}

impl ContactManager {
    pub fn new() -> Self {
        Self {
            contacts: HashMap::new(),
            favorites: Vec::new(),
            verification: ContactVerification,
            max_contacts: 500,
        }
    }

    pub fn with_max_contacts(mut self, max: usize) -> Self {
        self.max_contacts = max;
        self
    }

    // ==================== MÉTHODES DE BASE ====================

    pub fn add_or_update(&mut self, mut contact: Contact) -> Result<(), ContactManagerError> {
        if self.contacts.len() >= self.max_contacts && !self.contacts.contains_key(&contact.node_id) {
            return Err(ContactManagerError::MaxContactsReached);
        }
        let is_new = !self.contacts.contains_key(&contact.node_id);
        self.contacts.insert(contact.node_id, contact.clone());

        if is_new {
            info!("[ContactManager] Nouveau contact ajouté : {} (total: {})", contact.name, self.contacts.len());
        } else {
            debug!("[ContactManager] Contact mis à jour : {}", contact.name);
        }
        Ok(())
    }

    pub fn get(&self, node_id: &[u8; 32]) -> Option<&Contact> {
        self.contacts.get(node_id)
    }

    pub fn get_contact_mut(&mut self, node_id: &[u8; 32]) -> Option<&mut Contact> {
        self.contacts.get_mut(node_id)
    }

    pub fn remove(&mut self, node_id: &[u8; 32]) -> Result<(), ContactManagerError> {
        if self.contacts.remove(node_id).is_some() {
            self.favorites.retain(|id| id != node_id);
            debug!("[ContactManager] Contact supprimé");
            Ok(())
        } else {
            Err(ContactManagerError::ContactNotFound)
        }
    }

    // ==================== DID + BONUS RÉPUTATION ====================

    /// Lie un DID à un contact + applique automatiquement le bonus de réputation
    pub fn link_did_to_contact(&mut self, node_id: &[u8; 32], did: Did) -> Result<(), ContactManagerError> {
        let contact = self.contacts.get_mut(node_id).ok_or(ContactManagerError::ContactNotFound)?;
        
        if contact.revoked {
            return Err(ContactManagerError::ContactRevoked);
        }

        contact.set_did(did);
        
        // === BONUS AUTOMATIQUE DE RÉPUTATION ===
        if contact.has_decentralized_identity() && contact.verification_level >= 2 {
            contact.update_reputation(12);
            debug!(
                "[ContactManager] +12 points de réputation accordés à {} (DID vérifié)",
                contact.name
            );
        }

        debug!("[ContactManager] DID lié au contact {}", contact.name);
        Ok(())
    }

    pub fn create_and_link_did(
        &mut self,
        node_id: &[u8; 32],
        public_key: Vec<u8>,
    ) -> Result<String, ContactManagerError> {
        let did = Did::new(public_key).map_err(|_| ContactManagerError::InvalidDid)?;
        self.link_did_to_contact(node_id, did.clone())?;
        Ok(did.to_short_string())
    }

    /// Méthode manuelle (au cas où tu veux l'appeler séparément)
    pub fn apply_did_reputation_bonus(&mut self, node_id: &[u8; 32]) -> Result<(), ContactManagerError> {
        let contact = self.contacts.get_mut(node_id)
            .ok_or(ContactManagerError::ContactNotFound)?;

        if !contact.has_decentralized_identity() || contact.verification_level < 2 {
            return Err(ContactManagerError::InvalidDid);
        }

        contact.update_reputation(12);

        debug!(
            "[ContactManager] +12 points de réputation accordés manuellement à {} (DID vérifié)",
            contact.name
        );

        Ok(())
    }

    pub fn get_contacts_with_did(&self) -> Vec<&Contact> {
        self.contacts.values().filter(|c| c.has_decentralized_identity()).collect()
    }

    pub fn can_join_group(&self, node_id: &[u8; 32]) -> bool {
        if let Some(contact) = self.contacts.get(node_id) {
            contact.has_decentralized_identity() && contact.verification_level >= 2 && !contact.revoked
        } else {
            false
        }
    }

    // ==================== RÉPUTATION & INTERACTIONS ====================

    pub fn update_reputation(&mut self, node_id: &[u8; 32], delta: i32) -> Result<(), ContactManagerError> {
        let contact = self.contacts.get_mut(node_id).ok_or(ContactManagerError::ContactNotFound)?;
        contact.update_reputation(delta);
        Ok(())
    }

    pub fn touch_contact(&mut self, node_id: &[u8; 32]) -> Result<(), ContactManagerError> {
        let contact = self.contacts.get_mut(node_id).ok_or(ContactManagerError::ContactNotFound)?;
        contact.touch();
        Ok(())
    }

    pub fn revoke_contact(&mut self, node_id: &[u8; 32], reason: Option<String>) -> Result<(), ContactManagerError> {
        let contact = self.contacts.get_mut(node_id).ok_or(ContactManagerError::ContactNotFound)?;
        contact.revoke(reason);
        warn!("[ContactManager] Contact révoqué : {}", contact.name);
        Ok(())
    }

    // ==================== FAVORIS ====================

    pub fn toggle_favorite(&mut self, node_id: &[u8; 32]) -> Result<(), ContactManagerError> {
        if !self.contacts.contains_key(node_id) {
            return Err(ContactManagerError::ContactNotFound);
        }
        if self.favorites.contains(node_id) {
            self.favorites.retain(|id| id != node_id);
        } else {
            self.favorites.push(*node_id);
        }
        Ok(())
    }

    pub fn get_favorites(&self) -> Vec<&Contact> {
        self.favorites.iter().filter_map(|id| self.contacts.get(id)).collect()
    }

    // ==================== RECHERCHE & TRI ====================

    pub fn get_sorted_by_reputation(&self) -> Vec<&Contact> {
        let mut list: Vec<&Contact> = self.contacts.values().collect();
        list.sort_by(|a, b| b.reputation_score.cmp(&a.reputation_score));
        list
    }

    pub fn get_sorted_by_last_interaction(&self) -> Vec<&Contact> {
        let mut list: Vec<&Contact> = self.contacts.values().collect();
        list.sort_by(|a, b| b.last_interaction.cmp(&a.last_interaction));
        list
    }

    pub fn search_by_name(&self, query: &str) -> Vec<&Contact> {
        let q = query.to_lowercase();
        self.contacts.values().filter(|c| c.name.to_lowercase().contains(&q)).collect()
    }

    pub fn get_recommended_contacts(&self, limit: usize) -> Vec<&Contact> {
        let mut list: Vec<&Contact> = self.contacts.values().collect();
        list.sort_by(|a, b| {
            let score_a = a.reputation_score as f64 + (a.last_interaction.is_some() as f64 * 15.0);
            let score_b = b.reputation_score as f64 + (b.last_interaction.is_some() as f64 * 15.0);
            score_b.partial_cmp(&score_a).unwrap_or(std::cmp::Ordering::Equal)
        });
        list.into_iter().take(limit).collect()
    }

    pub fn get_active_contacts(&self) -> Vec<&Contact> {
        self.contacts.values().filter(|c| !c.revoked).collect()
    }

    // ==================== AUTO-ORGANISATION ====================

    pub fn decay_reputations(&mut self) {
        let now = chrono::Utc::now();
        let mut decayed = 0;
        for contact in self.contacts.values_mut() {
            if let Some(last) = contact.last_interaction {
                let days = (now - last).num_days();
                if days > 30 {
                    let decay = (days as f64 * 0.8).min(25.0) as i32;
                    contact.reputation_score = (contact.reputation_score - decay).max(0);
                    decayed += 1;
                }
            }
        }
        if decayed > 0 {
            debug!("[ContactManager] Réputation dégradée pour {} contacts", decayed);
        }
    }

    pub fn cleanup_inactive(&mut self, max_inactive_days: i64) -> usize {
        let before = self.contacts.len();
        self.contacts.retain(|_, c| {
            if c.revoked { return false; }
            if let Some(days) = c.days_since_last_interaction() {
                days < max_inactive_days
            } else { true }
        });
        before - self.contacts.len()
    }

    pub fn auto_organize(&mut self) {
        self.decay_reputations();
        let before = self.contacts.len();
        self.contacts.retain(|_, c| c.reputation_score > 8 || c.verification_level >= 2 || c.interaction_count > 5);
        let removed = before - self.contacts.len();
        if removed > 0 {
            warn!("[ContactManager] {} contacts de faible qualité nettoyés", removed);
        }
        info!("[ContactManager] Auto-organisation terminée ({} contacts restants)", self.contacts.len());
    }

    // ==================== STATISTIQUES ====================

    pub fn stats(&self) -> ContactStats {
        let total = self.contacts.len();
        let verified = self.contacts.values().filter(|c| c.verification_level >= 2).count();
        let favorites = self.favorites.len();
        let avg_reputation = if total > 0 {
            self.contacts.values().map(|c| c.reputation_score).sum::<i32>() / total as i32
        } else { 0 };

        ContactStats {
            total_contacts: total,
            verified_contacts: verified,
            favorite_contacts: favorites,
            average_reputation: avg_reputation,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ContactStats {
    pub total_contacts: usize,
    pub verified_contacts: usize,
    pub favorite_contacts: usize,
    pub average_reputation: i32,
}

impl Default for ContactManager {
    fn default() -> Self {
        Self::new()
    }
}