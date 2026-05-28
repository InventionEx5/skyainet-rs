// crates/secure-transport/src/crypto/gematria/roman_t369.rs
// =====================================================
// Roman T369 v3.2 — Chiffrement Hyper Sécurisé & Ultra Rapide
// Innovation : Roman Weighted Diffusion + Hyper256 + Logique Additive/Soustractive
// =====================================================

use sha2::{Sha256, Digest};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GematriaMode {
    Dynamic,
    Extended,
    Hyper256,
}

#[derive(Clone)]
pub struct RomanT369 {
    key: [u8; 32],
    nonce: [u8; 12],
    modulus: u16,
    permutation: [u8; 256],
    mode: GematriaMode,
    hyper_lookup: Option<[u8; 256]>,
    roman_weights: [u8; 7],
}

impl RomanT369 {
    pub fn new(key: [u8; 32], nonce: [u8; 12], mode: GematriaMode) -> Self {
        let modulus = match mode {
            GematriaMode::Dynamic => 95,
            GematriaMode::Extended => 128,
            GematriaMode::Hyper256 => 256,
        };

        let permutation = Self::generate_constant_permutation(&key, &nonce, modulus as u8);
        let hyper_lookup = if mode == GematriaMode::Hyper256 {
            Some(Self::precompute_hyper_lookup(&permutation))
        } else {
            None
        };

        let roman_weights = [1, 5, 10, 50, 100, 200, 250];

        Self {
            key,
            nonce,
            modulus: modulus as u16,
            permutation,
            mode,
            hyper_lookup,
            roman_weights,
        }
    }

    fn generate_constant_permutation(key: &[u8; 32], nonce: &[u8; 12], modulus: u8) -> [u8; 256] {
        let mut table = [0u8; 256];
        for i in 0..256u16 {
            let mut hasher = Sha256::new();
            hasher.update(key);
            hasher.update(nonce);
            hasher.update(&i.to_le_bytes());
            let hash = hasher.finalize();
            table[i as usize] = hash[0] % modulus;
        }
        table
    }

    fn precompute_hyper_lookup(permutation: &[u8; 256]) -> [u8; 256] {
        let mut lookup = [0u8; 256];
        for i in 0..256 {
            lookup[i] = permutation[i];
        }
        lookup
    }

    // =====================================================
    // INNOVATION : Roman Weighted Diffusion (Hyper Puissant)
    // =====================================================

    #[inline(always)]
    fn roman_diffuse(&self, byte: u8, position: usize) -> u8 {
        let roman_idx = (byte as usize + position) % 7;
        let weight = self.roman_weights[roman_idx];

        if (byte + position as u8) % 3 == 0 {
            byte.wrapping_sub(weight).rotate_right(2)
        } else if (byte + position as u8) % 3 == 1 {
            byte.wrapping_add(weight).rotate_left(3)
        } else {
            (byte ^ weight).rotate_left(1)
        }
    }

    #[inline(always)]
    fn roman_undiffuse(&self, byte: u8, position: usize) -> u8 {
        let roman_idx = (byte as usize + position) % 7;
        let weight = self.roman_weights[roman_idx];

        if (byte + position as u8) % 3 == 0 {
            byte.wrapping_add(weight).rotate_left(2)
        } else if (byte + position as u8) % 3 == 1 {
            byte.wrapping_sub(weight).rotate_right(3)
        } else {
            (byte ^ weight).rotate_right(1)
        }
    }

    // =====================================================
    // CHIFFREMENT / DÉCHIFFREMENT
    // =====================================================

    pub fn encrypt(&self, plaintext: &[u8]) -> Vec<u8> {
        let mut ciphertext = Vec::with_capacity(plaintext.len());

        if let Some(lookup) = &self.hyper_lookup {
            for (i, &byte) in plaintext.iter().enumerate() {
                let k = lookup[byte as usize];
                let diffused = self.roman_diffuse(k, i);
                ciphertext.push(diffused);
            }
        } else {
            for (i, &byte) in plaintext.iter().enumerate() {
                let k = self.permutation[byte as usize];
                let c = ((byte as u16 + k as u16) % self.modulus) as u8;
                let diffused = self.roman_diffuse(c, i);
                ciphertext.push(diffused);
            }
        }

        ciphertext
    }

    pub fn decrypt(&self, ciphertext: &[u8]) -> Option<Vec<u8>> {
        let mut plaintext = Vec::with_capacity(ciphertext.len());

        if let Some(lookup) = &self.hyper_lookup {
            for (i, &c) in ciphertext.iter().enumerate() {
                let undiffused = self.roman_undiffuse(c, i);
                let k = lookup[undiffused as usize];
                plaintext.push(k);
            }
        } else {
            for (i, &c) in ciphertext.iter().enumerate() {
                let undiffused = self.roman_undiffuse(c, i);
                let k = self.permutation[undiffused as usize];
                let m = ((undiffused as u16 + self.modulus - k as u16) % self.modulus) as u8;
                plaintext.push(m);
            }
        }

        Some(plaintext)
    }

    // =====================================================
    // ALPHABET 256 (avec les 7 chiffres romains)
    // =====================================================

    pub const ROMAN_T369_ALPHABET: [char; 256] = [
        // 52 Lettres latines
        'A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z',
        'a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z',

        // 10 Chiffres
        '0','1','2','3','4','5','6','7','8','9',

        // 33 Cyrilliques + 24 Grecques
        'А','Б','В','Г','Д','Е','Ё','Ж','З','И','Й','К','Л','М','Н','О','П','Р','С','Т','У','Ф','Х','Ц','Ч','Ш','Щ','Ъ','Ы','Ь','Э','Ю','Я',
        'Α','Β','Γ','Δ','Ε','Ζ','Η','Θ','Ι','Κ','Λ','Μ','Ν','Ξ','Ο','Π','Ρ','Σ','Τ','Υ','Φ','Χ','Ψ','Ω',

        // =====================================================
        // 7 CHIFFRES ROMAINS (Cœur du système)
        // =====================================================
        'I','V','X','L','C','D','M',

        // 2 symboles + 128 caractères internationaux
        '⁂','⁑',
        'ا','ب','ت','ث','ج','ح','خ','د','ذ','ر','ز','س','ش','ص','ض','ط','ظ','ع','غ','ف','ق','ك','ل','م','ن','ه','و','ي',
        'א','ב','ג','ד','ה','ו','ז','ח','ט','י','כ','ל','מ','נ','ס','ע','פ','צ','ק','ר','ש','ת',
        'ა','ბ','გ','د','ე','ვ','ზ','თ','ი','კ','ლ','მ','ნ','ო','პ','ჟ','რ','ს','ტ','უ','ფ','ქ','ღ','ყ','შ','ჩ','ც','ძ','წ','ჭ','ხ','ჯ','ჰ',
        'Ա','Բ','Գ','Դ','Ե','Զ','Է','Ը','Թ','Ժ','Ի','Լ','Խ','Ծ','Կ','Հ','Ձ','Ղ','Ճ','Մ','Յ','Ն','Շ','Ո','Չ','Պ','Ջ','Ռ','Ս','Վ','Տ','Ր','Ց',
        'अ','आ','इ','ई','उ','ऊ','ऋ','ए','ऐ','ओ','औ','क','ख','ग','घ','च','छ','ज','झ','ट',
        'ก','ข','ค','ฆ','ง','จ','ฉ','ช','ซ','ฌ',
        '∞','∑','∏','√','∫','∂','∇','∆','≈','≠','≤','≥',
    ];
}