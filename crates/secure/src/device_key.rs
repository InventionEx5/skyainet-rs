// crates/secure/src/device_key.rs
// =====================================================
// Device Key Management v5.1 — Strong Edition + Error Handling
// SkyAInet × Nikola T369 — Signature Dilithium + Rotation + Révocation Fine
// =====================================================

use crate::crypto::dilithium::{Dilithium5Signer, DilithiumError};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use tracing::{info, debug, warn};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DeviceKeyError {
    #[error("Maximum number of devices reached")]
    MaxDevicesReached,
    #[error("Device not found")]
    DeviceNotFound,
    #[error("Device is revoked")]
    DeviceRevoked,
    #[error("Device has expired")]
    DeviceExpired,
    #[error("Signature verification failed")]
    SignatureVerificationFailed,
    #[error("Invalid public key length")]
    InvalidPublicKey,
    #[error("Dilithium error: {0}")]
    DilithiumError(#[from] DilithiumError),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum DeviceStatus {
    Active,
    Revoked,
    Expired,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeviceKey {
    pub device_id: [u8; 32],
    pub public_key: Vec<u8>,
    pub signature: Vec<u8>,
    pub created_at: DateTime<Utc>,
    pub last_rotation: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub status: DeviceStatus,
    pub rotation_count: u32,
}

pub struct DeviceKeyManager {
    pub identity_signer: Dilithium5Signer,
    pub devices: HashMap<[u8; 32], DeviceKey>,
    pub max_devices: usize,
    pub default_expiration_days: Option<u32>,
}

impl DeviceKeyManager {
    pub fn new(identity_signer: Dilithium5Signer, max_devices: usize) -> Self {
        Self {
            identity_signer,
            devices: HashMap::new(),
            max_devices,
            default_expiration_days: Some(365),
        }
    }

    /// Enregistre un nouvel appareil avec signature Dilithium
    pub fn register_device(&mut self, device_public_key: Vec<u8>) -> Result<DeviceKey, DeviceKeyError> {
        if self.devices.len() >= self.max_devices {
            return Err(DeviceKeyError::MaxDevicesReached);
        }

        if device_public_key.len() != 32 {
            return Err(DeviceKeyError::InvalidPublicKey);
        }

        let device_id: [u8; 32] = rand::random();
        let signature = self.identity_signer.sign(&device_public_key);

        let now = Utc::now();
        let expires_at = self.default_expiration_days.map(|days| {
            now + chrono::Duration::days(days as i64)
        });

        let device_key = DeviceKey {
            device_id,
            public_key: device_public_key,
            signature,
            created_at: now,
            last_rotation: now,
            expires_at,
            status: DeviceStatus::Active,
            rotation_count: 0,
        };

        self.devices.insert(device_id, device_key.clone());

        info!(
            "[DeviceKeyManager] Nouvel appareil enregistré : {} (total: {})",
            hex::encode(&device_id[0..8]),
            self.devices.len()
        );

        Ok(device_key)
    }

    /// Révoque un appareil
    pub fn revoke_device(&mut self, device_id: &[u8; 32]) -> Result<(), DeviceKeyError> {
        let device = self.devices.get_mut(device_id).ok_or(DeviceKeyError::DeviceNotFound)?;

        if device.status == DeviceStatus::Revoked {
            return Ok(());
        }

        device.status = DeviceStatus::Revoked;
        warn!(
            "[DeviceKeyManager] Appareil révoqué : {}",
            hex::encode(&device_id[0..8])
        );
        Ok(())
    }

    /// Rotation d'une Device Key
    pub fn rotate_device_key(
        &mut self,
        device_id: &[u8; 32],
        new_public_key: Vec<u8>,
    ) -> Result<(), DeviceKeyError> {
        let device = self.devices.get_mut(device_id).ok_or(DeviceKeyError::DeviceNotFound)?;

        if device.status != DeviceStatus::Active {
            return Err(DeviceKeyError::DeviceRevoked);
        }

        if new_public_key.len() != 32 {
            return Err(DeviceKeyError::InvalidPublicKey);
        }

        device.public_key = new_public_key.clone();
        device.signature = self.identity_signer.sign(&new_public_key);
        device.last_rotation = Utc::now();
        device.rotation_count += 1;

        debug!(
            "[DeviceKeyManager] Rotation effectuée pour l’appareil {} (rotation #{})",
            hex::encode(&device_id[0..8]),
            device.rotation_count
        );

        Ok(())
    }

    /// Vérifie la validité complète d’une Device Key
    pub fn verify_device(&self, device: &DeviceKey) -> Result<bool, DeviceKeyError> {
        if device.status == DeviceStatus::Revoked {
            return Err(DeviceKeyError::DeviceRevoked);
        }

        if let Some(exp) = device.expires_at {
            if Utc::now() > exp {
                return Err(DeviceKeyError::DeviceExpired);
            }
        }

        // Vérification Dilithium réelle
        self.identity_signer
            .verify(&device.public_key, &device.signature)
            .map_err(|_| DeviceKeyError::SignatureVerificationFailed)?;

        Ok(true)
    }

    /// Vérifie si un appareil est valide (statut + expiration + signature)
    pub fn is_device_valid(&self, device_id: &[u8; 32]) -> Result<bool, DeviceKeyError> {
        let device = self.devices.get(device_id).ok_or(DeviceKeyError::DeviceNotFound)?;
        self.verify_device(device)
    }

    /// Récupère un appareil par son ID
    pub fn get_device(&self, device_id: &[u8; 32]) -> Option<&DeviceKey> {
        self.devices.get(device_id)
    }

    /// Récupère un appareil mutable (pour rotation manuelle par exemple)
    pub fn get_device_mut(&mut self, device_id: &[u8; 32]) -> Result<&mut DeviceKey, DeviceKeyError> {
        self.devices.get_mut(device_id).ok_or(DeviceKeyError::DeviceNotFound)
    }

    /// Liste tous les appareils actifs
    pub fn get_active_devices(&self) -> Vec<&DeviceKey> {
        self.devices
            .values()
            .filter(|d| d.status == DeviceStatus::Active)
            .collect()
    }

    /// Nettoie les appareils expirés ou révoqués
    pub fn cleanup_expired(&mut self) -> usize {
        let now = Utc::now();
        let before = self.devices.len();

        self.devices.retain(|_, device| {
            if device.status == DeviceStatus::Revoked {
                return false;
            }
            if let Some(exp) = device.expires_at {
                if now > exp {
                    return false;
                }
            }
            true
        });

        let removed = before - self.devices.len();
        if removed > 0 {
            debug!("[DeviceKeyManager] {} appareils expirés/révoqués nettoyés", removed);
        }
        removed
    }

    /// Retourne le nombre total d’appareils
    pub fn device_count(&self) -> usize {
        self.devices.len()
    }
}