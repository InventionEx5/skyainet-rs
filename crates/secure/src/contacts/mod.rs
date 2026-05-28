// crates/secure/src/contacts/mod.rs
// =====================================================
// Contacts Module — SkyAInet Secure Layer
// =====================================================

pub mod contact;
pub mod verification;
pub mod manager;

// =====================================================
// RÉ-EXPORTS PUBLICS (pour un usage facile)
// =====================================================

pub use contact::Contact;
pub use verification::{ContactVerification, VerificationLevel, VerificationError};
pub use manager::{ContactManager, ContactManagerError, ContactStats};