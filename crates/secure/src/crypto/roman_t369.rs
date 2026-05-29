// crates/secure/src/crypto/roman_t369.rs
// =====================================================
// RomanT369 v5.0 — STRONG EDITION
// 7 Rounds Roman + Roman S-Box + Dominant Post-Quantique Amélioré
// Sécurité maximale + Performance optimale
// SkyAInet × Nikola T369
// =====================================================

use sha2::{Sha256, Digest};
use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GematriaMode {
    Dynamic,
    Extended,
    Hyper256,
}

#[derive(Error, Debug)]
pub enum RomanError {
    #[error("Decryption failed")]
    DecryptionFailed,
}

#[derive(Clone)]
pub struct RomanT369 {
    key: [u8; 32],
    nonce: [u8; 12],
    modulus: u16,
    permutation: [u8; 256],
    mode: GematriaMode,
    hyper_lookup: Option<[u8; 256]>,
    hyper_lookup_inverse: Option<[u8; 256]>,
    roman_weights: [u8; 7],
    roman_sbox: [u8; 256],
    roman_sbox_inverse: [u8; 256],
}

impl RomanT369 {
    pub fn new(key: [u8; 32], nonce: [u8; 12], mode: GematriaMode) -> Self {
        let modulus = match mode {
            GematriaMode::Dynamic => 95,
            GematriaMode::Extended => 128,
            GematriaMode::Hyper256 => 256,
        };

        let permutation = Self::generate_constant_permutation(&key, &nonce, modulus as u8);

        let (hyper_lookup, hyper_lookup_inverse) = if mode == GematriaMode::Hyper256 {
            let l = Self::precompute_hyper_lookup(&permutation);
            let inv = Self::precompute_inverse(&l);
            (Some(l), Some(inv))
        } else {
            (None, None)
        };

        let roman_sbox = Self::precompute_roman_sbox(&key, &permutation);
        let roman_sbox_inverse = Self::precompute_inverse(&roman_sbox);

        Self {
            key,
            nonce,
            modulus: modulus as u16,
            permutation,
            mode,
            hyper_lookup,
            hyper_lookup_inverse,
            roman_weights: [1, 5, 10, 50, 100, 200, 250],
            roman_sbox,
            roman_sbox_inverse,
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
        let mut used = [false; 256];
        for i in 0..256 {
            let mut candidate = permutation[i];
            while used[candidate as usize] {
                candidate = candidate.wrapping_add(1);
            }
            used[candidate as usize] = true;
            lookup[i] = candidate;
        }
        lookup
    }

    fn precompute_inverse(lookup: &[u8; 256]) -> [u8; 256] {
        let mut inv = [0u8; 256];
        for (i, &v) in lookup.iter().enumerate() {
            inv[v as usize] = i as u8;
        }
        inv
    }

    // =====================================================
    // ROMAN S-BOX FORT (remplace l'ancienne S-Box faible)
    // =====================================================
    fn precompute_roman_sbox(key: &[u8; 32], permutation: &[u8; 256]) -> [u8; 256] {
        let mut table = [0u8; 256];
        for byte in 0..256u8 {
            let mut val = byte;
            for round in 0..7 {
                let idx = (val as usize + round * 17) % 7;
                let weight = [1, 5, 10, 50, 100, 200, 250][idx];
                let phase = (val as usize + round) % 3;

                val = match phase {
                    0 => val.wrapping_sub(weight).rotate_right(2 + (round % 3) as u32),
                    1 => val.wrapping_add(weight).rotate_left(3 + (round % 2) as u32),
                    _ => (val ^ weight).rotate_left(1 + (round % 4) as u32),
                };
            }
            val ^= permutation[val as usize];
            val = val.wrapping_add(key[val as usize % 32]);
            table[byte as usize] = val;
        }
        table
    }

    // =====================================================
    // DOMINANT POST-QUANTIQUE AMÉLIORÉ (3 octets précédents)
    // =====================================================
    #[inline(always)]
    fn get_dominant_character(&self, val: u8, position: usize, prev1: u8, prev2: u8, prev3: u8) -> u8 {
        let mut state: u64 = val as u64;

        state ^= (position as u64).wrapping_mul(0x9E3779B97F4A7C15);
        state = state.rotate_left(17);
        state ^= self.key[position % 32] as u64;
        state = state.wrapping_mul(0x85EBCA77C2B2AE63);
        state ^= prev1 as u64;
        state = state.rotate_left(13);
        state ^= self.key[(position + 17) % 32] as u64;
        state = state.wrapping_add((self.key[(position + 5) % 32] as u64) << 16);
        state ^= prev2 as u64;
        state = state.rotate_left(7);
        state ^= prev3 as u64;
        state ^= (position as u64) << 32;

        (state as usize % 256) as u8
    }

    // =====================================================
    // 7 ROUNDS ROMAN WEIGHTED DIFFUSION
    // =====================================================
    #[inline(always)]
    fn roman_diffuse(&self, mut byte: u8, position: usize) -> u8 {
        for round in 0..7 {
            let idx = (byte as usize + position + round * 17) % 7;
            let weight = self.roman_weights[idx];
            let phase = (byte as usize + position + round) % 3;

            byte = match phase {
                0 => byte.wrapping_sub(weight).rotate_right(2 + (round % 3) as u32),
                1 => byte.wrapping_add(weight).rotate_left(3 + (round % 2) as u32),
                _ => (byte ^ weight).rotate_left(1 + (round % 4) as u32),
            };
        }
        byte
    }

    #[inline(always)]
    fn roman_undiffuse(&self, mut byte: u8, position: usize) -> u8 {
        for round in (0..7).rev() {
            let idx = (byte as usize + position + round * 17) % 7;
            let weight = self.roman_weights[idx];
            let phase = (byte as usize + position + round) % 3;

            byte = match phase {
                0 => byte.wrapping_add(weight).rotate_left(2 + (round % 3) as u32),
                1 => byte.wrapping_sub(weight).rotate_right(3 + (round % 2) as u32),
                _ => (byte ^ weight).rotate_right(1 + (round % 4) as u32),
            };
        }
        byte
    }

    // =====================================================
    // CHIFFREMENT v5.0 STRONG
    // =====================================================
    pub fn encrypt(&self, plaintext: &[u8]) -> Vec<u8> {
        let mut ciphertext = Vec::with_capacity(plaintext.len());
        let mut prev1 = 0u8;
        let mut prev2 = 0u8;
        let mut prev3 = 0u8;

        if let Some(lookup) = &self.hyper_lookup {
            for (i, &byte) in plaintext.iter().enumerate() {
                let mut val = lookup[byte as usize];

                val = self.roman_sbox[val as usize];
                val = self.roman_diffuse(val, i);

                let dominant = self.get_dominant_character(val, i, prev1, prev2, prev3);
                val = (val ^ dominant).rotate_right(3);
                val = val.wrapping_add((dominant as u16).wrapping_mul(29) as u8 % 256);

                ciphertext.push(val);

                prev3 = prev2;
                prev2 = prev1;
                prev1 = val;
            }
        } else {
            for (i, &byte) in plaintext.iter().enumerate() {
                let k = self.permutation[byte as usize];
                let mut val = ((byte as u16 + k as u16) % self.modulus) as u8;

                val = self.roman_sbox[val as usize];
                val = self.roman_diffuse(val, i);

                let dominant = self.get_dominant_character(val, i, prev1, prev2, prev3);
                val = (val ^ dominant).rotate_right(3);
                val = val.wrapping_add((dominant as u16).wrapping_mul(29) as u8 % 256);

                ciphertext.push(val);

                prev3 = prev2;
                prev2 = prev1;
                prev1 = val;
            }
        }
        ciphertext
    }

    // =====================================================
    // DÉCHIFFREMENT v5.0 STRONG
    // =====================================================
    pub fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>, RomanError> {
        let mut plaintext = Vec::with_capacity(ciphertext.len());
        let mut prev1 = 0u8;
        let mut prev2 = 0u8;
        let mut prev3 = 0u8;

        if let Some(lookup) = &self.hyper_lookup {
            let inv = self.hyper_lookup_inverse.as_ref().unwrap();
            for (i, &c) in ciphertext.iter().enumerate() {
                let mut val = c;

                let dominant = self.get_dominant_character(val, i, prev1, prev2, prev3);
                val = val.wrapping_sub((dominant as u16).wrapping_mul(29) as u8 % 256);
                val = (val ^ dominant).rotate_left(3);

                val = self.roman_undiffuse(val, i);
                val = self.roman_sbox_inverse[val as usize];

                let p = inv[val as usize];
                plaintext.push(p);

                prev3 = prev2;
                prev2 = prev1;
                prev1 = val;
            }
        } else {
            for (i, &c) in ciphertext.iter().enumerate() {
                let mut val = c;

                let dominant = self.get_dominant_character(val, i, prev1, prev2, prev3);
                val = val.wrapping_sub((dominant as u16).wrapping_mul(29) as u8 % 256);
                val = (val ^ dominant).rotate_left(3);

                val = self.roman_undiffuse(val, i);
                val = self.roman_sbox_inverse[val as usize];

                let k = self.permutation[val as usize];
                let m = ((val as u16 + self.modulus - k as u16) % self.modulus) as u8;
                plaintext.push(m);

                prev3 = prev2;
                prev2 = prev1;
                prev1 = val;
            }
        }
        Ok(plaintext)
    }

    // =====================================================
    // MÉTHODES D'AFFICHAGE
    // =====================================================
    pub fn to_human_readable(&self, data: &[u8]) -> String {
        data.iter()
            .map(|&b| Self::ROMAN_T369_ALPHABET[b as usize])
            .collect()
    }

    pub fn from_human_readable(&self, s: &str) -> Option<Vec<u8>> {
        let mut bytes = Vec::with_capacity(s.chars().count());
        for ch in s.chars() {
            match Self::ROMAN_T369_ALPHABET.iter().position(|&c| c == ch) {
                Some(idx) => bytes.push(idx as u8),
                None => return None,
            }
        }
        Some(bytes)
    }

    pub const ROMAN_T369_ALPHABET: [char; 256] = [
        'A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z',
        'a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z',
        '0','1','2','3','4','5','6','7','8','9',
        'А','Б','В','Г','Д','Е','Ё','Ж','З','И','Й','К','Л','М','Н','О','П','Р','С','Т','У','Ф','Х','Ц','Ч','Ш','Щ','Ъ','Ы','Ь','Э','Ю','Я',
        'Α','Β','Γ','Δ','Ε','Ζ','Η','Θ','Ι','Κ','Λ','Μ','Ν','Ξ','Ο','Π','Ρ','Σ','Τ','Υ','Φ','Χ','Ψ','Ω',
        'I','V','X','L','C','D','M',
        '⁂','⁑',
        'ا','ب','ت','ث','ج','ح','خ','د','ž','ر','ز','س','ش','ص','ض','ط','ظ','ع','غ','ف','ق','ك','ل','م','ن','ه','و','ي',
        'א','ב','ג','ד','ה','ו','ז','ח','ט','י','כ','ל','מ','נ','ס','ע','פ','צ','ק','ר','ש','ת',
        'ა','ბ','გ','د','ე','ვ','ზ','თ','ი','კ','ლ','მ','ნ','ო','პ','ჟ','ر','س','ტ','უ','ფ','ქ','ღ','ყ','შ','ჩ','ც','ძ','წ','ჭ','ხ','ჯ','ჰ',
        'Ա','Բ','Գ','Դ','Ե','Զ','Է','Ը','Թ','Ժ','Ի','Լ','Խ','Ծ','Կ','Հ','Ձ','Ղ','Ճ','Մ','Յ','Ն','Շ','Ո','Չ','Պ','Ջ','Ռ','Ս','Վ','Տ','Ր','Ց',
        'अ','आ','इ','ई','उ','ऊ','ऋ','ए','ऐ','ओ','औ','क','ख','ग','घ','च','छ','ج','झ','ट',
        'ก','ข','ค','ฆ','ง','จ','ฉ','ช','ซ','ฌ',
        '∞','∑','∏','√','∫','∂','∇','∆','≈','≠','≤','≥',
    ];
}