// crates/model/src/thevie/neural_mesh.rs
// =====================================================
// Neural Mesh v6.0 — Cerveau Distribué Intelligent
// Auto-organisation, Neurogenèse, Recherche Sémantique + Persistance
// =====================================================

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};
use tracing::{info, debug, warn};

use super::personality::Personality;
use super::neurone::Neurone;
use super::synapse::Synapse;
use super::persistent_storage::PersistentStorage;
use crate::memory::vector_store::{VectorStore, VectorEntry};

pub type NeuronId = u64;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Lesson {
    pub query: String,
    pub response: String,
    pub quality: f32,
    pub expert_used: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshStats {
    pub total_neurons: usize,
    pub total_synapses: usize,
    pub avg_strength: f32,
    pub avg_activity: f32,
    pub most_connected_neuron: Option<NeuronId>,
    pub last_maintenance: u64,
}

pub struct NeuralMesh {
    neurons: HashMap<NeuronId, Neurone>,
    synapses: HashMap<(NeuronId, NeuronId), Synapse>,
    id_counter: AtomicU64,
    last_maintenance: Instant,
    persistent_storage: Option<PersistentStorage>,
    vector_store: VectorStore,
}

impl NeuralMesh {
    pub fn new() -> Self {
        let storage = PersistentStorage::new("./data/neural_mesh").ok();

        Self {
            neurons: HashMap::new(),
            synapses: HashMap::new(),
            id_counter: AtomicU64::new(1),
            last_maintenance: Instant::now(),
            persistent_storage: storage,
            vector_store: VectorStore::new(),
        }
    }

    fn generate_id(&self) -> NeuronId {
        self.id_counter.fetch_add(1, Ordering::SeqCst)
    }

    /// Ajoute un nouveau neurone avec auto-connexion intelligente
    pub fn add_neuron(&mut self, mut neuron: Neurone) -> NeuronId {
        let id = self.generate_id();
        neuron.id = id;

        self.neurons.insert(id, neuron);
        self.auto_establish_synapses(id);

        info!("🧠 Neurone {} créé et connecté au mesh", id);
        id
    }

    /// Connexions automatiques avec les neurones les plus pertinents
    fn auto_establish_synapses(&mut self, new_id: NeuronId) {
        if self.neurons.len() <= 1 {
            return;
        }

        let mut candidates: Vec<(NeuronId, f32)> = self.neurons
            .iter()
            .filter(|(&id, _)| id != new_id)
            .map(|(&id, n)| (id, self.calculate_connection_score(n)))
            .collect();

        candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        for (target_id, score) in candidates.into_iter().take(4) {
            let strength = (score / 100.0).clamp(0.35, 0.95);
            self.connect(new_id, target_id, strength);
            self.connect(target_id, new_id, strength * 0.85);
        }
    }

    fn calculate_connection_score(&self, neuron: &Neurone) -> f32 {
        (neuron.activity_score as f32 * 0.45)
            + (neuron.personality.wisdom * 28.0)
            + (neuron.personality.cooperation * 22.0)
            + (neuron.personality.curiosity * 8.0)
    }

    pub fn connect(&mut self, from: NeuronId, to: NeuronId, strength: f32) {
        if from == to { return; }
        let syn = Synapse::new(from, to);
        let mut syn = syn; // mutable
        syn.strength = strength.clamp(0.15, 1.0);
        self.synapses.insert((from, to), syn);
    }

    pub fn hebbian_update(&mut self, from: NeuronId, to: NeuronId, success: bool) {
        if let Some(syn) = self.synapses.get_mut(&(from, to)) {
            if success {
                syn.strength = (syn.strength + 0.12).min(1.0);
            } else {
                syn.strength = (syn.strength - 0.18).max(0.15);
            }
            syn.usage_count += 1;
            syn.last_used = Self::now_millis();
        }
    }

    pub fn prune_weak_synapses(&mut self) {
        let before = self.synapses.len();
        self.synapses.retain(|_, s| s.strength > 0.22 || s.usage_count > 12);
        if before != self.synapses.len() {
            debug!("🧹 Pruning terminé : {} synapses supprimées", before - self.synapses.len());
        }
    }

    pub fn decay_synapses(&mut self) {
        let now = Self::now_millis();
        for syn in self.synapses.values_mut() {
            let age_hours = (now - syn.last_used) as f32 / 3_600_000.0;
            if age_hours > 8.0 {
                let decay = (age_hours / 60.0).min(0.28);
                syn.strength = (syn.strength - decay).max(0.12);
            }
        }
    }

    pub fn circulate_lesson(&mut self, from: NeuronId, lesson: &Lesson) {
        // Indexation sémantique
        let embedding = self.embed_text(&lesson.query);
        let entry = VectorEntry {
            id: format!("lesson_{}", lesson.timestamp),
            embedding,
            metadata: Some(serde_json::json!({
                "query": &lesson.query,
                "quality": lesson.quality,
                "expert": &lesson.expert_used
            })),
        };
        self.vector_store.insert(entry);

        // Propagation via synapses
        let peers = self.get_top_connected(from, 5);
        for peer_id in peers {
            self.hebbian_update(from, peer_id, true);
        }

        debug!("📡 Leçon propagée et indexée depuis neurone {}", from);
    }

    pub fn semantic_search(&self, query: &str, top_k: usize) -> Vec<(Lesson, f32)> {
        let query_embedding = self.embed_text(query);
        let results = self.vector_store.search(&query_embedding, top_k);

        results.into_iter()
            .filter_map(|(id, score)| {
                // Pour l'instant, reconstruction simplifiée
                Some((Lesson {
                    query: query.to_string(),
                    response: "Réponse sémantiquement proche".to_string(),
                    quality: score,
                    expert_used: "semantic".to_string(),
                    timestamp: Self::now_millis(),
                }, score))
            })
            .collect()
    }

    fn embed_text(&self, text: &str) -> Vec<f32> {
        let mut emb = vec![0.0f32; 128];
        let bytes = text.as_bytes();

        for (i, &b) in bytes.iter().enumerate() {
            emb[i % 128] += (b as f32) / 255.0;
        }

        let norm: f32 = emb.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            for v in &mut emb { *v /= norm; }
        }
        emb
    }

    pub fn get_top_connected(&self, neuron_id: NeuronId, k: usize) -> Vec<NeuronId> {
        let mut conns: Vec<_> = self.synapses
            .iter()
            .filter(|((f, _), _)| *f == neuron_id)
            .map(|((_, t), s)| (*t, s.strength))
            .collect();

        conns.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        conns.into_iter().take(k).map(|(id, _)| id).collect()
    }

    pub fn get_mesh_stats(&self) -> MeshStats {
        let total_synapses = self.synapses.len();
        let avg_strength = if total_synapses > 0 {
            self.synapses.values().map(|s| s.strength).sum::<f32>() / total_synapses as f32
        } else { 0.0 };

        let avg_activity = if !self.neurons.is_empty() {
            self.neurons.values().map(|n| n.activity_score as f32).sum::<f32>() / self.neurons.len() as f32
        } else { 0.0 };

        let most_connected = self.synapses
            .keys()
            .fold(HashMap::new(), |mut acc, (_, to)| {
                *acc.entry(*to).or_insert(0) += 1;
                acc
            })
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(id, _)| id);

        MeshStats {
            total_neurons: self.neurons.len(),
            total_synapses,
            avg_strength,
            avg_activity,
            most_connected_neuron: most_connected,
            last_maintenance: Self::now_millis(),
        }
    }

    pub fn run_maintenance(&mut self) {
        self.prune_weak_synapses();
        self.decay_synapses();
        self.last_maintenance = Instant::now();
        debug!("🔧 Maintenance Neural Mesh terminée");
    }

    pub fn persist(&self) {
        if let Some(storage) = &self.persistent_storage {
            for (id, neuron) in &self.neurons {
                if let Ok(data) = serde_json::to_vec(neuron) {
                    let _ = storage.save_neuron(*id, &data);
                }
            }
            info!("[NeuralMesh] {} neurones persistés", self.neurons.len());
        }
    }

    // Accesseurs
    pub fn get_neuron(&self, id: NeuronId) -> Option<&Neurone> { self.neurons.get(&id) }
    pub fn get_neuron_mut(&mut self, id: NeuronId) -> Option<&mut Neurone> { self.neurons.get_mut(&id) }
    pub fn remove_neuron(&mut self, id: NeuronId) {
        self.neurons.remove(&id);
        self.synapses.retain(|(f, t), _| *f != id && *t != id);
    }

    fn now_millis() -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64
    }
}

impl Default for NeuralMesh {
    fn default() -> Self {
        Self::new()
    }
}