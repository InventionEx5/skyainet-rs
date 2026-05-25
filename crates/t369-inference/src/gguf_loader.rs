use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::collections::HashMap;

/// Chargeur GGUF natif (pur Rust)
pub struct GgufLoader {
    pub magic: [u8; 4],
    pub version: u32,
    pub tensor_count: u64,
    pub metadata_kv_count: u64,
    pub metadata: HashMap<String, String>,
    pub tensors: Vec<GgufTensor>,
}

#[derive(Debug, Clone)]
pub struct GgufTensor {
    pub name: String,
    pub shape: Vec<u64>,
    pub data_type: u32,
    pub offset: u64,
    pub data: Vec<u8>,
}

impl GgufLoader {
    pub fn load(path: &str) -> Result<Self, String> {
        let mut file = File::open(path).map_err(|e| format!("Impossible d'ouvrir {}: {}", path, e))?;

        // === Lecture du header GGUF ===
        let mut magic = [0u8; 4];
        file.read_exact(&mut magic).map_err(|_| "Fichier trop petit".to_string())?;

        if &magic != b"GGUF" {
            return Err("Ce n'est pas un fichier GGUF valide".to_string());
        }

        let mut version_buf = [0u8; 4];
        file.read_exact(&mut version_buf).map_err(|_| "Erreur lecture version".to_string())?;
        let version = u32::from_le_bytes(version_buf);

        let mut tensor_count_buf = [0u8; 8];
        file.read_exact(&mut tensor_count_buf).map_err(|_| "Erreur lecture tensor_count".to_string())?;
        let tensor_count = u64::from_le_bytes(tensor_count_buf);

        let mut metadata_kv_count_buf = [0u8; 8];
        file.read_exact(&mut metadata_kv_count_buf).map_err(|_| "Erreur lecture metadata".to_string())?;
        let metadata_kv_count = u64::from_le_bytes(metadata_kv_count_buf);

        // === Lecture des métadonnées (simplifiée) ===
        let mut metadata = HashMap::new();
        for _ in 0..metadata_kv_count {
            // On lit juste les clés pour l'instant (parsing complet plus tard)
            let key = Self::read_string(&mut file)?;
            let value_type = Self::read_u32(&mut file)?;
            
            // On skip les valeurs pour l'instant (on peut les ajouter plus tard)
            metadata.insert(key, format!("type_{}", value_type));
        }

        // === Lecture des tenseurs (headers seulement) ===
        let mut tensors = Vec::new();
        let mut data_offset = 0u64;

        for _ in 0..tensor_count {
            let name = Self::read_string(&mut file)?;
            let n_dims = Self::read_u32(&mut file)? as usize;

            let mut shape = Vec::new();
            for _ in 0..n_dims {
                let dim = Self::read_u64(&mut file)?;
                shape.push(dim);
            }

            let data_type = Self::read_u32(&mut file)?;
            let offset = Self::read_u64(&mut file)?;

            tensors.push(GgufTensor {
                name,
                shape,
                data_type,
                offset,
                data: vec![],
            });
        }

        // Calcul de l'offset des données
        let alignment = 32; // GGUF utilise généralement 32
        data_offset = ((file.stream_position().map_err(|e| e.to_string())? + alignment as u64 - 1) / alignment as u64) * alignment as u64;

        Ok(Self {
            magic,
            version,
            tensor_count,
            metadata_kv_count,
            metadata,
            tensors,
        })
    }

    fn read_string(file: &mut File) -> Result<String, String> {
        let len = Self::read_u64(file)? as usize;
        let mut buf = vec![0u8; len];
        file.read_exact(&mut buf).map_err(|e| e.to_string())?;
        String::from_utf8(buf).map_err(|e| e.to_string())
    }

    fn read_u32(file: &mut File) -> Result<u32, String> {
        let mut buf = [0u8; 4];
        file.read_exact(&mut buf).map_err(|e| e.to_string())?;
        Ok(u32::from_le_bytes(buf))
    }

    fn read_u64(file: &mut File) -> Result<u64, String> {
        let mut buf = [0u8; 8];
        file.read_exact(&mut buf).map_err(|e| e.to_string())?;
        Ok(u64::from_le_bytes(buf))
    }

    /// Charge les données d'un tenseur spécifique
    pub fn load_tensor_data(&mut self, file: &mut File, tensor_index: usize) -> Result<(), String> {
        if tensor_index >= self.tensors.len() {
            return Err("Index de tenseur invalide".to_string());
        }

        let tensor = &mut self.tensors[tensor_index];
        file.seek(SeekFrom::Start(tensor.offset)).map_err(|e| e.to_string())?;

        let size: usize = tensor.shape.iter().product::<u64>() as usize 
            * Self::get_type_size(tensor.data_type);

        tensor.data = vec![0u8; size];
        file.read_exact(&mut tensor.data).map_err(|e| e.to_string())?;

        Ok(())
    }

    fn get_type_size(data_type: u32) -> usize {
        match data_type {
            0 => 4,   // F32
            1 => 2,   // F16
            2 => 1,   // Q4_0 (simplifié)
            3 => 1,   // Q4_1
            _ => 4,
        }
    }
}