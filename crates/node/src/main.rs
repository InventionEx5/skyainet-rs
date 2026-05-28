// crates/node/src/main.rs
// =====================================================
// SkyNode Tauri Entry Point — Version Finale v6.8
// Connecte le Frontend (skynode.html) au Backend Rust
// Avec Rewards, Evolution et toutes les commandes
// =====================================================

use std::sync::Arc;
use tokio::sync::Mutex;

use crate::skynode::init_skynode;

// =====================================================
// IMPORT DE TOUTES LES COMMANDES TAURI
// =====================================================

use crate::skynode::{
    // Commandes de base
    get_node_stats,
    get_storage_list,
    upload_file,
    download_file,
    delete_file,
    sync_storage,
    route_query,
    
    // Commandes IA & Hub
    generate_with_ai,
    send_ai_message,
    get_registered_ais,
    toggle_external_ai,
    
    // Commandes Gateway
    enable_gateway,
    generate_dynamic_site,
    create_api_key,
    
    // Commandes Évolution
    run_evolution_cycle,
    trigger_traditional_training,
    
    // Commandes Rewards (NOUVEAU)
    claim_rewards,
    get_rewards_stats,
};

#[tokio::main]
async fn main() {
    // Initialisation du SkyNode (partagé avec le frontend)
    let skynode: Arc<Mutex<crate::skynode::SkyNode>> = init_skynode();

    println!("🚀 SkyNode Tauri v6.8 démarré avec succès");
    println!("   → Frontend : skynode.html");
    println!("   → Moteur d'inférence : T369Inference");
    println!("   → Système de Rewards : Activé");
    println!("   → Évolution Hybride : Dream Cycle + Traditional Training");
    println!("   → Stockage : RomanT369 + ZipMemory");

    tauri::Builder::default()
        // Partage de l'état du SkyNode
        .manage(skynode.clone())

        // Enregistrement de TOUTES les commandes
        .invoke_handler(tauri::generate_handler![
            // Base
            get_node_stats,
            get_storage_list,
            upload_file,
            download_file,
            delete_file,
            sync_storage,
            route_query,
            
            // IA & Hub
            generate_with_ai,
            send_ai_message,
            get_registered_ais,
            toggle_external_ai,
            
            // Gateway
            enable_gateway,
            generate_dynamic_site,
            create_api_key,
            
            // Évolution
            run_evolution_cycle,
            trigger_traditional_training,
            
            // Rewards
            claim_rewards,
            get_rewards_stats,
        ])

        // Lancement de l'application
        .run(tauri::generate_context!())
        .expect("Erreur lors du démarrage de SkyNode Tauri");
}