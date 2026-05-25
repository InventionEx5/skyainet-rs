# SkyAInet × Nikola T369

**SkyAInet** est une infrastructure post-quantique décentralisée pour l'IA collective, la gouvernance décentralisée et l'intelligence distribuée.

Version actuelle : **v0.4.2** • Quantique T369

---

## Fonctionnalités principales

SkyAInet propose un ensemble complet de fonctionnalités :

- **Thevie** — Intelligence collective post-quantique (PAEVF)
- **SkyNode** — Nœuds de calcul décentralisés (Mini / Light / Full / DreamWeaver / Validator)
- **Dream Me** — Cycles de sagesse collective et apprentissage onirique
- **Gouvernance** — Système de Conviction Voting et propositions décentralisées
- **Marketplace** — Location de puissance de calcul (TFLOPS)
- **Messagerie** — Communications chiffrées post-quantiques (Double Ratchet + RomanT369)
- **Monitoring Avancé** — KL Divergence, Ethical Score, ZIP Memory Manager
- **Wallet** — Gestion des clés, Device Keys, phrase BIP39
- **Secure Transport** — Couche cryptographique complète (Nikola T369, KemT369, Gematria Hybrid)

---

## Démarrage rapide

### Prérequis

- Rust 1.85+
- Tauri CLI
- (Optionnel) Python 3.10+ pour les outils IA

### Installation

```bash
git clone https://github.com/skyainet/skyainet-rs.git
cd skyainet-rs

# Exécution du setup complet
chmod +x setup.sh
./setup.sh
```

### Lancement

```bash
# Mode développement
cargo tauri dev

# Build release
cargo tauri build
```

---

## Structure du projet

Le projet est organisé selon une architecture modulaire claire :

skyainet-rs/
├── crates/
│   ├── core/                    # Types fondamentaux et alignement éthique
│   ├── node/                    # Gestion des nœuds SkyAInet
│   ├── model/                   # Thevie et intelligence collective
│   ├── secure-transport/        # Cryptographie post-quantique (Nikola T369)
│   ├── skyainet-inference/      # Moteur d'inférence hybride
│   ├── memory/                  # Zip Memory + IPFS + Vector Store
│   ├── sentinel/                # Auto-healing et protection
│   ├── api/                     # REST + GraphQL + WebSocket
│   ├── financial/               # Treasury et liquidité
│   └── governance/              # DAO et Conviction Voting
├── src-tauri/                   # Application Tauri (Backend Rust)
├── ui/                          # Interface utilisateur (11 pages HTML)
├── scripts/                     # Scripts de déploiement et distillation
├── contracts/                   # Smart contracts Solidity
├── docker/                      # Conteneurs Docker
├── monitoring/                  # Prometheus + Grafana
├── bindings/python/             # Bindings Python
├── whitepaper/                  # Documentation et whitepaper
└── Cargo.toml

---

## Pages principales de l'interface

L'interface utilisateur Tauri contient les pages suivantes :

- **Thevie** : IA collective et alignement PAEVF
- **SkyNode** : Gestion des nœuds de calcul
- **index.html** : Vue d'ensemble du réseau (page d'accueil)
- **Wallet** : Gestion des identités et clés
- **Settings** : Configuration, sécurité, préférences
- **Nodes** : Mes nœuds et locations
- **Monitoring** : Métriques avancées + ZIP Memory
- **Messaging** : Messagerie post-quantique
- **Marketplace** : Location de puissance TFLOPS
- **Governance** : Propositions et Conviction Voting
- **Dream Me** : Cycles de sagesse collective

---

## Licence

Ce projet est distribué sous licence **AGPL-3.0-only**.

---

**SkyAInet × Nikola T369**  
*Une intelligence qui grandit avec nous.*