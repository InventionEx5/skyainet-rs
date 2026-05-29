// crates/secure-transport/src/defense/anti_debug.rs
// =====================================================
// Anti-Debug & Anti-Reverse Engineering v5.2 — Fully Hardened
// SkyAInet × Nikola T369 — Physical Attacks Protection (PA)
// =====================================================

use std::fs;
use std::time::Instant;
use tracing::{warn, error, info};
use sha2::{Sha256, Digest};

#[cfg(target_os = "linux")]
use libc::{ptrace, PTRACE_TRACEME};

#[cfg(target_os = "windows")]
use winapi::um::debugapi::{IsDebuggerPresent, CheckRemoteDebuggerPresent};
#[cfg(target_os = "windows")]
use winapi::um::winnt::HANDLE;

pub struct AntiDebug {
    pub tamper_detected: bool,
    pub debugger_detected: bool,
    binary_hash: Option<[u8; 32]>, // Hash SHA-256 du binaire au démarrage
}

impl AntiDebug {
    pub fn new() -> Self {
        let mut anti = Self {
            tamper_detected: false,
            debugger_detected: false,
            binary_hash: None,
        };
        anti.compute_binary_hash();
        anti
    }

    /// Calcule et stocke le hash SHA-256 du binaire (pour détection de tamper)
    fn compute_binary_hash(&mut self) {
        if let Ok(exe_path) = std::env::current_exe() {
            if let Ok(data) = fs::read(&exe_path) {
                let mut hasher = Sha256::new();
                hasher.update(&data);
                let hash: [u8; 32] = hasher.finalize().into();
                self.binary_hash = Some(hash);
            }
        }
    }

    /// Détection multi-couche très robuste
    pub fn detect_debugger(&mut self) -> bool {
        let mut detected = false;

        // === 1. Linux ===
        #[cfg(target_os = "linux")]
        {
            if let Ok(status) = fs::read_to_string("/proc/self/status") {
                if status.contains("TracerPid:\t") && !status.contains("TracerPid:\t0") {
                    warn!("[AntiDebug] Debugger détecté via /proc/self/status");
                    detected = true;
                }
            }

            unsafe {
                if ptrace(PTRACE_TRACEME, 0, std::ptr::null_mut(), std::ptr::null_mut()) == -1 {
                    warn!("[AntiDebug] ptrace(PTRACE_TRACEME) a échoué → Debugger présent");
                    detected = true;
                }
            }
        }

        // === 2. macOS ===
        #[cfg(target_os = "macos")]
        {
            if std::env::var("DYLD_INSERT_LIBRARIES").is_ok()
                || std::env::var("MallocStackLogging").is_ok()
            {
                warn!("[AntiDebug] Injection ou debugging détecté sur macOS");
                detected = true;
            }
        }

        // === 3. Windows (implémentation réelle) ===
        #[cfg(target_os = "windows")]
        {
            unsafe {
                if IsDebuggerPresent() != 0 {
                    warn!("[AntiDebug] IsDebuggerPresent() a retourné TRUE");
                    detected = true;
                }

                let mut is_remote_debugger_present: i32 = 0;
                if CheckRemoteDebuggerPresent(
                    std::ptr::null_mut() as HANDLE,
                    &mut is_remote_debugger_present,
                ) != 0
                    && is_remote_debugger_present != 0
                {
                    warn!("[AntiDebug] CheckRemoteDebuggerPresent() a retourné TRUE");
                    detected = true;
                }
            }
        }

        // === 4. Variables d'environnement suspectes ===
        let suspicious_vars = [
            "GDB", "LLDB", "DEBUG", "LD_PRELOAD", "DYLD_INSERT_LIBRARIES",
            "MallocStackLogging", "MallocStackLoggingNoCompact", "RUST_BACKTRACE",
        ];

        for var in &suspicious_vars {
            if std::env::var(var).is_ok() {
                warn!("[AntiDebug] Variable d'environnement suspecte: {}", var);
                detected = true;
            }
        }

        // === 5. Test de timing anti-VM ===
        let start = Instant::now();
        let _sum: u64 = (0..2_500_000).sum();
        if start.elapsed().as_millis() > 1300 {
            warn!("[AntiDebug] Exécution anormalement lente (possible VM/debugger)");
            detected = true;
        }

        self.debugger_detected = detected;
        detected
    }

    pub fn self_terminate_if_debugged(&mut self) {
        if self.detect_debugger() {
            error!("[AntiDebug] === DEBUGGER DÉTECTÉ === Auto-destruction");
            std::process::exit(1);
        }
    }

    /// Détection de tamper avec hash SHA-256
    pub fn detect_tamper(&mut self) -> bool {
        if let Some(original_hash) = self.binary_hash {
            if let Ok(exe_path) = std::env::current_exe() {
                if let Ok(data) = fs::read(&exe_path) {
                    let mut hasher = Sha256::new();
                    hasher.update(&data);
                    let current_hash: [u8; 32] = hasher.finalize().into();

                    if current_hash != original_hash {
                        self.tamper_detected = true;
                        error!("[AntiDebug] TAMPER DÉTECTÉ ! Le binaire a été modifié.");
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Active la protection maximale
    pub fn enable_hardened_mode(&mut self) {
        info!("[AntiDebug] Mode ULTRA DURCI v5.2 activé");

        self.detect_tamper();
        self.self_terminate_if_debugged();

        if self.tamper_detected || self.debugger_detected {
            error!("[AntiDebug] Protection déclenchée → Arrêt immédiat du processus");
            std::process::exit(1);
        }
    }
}

impl Default for AntiDebug {
    fn default() -> Self {
        Self::new()
    }
}