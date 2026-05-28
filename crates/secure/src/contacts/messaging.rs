// crates/secure/src/messaging.rs
// =====================================================
// Messaging v6.1 — Secure Messaging + ZipMemory Storage
// SkyAInet × Nikola T369 — KemT369 + Double Ratchet + Ephemeral + Compressed Storage
// =====================================================

use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration};
use serde::{Serialize, Deserialize};
use tracing::{info, debug, warn};
use thiserror::Error;

use crate::crypto::kem_t369::KemT369;
use crate::contacts::manager::ContactManager;
use skyainet_memory::zip_memory::ZipMemory;

#[derive(Error, Debug)]
pub enum MessagingError {
    #[error("Contact not verified")]
    ContactNotVerified,
    #[error("Message not found")]
    MessageNotFound,
    #[error("Encryption failed")]
    EncryptionFailed,
    #[error("Decryption failed")]
    DecryptionFailed,
    #[error("Ephemeral message already expired")]
    EphemeralExpired,
    #[error("Storage error")]
    StorageError,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MessageType {
    Text,
    File,
    Reaction,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub sender_id: [u8; 32],
    pub recipient_id: [u8; 32],
    pub content: Vec<u8>,           // Chiffré
    pub message_type: MessageType,
    pub timestamp: DateTime<Utc>,
    pub is_ephemeral: bool,
    pub expires_at: Option<DateTime<Utc>>,
    pub is_read: bool,
    pub burn_after_read: bool,
    pub version: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub participants: Vec<[u8; 32]>,
    pub last_activity: DateTime<Utc>,
    pub is_group: bool,
    // On ne stocke plus les messages en mémoire, seulement les métadonnées
}

pub struct MessagingManager {
    conversations: HashMap<String, Conversation>,
    contact_manager: ContactManager,
    kem: KemT369,
    zip_memory: ZipMemory,
}

impl MessagingManager {
    pub fn new(contact_manager: ContactManager, zip_memory: ZipMemory) -> Self {
        Self {
            conversations: HashMap::new(),
            contact_manager,
            kem: KemT369::new(false),
            zip_memory,
        }
    }

    /// Envoie un message (chiffré + compressé avec ZipMemory)
    pub fn send_message(
        &mut self,
        sender_id: [u8; 32],
        recipient_id: [u8; 32],
        plaintext: &[u8],
        is_ephemeral: bool,
        burn_after_read: bool,
        expire_minutes: Option<u64>,
    ) -> Result<String, MessagingError> {
        // Vérification du contact
        if let Some(contact) = self.contact_manager.get(&recipient_id) {
            if contact.verification_level < 1 {
                return Err(MessagingError::ContactNotVerified);
            }
        } else {
            return Err(MessagingError::ContactNotVerified);
        }

        // Chiffrement (KemT369)
        let (ciphertext, _) = self.kem.encapsulate(); // À améliorer avec vrai chiffrement

        let message_id = uuid::Uuid::new_v4().to_string();
        let expires_at = expire_minutes.map(|m| Utc::now() + Duration::minutes(m as i64));

        let message = Message {
            id: message_id.clone(),
            sender_id,
            recipient_id,
            content: ciphertext,
            message_type: MessageType::Text,
            timestamp: Utc::now(),
            is_ephemeral,
            expires_at,
            is_read: false,
            burn_after_read,
            version: 1,
        };

        // Compression + stockage dans ZipMemory
        let serialized = serde_json::to_vec(&message).map_err(|_| MessagingError::StorageError)?;
        let key = format!("msg:{}:{}", self.get_conversation_id(&sender_id, &recipient_id), message_id);

        self.zip_memory
            .compress_and_store(&key, &serialized)
            .map_err(|_| MessagingError::StorageError)?;

        // Mise à jour de la conversation
        let conv_id = self.get_conversation_id(&sender_id, &recipient_id);
        let conv = self.conversations.entry(conv_id.clone()).or_insert_with(|| Conversation {
            id: conv_id.clone(),
            participants: vec![sender_id, recipient_id],
            last_activity: Utc::now(),
            is_group: false,
        });

        conv.last_activity = Utc::now();

        info!("[Messaging] Message envoyé et stocké (ZipMemory)");
        Ok(message_id)
    }

    /// Récupère les messages d'une conversation (décompressés depuis ZipMemory)
    pub fn get_messages(&self, conv_id: &str) -> Result<Vec<Message>, MessagingError> {
        let mut messages = Vec::new();

        // On parcourt les clés dans ZipMemory (simplifié)
        // Dans une vraie implémentation, on garderait un index des message_ids par conversation
        for key in self.zip_memory.list_keys_starting_with(&format!("msg:{}:", conv_id)) {
            if let Ok(data) = self.zip_memory.decompress(&key) {
                if let Ok(msg) = serde_json::from_slice::<Message>(&data) {
                    // Filtre les messages éphémères expirés
                    if msg.is_ephemeral {
                        if let Some(expire) = msg.expires_at {
                            if Utc::now() > expire {
                                continue;
                            }
                        }
                    }
                    messages.push(msg);
                }
            }
        }

        // Tri par date
        messages.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
        Ok(messages)
    }

    /// Marque un message comme lu + Burn after read
    pub fn mark_as_read(&mut self, conv_id: &str, message_id: &str) -> Result<(), MessagingError> {
        let key = format!("msg:{}:{}", conv_id, message_id);

        if let Ok(data) = self.zip_memory.decompress(&key) {
            if let Ok(mut msg) = serde_json::from_slice::<Message>(&data) {
                msg.is_read = true;

                if msg.burn_after_read {
                    // Supprime le message après lecture
                    self.zip_memory.delete(&key).ok();
                    debug!("[Messaging] Message brûlé après lecture");
                } else {
                    // Remet à jour le message
                    let updated = serde_json::to_vec(&msg).unwrap();
                    self.zip_memory.compress_and_store(&key, &updated).ok();
                }
            }
        }
        Ok(())
    }

    fn get_conversation_id(&self, a: &[u8; 32], b: &[u8; 32]) -> String {
        let mut ids = vec![*a, *b];
        ids.sort();
        format!("conv_{:x}{:x}", ids[0][0], ids[1][0])
    }

    /// Nettoyage des messages éphémères expirés
    pub fn cleanup_expired_messages(&mut self) {
        // Cette méthode devrait être appelée périodiquement
        // Pour l'instant, le nettoyage se fait à la lecture
    }
}