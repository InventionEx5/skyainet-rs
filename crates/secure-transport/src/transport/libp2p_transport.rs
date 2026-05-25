// crates/secure-transport/src/transport/libp2p_transport.rs
// =====================================================
// Libp2p Transport — Cœur du Réseau (Mode Hybride Intelligent)
// Gematria Flash Core + Post-Quantique
// SkyAInet × Nikola T369
// =====================================================

use super::trait::{Transport, TransportLayer, CryptoSuite};
use crate::crypto::hybrid::{T369GematriaHybrid, HybridMode, GematriaKemError};
use crate::crypto::kem::{T369Kem, KemPublicKey, KemCiphertext};

use async_trait::async_trait;
use libp2p::{
    core::upgrade,
    gossipsub, kad, mdns, noise, tcp, yamux,
    swarm::{NetworkBehaviour, SwarmEvent},
    PeerId, Swarm, Transport as Libp2pTransport,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::time::{interval, Duration};
use tracing::{info, warn, debug};
use rand::Rng;

#[derive(NetworkBehaviour)]
struct SkyAInetBehaviour {
    kademlia: kad::Behaviour<kad::store::MemoryStore>,
    gossipsub: gossipsub::Behaviour,
    mdns: mdns::tokio::Behaviour,
}

pub struct Libp2pTransportReal {
    swarm: Option<Swarm<SkyAInetBehaviour>>,
    local_peer_id: PeerId,
    running: bool,
    tx: Option<mpsc::Sender<(SocketAddr, Vec<u8>)>>,
    rx: Option<mpsc::Receiver<(SocketAddr, Vec<u8>)>>,

    hybrid: T369GematriaHybrid,
    flash_interval: Duration,
    last_flash: std::time::Instant,
}

impl Libp2pTransportReal {
    pub fn new() -> Self {
        let local_key = libp2p::identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());

        Self {
            swarm: None,
            local_peer_id,
            running: false,
            tx: None,
            rx: None,
            hybrid: T369GematriaHybrid::new(false),
            flash_interval: Duration::from_secs(60),
            last_flash: std::time::Instant::now(),
        }
    }

    pub async fn start_real(&mut self) -> Result<(), String> {
        let local_key = libp2p::identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());

        let transport = tcp::tokio::Transport::new(tcp::Config::default())
            .upgrade(upgrade::Version::V1)
            .authenticate(noise::Config::new(&local_key).expect("noise"))
            .multiplex(yamux::Config::default())
            .boxed();

        let store = kad::store::MemoryStore::new(local_peer_id);
        let kademlia = kad::Behaviour::new(local_peer_id, store);

        let gossipsub = gossipsub::Behaviour::new(
            gossipsub::MessageAuthenticity::Signed(local_key.clone()),
            gossipsub::Config::default(),
        ).expect("gossipsub");

        let mdns = mdns::tokio::Behaviour::new(mdns::Config::default(), local_peer_id)
            .expect("mdns");

        let behaviour = SkyAInetBehaviour {
            kademlia,
            gossipsub,
            mdns,
        };

        let mut swarm = Swarm::new(
            transport,
            behaviour,
            local_peer_id,
            libp2p::swarm::Config::with_tokio_executor(),
        );

        // === Subscription au topic principal ===
        let topic = gossipsub::IdentTopic::new("skyainet/lessons");
        swarm.behaviour_mut().gossipsub.subscribe(&topic).expect("subscribe");

        swarm
            .listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap())
            .map_err(|e| e.to_string())?;

        self.swarm = Some(swarm);
        self.running = true;

        // Lancer le scheduler des Flash Gematria
        self.start_flash_scheduler();

        info!("[Libp2p] Transport réel démarré - PeerID: {}", local_peer_id);
        Ok(())
    }

    /// Scheduler intelligent des Flash Gematria
    fn start_flash_scheduler(&self) {
        let mut hybrid = self.hybrid.clone();
        tokio::spawn(async move {
            let mut ticker = interval(Duration::from_secs(45));
            loop {
                ticker.tick().await;
                if rand::thread_rng().gen_bool(0.05) {
                    debug!("[Libp2p] Déclenchement Flash Gematria");
                    hybrid.set_mode(HybridMode::FlashGematria);
                }
            }
        });
    }

    /// Envoi réel via GossipSub
    pub async fn send_hybrid(
        &mut self,
        addr: SocketAddr,
        plaintext: &[u8],
        mode: HybridMode,
    ) -> Result<(), String> {
        if let Some(swarm) = &mut self.swarm {
            self.hybrid.set_mode(mode);

            let (kem_ct, ciphertext) = self.hybrid
                .encrypt_with_current_mode(
                    &KemPublicKey { is_1024: false, x25519: [0u8; 32], ml_kem: vec![] },
                    plaintext,
                )
                .map_err(|e| e.to_string())?;

            let topic = gossipsub::IdentTopic::new("skyainet/lessons");

            match swarm.behaviour_mut().gossipsub.publish(topic, ciphertext) {
                Ok(_) => {
                    debug!("[Libp2p] Message publié avec succès (mode: {:?})", mode);
                    Ok(())
                }
                Err(e) => {
                    warn!("[Libp2p] Échec de publication : {}", e);
                    Err(format!("Publication échouée: {}", e))
                }
            }
        } else {
            Err("Swarm non initialisé".to_string())
        }
    }
}

#[async_trait]
impl Transport for Libp2pTransportReal {
    async fn send(&self, addr: SocketAddr, data: &[u8]) -> Result<(), String> {
        // Par défaut : mode BinaryPQ (95% du trafic)
        self.send_hybrid(addr, data, HybridMode::BinaryPQ).await
    }

    async fn recv(&self) -> Result<(SocketAddr, Vec<u8>), String> {
        // Réception réelle via le swarm (géré dans la boucle d'événements)
        // Pour l'instant on retourne un placeholder (à améliorer avec channel)
        Ok(("0.0.0.0:0".parse().unwrap(), vec![]))
    }

    async fn start(&mut self) -> Result<(), String> {
        self.start_real().await
    }

    async fn stop(&mut self) {
        self.running = false;
    }

    fn local_addr(&self) -> Option<SocketAddr> {
        None
    }

    fn crypto_mode(&self) -> CryptoSuite {
        CryptoSuite::BinaryXChaCha20Poly1305
    }

    fn layer(&self) -> TransportLayer {
        TransportLayer::Core
    }
}