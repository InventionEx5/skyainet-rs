// crates/secure/src/group/sender_keys.rs
// =====================================================
// Sender Keys v6.3 — Group Messaging Sécurisé
// Compatible avec Contact v6.2 + DID + messaging.html
// SkyAInet × Nikola T369
// =====================================================

use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use tracing::{info, debug, warn};
use thiserror::Error;

use hkdf::Hkdf;
use sha2::Sha256;

use crate::contacts::contact::Contact;
use crate::contacts::manager::ContactManager;
use crate::crypto::gematria_aead::GematriaAead;
use crate::crypto::roman_t369::{RomanT369, GematriaMode};

#[derive(Error, Debug)]
pub enum GroupError {
    #[error("Group not found")]
    GroupNotFound,
    #[error("Member not found in group")]
    MemberNotFound,
    #[error("Contact is not verified or has no DID")]
    ContactNotVerified,
    #[error("Maximum members reached")]
    MaxMembersReached,
    #[error("Encryption failed")]
    EncryptionFailed,
    #[error("Decryption failed")]
    DecryptionFailed,
    #[error("Sender key not initialized")]
    SenderKeyNotInitialized,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Group {
    pub group_id: [u8; 16],
    pub name: String,
    pub description: Option<String>,
    pub creator: [u8; 32],
    pub members: Vec<[u8; 32]>,
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

    /// Crée un nouveau groupe
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

        info!("[GroupManager] Groupe créé : {}", hex::encode(&group_id[0..4]));
        Ok(group_id)
    }

    /// Ajoute un membre (exige DID + vérification niveau 2+)
    pub fn add_member(&mut self, group_id: &[u8; 16], contact: &Contact) -> Result<(), GroupError> {
        let group = self.groups.get_mut(group_id).ok_or(GroupError::GroupNotFound)?;

        if group.members.len() >= self.max_members_per_group {
            return Err(GroupError::MaxMembersReached);
        }

        // Vérification renforcée via ContactManager + DID
        if !self.contact_manager.can_join_group(&contact.node_id) {
            return Err(GroupError::ContactNotVerified);
        }

        if group.members.contains(&contact.node_id) {
            return Ok(());
        }

        group.members.push(contact.node_id);
        group.last_activity = Utc::now();

        // Initialise la Sender Key
        let initial_key: [u8; 32] = rand::random();
        self.sender_keys
            .entry(*group_id)
            .or_default()
            .insert(contact.node_id, initial_key);

        debug!("[GroupManager] Membre ajouté au groupe {}", hex::encode(&group_id[0..4]));
        Ok(())
    }

    /// Envoie un message chiffré dans le groupe (Sender Key + GematriaAead)
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
            .get_mut(group_id)
            .and_then(|keys| keys.get_mut(sender_node_id))
            .ok_or(GroupError::SenderKeyNotInitialized)?;

        // Dérivation de la clé de message (style Sender Keys)
        let mut message_key = [0u8; 32];
        let hk = Hkdf::<Sha256>::new(None, chain_key);
        hk.expand(b"message-key", &mut message_key).expect("HKDF failed");

        let aead = GematriaAead::new(message_key, [0u8; 12]);
        let encrypted = aead.encrypt(plaintext);

        // Rotation légère de la chaîne (Sender Key rotation)
        let mut new_chain = [0u8; 32];
        let hk2 = Hkdf::<Sha256>::new(None, chain_key);
        hk2.expand(b"next-chain-key", &mut new_chain).expect("HKDF rotation failed");
        *chain_key = new_chain;

        debug!("[GroupManager] Message envoyé dans le groupe {}", hex::encode(&group_id[0..4]));
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
            .ok_or(GroupError::SenderKeyNotInitialized)?;

        let mut message_key = [0u8; 32];
        let hk = Hkdf::<Sha256>::new(None, chain_key);
        hk.expand(b"message-key", &mut message_key).expect("HKDF failed");

        let aead = GematriaAead::new(message_key, [0u8; 12]);
        let decrypted = aead.decrypt(ciphertext).ok_or(GroupError::DecryptionFailed)?;

        Ok(decrypted)
    }

    /// Rotation explicite des Sender Keys du groupe
    pub fn rotate_sender_keys(&mut self, group_id: &[u8; 16]) -> Result<(), GroupError> {
        let group = self.groups.get_mut(group_id).ok_or(GroupError::GroupNotFound)?;
        let keys = self.sender_keys.get_mut(group_id).ok_or(GroupError::GroupNotFound)?;

        for (node_id, chain_key) in keys.iter_mut() {
            let mut new_key = [0u8; 32];
            let hk = Hkdf::<Sha256>::new(Some(b"GROUP-ROTATION"), chain_key);
            hk.expand(b"next-epoch-key", &mut new_key).expect("HKDF rotation failed");
            *chain_key = new_key;
        }

        group.epoch += 1;
        group.last_activity = Utc::now();

        info!("[GroupManager] Rotation d'epoch effectuée pour le groupe {} → Epoch {}", 
              hex::encode(&group_id[0..4]), group.epoch);
        Ok(())
    }

    pub fn get_group(&self, group_id: &[u8; 16]) -> Option<&Group> {
        self.groups.get(group_id)
    }

    pub fn list_groups(&self) -> Vec<&Group> {
        self.groups.values().collect()
    }

    pub fn remove_member(&mut self, group_id: &[u8; 16], node_id: &[u8; 32]) -> Result<(), GroupError> {
        let group = self.groups.get_mut(group_id).ok_or(GroupError::GroupNotFound)?;
        group.members.retain(|id| id != node_id);
        if let Some(keys) = self.sender_keys.get_mut(group_id) {
            keys.remove(node_id);
        }
        Ok(())
    }
}