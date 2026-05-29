// crates/secure/src/group/sender_keys.rs
// =====================================================
// Sender Keys v5.2 — Group Messaging Sécurisé
// Compatible avec Contact System + messaging.html
// SkyAInet × Nikola T369
// =====================================================

use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use tracing::{info, debug, warn};
use thiserror::Error;

use crate::crypto::gematria_aead::GematriaAead;
use crate::crypto::roman_t369::{RomanT369, GematriaMode};
use crate::contact::contact::Contact;
use crate::contact::manager::ContactManager;

#[derive(Error, Debug)]
pub enum GroupError {
    #[error("Group not found")]
    GroupNotFound,
    #[error("Member not found in group")]
    MemberNotFound,
    #[error("Contact is not verified")]
    ContactNotVerified,
    #[error("Maximum members reached")]
    MaxMembersReached,
    #[error("Encryption failed")]
    EncryptionFailed,
    #[error("Decryption failed")]
    DecryptionFailed,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Group {
    pub group_id: [u8; 16],
    pub name: String,
    pub description: Option<String>,
    pub creator: [u8; 32],
    pub members: Vec<[u8; 32]>,           // node_id des contacts
    pub created_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub epoch: u64,
}

pub struct GroupManager {
    groups: HashMap<[u8; 16], Group>,
    sender_keys: HashMap<[u8; 16], HashMap<[u8; 32], [u8; 32]>>, // group_id → node_id → chain_key
    contact_manager: ContactManager,
    max_members_per_group: usize,
    roman: RomanT369,
}

impl GroupManager {
    pub fn new(contact_manager: ContactManager) -> Self {
        Self {
            groups: HashMap::new(),
            sender_keys: HashMap::new(),
            contact_manager,
            max_members_per_group: 50,
            roman: RomanT369::new([0x42u8; 32], [0u8; 12], GematriaMode::Hyper256),
        }
    }

    /// Crée un nouveau groupe (compatible avec messaging.html)
    pub fn create_group(
        &mut self,
        creator_node_id: [u8; 32],
        name: String,
        description: Option<String>,
    ) -> Result<[u8; 16], GroupError> {
        let group_id: [u8; 16] = rand::random();

        let group = Group {
            group_id,
            name,
            description,
            creator: creator_node_id,
            members: vec![creator_node_id],
            created_at: Utc::now(),
            last_activity: Utc::now(),
            epoch: 0,
        };

        self.groups.insert(group_id, group);
        self.sender_keys.insert(group_id, HashMap::new());

        info!("[GroupManager] Groupe créé : {} ({:?})", group_id[0], group_id);
        Ok(group_id)
    }

    /// Ajoute un membre au groupe (doit être un contact vérifié)
    pub fn add_member(&mut self, group_id: &[u8; 16], contact: &Contact) -> Result<(), GroupError> {
        let group = self.groups.get_mut(group_id).ok_or(GroupError::GroupNotFound)?;

        if group.members.len() >= self.max_members_per_group {
            return Err(GroupError::MaxMembersReached);
        }

        if !contact.is_trusted() {
            return Err(GroupError::ContactNotVerified);
        }

        if group.members.contains(&contact.node_id) {
            return Ok(()); // déjà membre
        }

        group.members.push(contact.node_id);
        group.last_activity = Utc::now();

        // Initialise une Sender Key pour le nouveau membre
        let initial_key: [u8; 32] = rand::random();
        self.sender_keys
            .entry(*group_id)
            .or_default()
            .insert(contact.node_id, initial_key);

        debug!("[GroupManager] Membre ajouté au groupe {:?}", group_id);
        Ok(())
    }

    /// Envoie un message chiffré dans le groupe
    pub fn send_group_message(
        &mut self,
        group_id: &[u8; 16],
        sender_node_id: &[u8; 32],
        plaintext: &[u8],
    ) -> Result<Vec<u8>, GroupError> {
        let group = self.groups.get(group_id).ok_or(GroupError::GroupNotFound)?;

        if !group.members.contains(sender_node_id) {
            return Err(GroupError::MemberNotFound);
        }

        let chain_key = self.sender_keys
            .get(group_id)
            .and_then(|keys| keys.get(sender_node_id))
            .ok_or(GroupError::MemberNotFound)?;

        // Dérivation de la clé de message
        let mut message_key = [0u8; 32];
        let hk = hkdf::Hkdf::<sha2::Sha256>::new(None, chain_key);
        hk.expand(b"message-key", &mut message_key).expect("HKDF failed");

        // Chiffrement avec GematriaAead
        let aead = GematriaAead::new(message_key, [0u8; 12]);
        let encrypted = aead.encrypt(plaintext);

        // Mise à jour de la chaîne (rotation légère)
        // TODO: Implémenter vraie rotation Sender Key

        debug!("[GroupManager] Message envoyé dans le groupe {:?}", group_id);
        Ok(encrypted)
    }

    /// Déchiffre un message de groupe
    pub fn decrypt_group_message(
        &self,
        group_id: &[u8; 16],
        sender_node_id: &[u8; 32],
        ciphertext: &[u8],
    ) -> Result<Vec<u8>, GroupError> {
        let chain_key = self.sender_keys
            .get(group_id)
            .and_then(|keys| keys.get(sender_node_id))
            .ok_or(GroupError::MemberNotFound)?;

        let mut message_key = [0u8; 32];
        let hk = hkdf::Hkdf::<sha2::Sha256>::new(None, chain_key);
        hk.expand(b"message-key", &mut message_key).expect("HKDF failed");

        let aead = GematriaAead::new(message_key, [0u8; 12]);
        let decrypted = aead.decrypt(ciphertext).ok_or(GroupError::DecryptionFailed)?;

        Ok(decrypted)
    }

    /// Récupère les informations d'un groupe
    pub fn get_group(&self, group_id: &[u8; 16]) -> Option<&Group> {
        self.groups.get(group_id)
    }

    /// Liste tous les groupes
    pub fn list_groups(&self) -> Vec<&Group> {
        self.groups.values().collect()
    }

    /// Supprime un membre du groupe
    pub fn remove_member(&mut self, group_id: &[u8; 16], node_id: &[u8; 32]) -> Result<(), GroupError> {
        let group = self.groups.get_mut(group_id).ok_or(GroupError::GroupNotFound)?;
        group.members.retain(|id| id != node_id);
        self.sender_keys.get_mut(group_id).map(|keys| keys.remove(node_id));
        Ok(())
    }
}