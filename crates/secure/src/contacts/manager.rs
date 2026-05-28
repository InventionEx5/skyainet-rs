// crates/secure/src/contacts/manager.rs
// =====================================================
// ContactManager v6.0 — Gestion Intelligente des Contacts
// SkyAInet × Nikola T369 — Auto-Organisation + Réputation + Vérification
// Version Ultra Améliorée (Production Ready)
// =====================================================

use super::contact::Contact;
use super::verification::ContactVerification;
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

    /// Ajoute ou met à jour un contact
    pub fn add_or_update(&mut self, mut contact: Contact) -> Result<(), ContactManagerError> {
        if self.contacts.len() >= self.max_contacts && !self.contacts.contains_key(&contact.node_id) {
            return Err(ContactManagerError::MaxContactsReached);
        }

        let is_new = !self.contacts.contains_key(&contact.node_id);
        self.contacts.insert(contact.node_id, contact.clone());

        if is_new {
            info!(
                "[ContactManager] Nouveau contact ajouté : {} (total: {})",
                contact.name, self.contacts.len()
            );
        } else {
            debug!("[ContactManager] Contact mis à jour : {}", contact.name);
        }

        Ok(())
    }

    /// Récupère un contact
    pub fn get(&self, node_id: &[u8; 32]) -> Option<&Contact> {
        self.contacts.get(node_id)
    }

    /// Supprime un contact
    pub fn remove(&mut self, node_id: &[u8; 32]) -> Result<(), ContactManagerError> {
        if self.contacts.remove(node_id).is_some() {
            self.favorites.retain(|id| id != node_id);
            debug!("[ContactManager] Contact supprimé");
            Ok(())
        } else {
            Err(ContactManagerError::ContactNotFound)
        }
    }

    /// Ajoute / retire des favoris
    pub fn toggle_favorite(&mut self, node_id: &[u8; 32]) -> Result<(), ContactManagerError> {
        if !self.contacts.contains_key(node_id) {
            return Err(ContactManagerError::ContactNotFound);
        }

        if self.favorites.contains(node_id) {
            self.favorites.retain(|id| id != node_id);
            debug!("[ContactManager] Retiré des favoris");
        } else {
            self.favorites.push(*node_id);
            debug!("[ContactManager] Ajouté aux favoris");
        }
        Ok(())
    }

    /// Trie les contacts par réputation (du plus haut au plus bas)
    pub fn get_sorted_by_reputation(&self) -> Vec<&Contact> {
        let mut list: Vec<&Contact> = self.contacts.values().collect();
        list.sort_by(|a, b| b.reputation_score.cmp(&a.reputation_score));
        list
    }

    /// Trie les contacts par dernière interaction (les plus récents en premier)
    pub fn get_sorted_by_last_interaction(&self) -> Vec<&Contact> {
        let mut list: Vec<&Contact> = self.contacts.values().collect();
        list.sort_by(|a, b| b.last_interaction.cmp(&a.last_interaction));
        list
    }

    /// Retourne les contacts favoris
    pub fn get_favorites(&self) -> Vec<&Contact> {
        self.favorites
            .iter()
            .filter_map(|id| self.contacts.get(id))
            .collect()
    }

    /// Recherche par nom (insensible à la casse)
    pub fn search_by_name(&self, query: &str) -> Vec<&Contact> {
        let q = query.to_lowercase();
        self.contacts
            .values()
            .filter(|c| c.name.to_lowercase().contains(&q))
            .collect()
    }

    /// Retourne les contacts recommandés (réputation élevée + récemment actifs)
    pub fn get_recommended_contacts(&self, limit: usize) -> Vec<&Contact> {
        let mut list: Vec<&Contact> = self.contacts.values().collect();

        list.sort_by(|a, b| {
            let score_a = a.reputation_score as f64 + (a.last_interaction.is_some() as f64 * 15.0);
            let score_b = b.reputation_score as f64 + (b.last_interaction.is_some() as f64 * 15.0);
            score_b.partial_cmp(&score_a).unwrap_or(std::cmp::Ordering::Equal)
        });

        list.into_iter().take(limit).collect()
    }

    /// Applique une dégradation de réputation sur les contacts inactifs
    pub fn decay_reputations(&mut self) {
        let now = chrono::Utc::now();
        let mut decayed = 0;

        for contact in self.contacts.values_mut() {
            if let Some(last) = contact.last_interaction {
                let days_inactive = (now - last).num_days();
                if days_inactive > 30 {
                    let decay = (days_inactive as f64 * 0.8).min(25.0) as i32;
                    contact.reputation_score = (contact.reputation_score - decay).max(0);
                    decayed += 1;
                }
            }
        }

        if decayed > 0 {
            debug!("[ContactManager] Réputation dégradée pour {} contacts inactifs", decayed);
        }
    }

    /// Auto-organisation intelligente (tri + nettoyage + optimisation)
    pub fn auto_organize(&mut self) {
        self.decay_reputations();

        let before = self.contacts.len();

        // Nettoyage des contacts de très faible qualité
        self.contacts.retain(|_, c| {
            c.reputation_score > 8 || c.verification_level >= 2 || c.interaction_count > 5
        });

        let removed = before - self.contacts.len();

        if removed > 0 {
            warn!("[ContactManager] {} contacts de faible qualité nettoyés", removed);
        }

        info!("[ContactManager] Auto-organisation terminée ({} contacts restants)", self.contacts.len());
    }

    /// Statistiques du gestionnaire
    pub fn stats(&self) -> ContactStats {
        let total = self.contacts.len();
        let verified = self.contacts.values().filter(|c| c.verification_level >= 2).count();
        let favorites = self.favorites.len();

        let avg_reputation = if total > 0 {
            self.contacts.values().map(|c| c.reputation_score).sum::<i32>() / total as i32
        } else {
            0
        };

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