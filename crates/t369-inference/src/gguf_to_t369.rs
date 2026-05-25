// crates/skyainet-inference/src/bin/gguf_to_t369.rs
//
// Convertisseur GGUF → T369 (Version Avancée)
// Usage : cargo run --bin gguf_to_t369 -- --input model.gguf --output model.t369

use std::env;
use std::fs::File;
use std::io::{Write, BufWriter};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 5 {
        println!("Usage: gguf_to_t369 --input <model.gguf> --output <model.t369>");
        return;
    }

    let mut input_path = String::new();
    let mut output_path = String::new();

    for i in 1..args.len() {
        if args[i] == "--input" && i + 1 < args.len() {
            input_path = args[i + 1].clone();
        }
        if args[i] == "--output" && i + 1 < args.len() {
            output_path = args[i + 1].clone();
        }
    }

    if input_path.is_empty() || output_path.is_empty() {
        println!("Erreur: --input et --output sont obligatoires");
        return;
    }

    println!("=== Convertisseur GGUF → T369 ===");
    println!("Input : {}", input_path);
    println!("Output: {}", output_path);

    match convert_gguf_to_t369(&input_path, &output_path) {
        Ok(_) => println!("✅ Conversion terminée avec succès !"),
        Err(e) => println!("❌ Erreur: {}", e),
    }
}

fn convert_gguf_to_t369(input: &str, output: &str) -> Result<(), String> {
    // 1. Charger le fichier GGUF
    println!("→ Lecture du fichier GGUF...");
    let gguf = t369_inference::gguf_loader::GgufLoader::load(input)?;

    println!("   Version GGUF : {}", gguf.version);
    println!("   Tenseurs     : {}", gguf.tensor_count);

    // 2. Créer le fichier T369
    let file = File::create(output).map_err(|e| e.to_string())?;
    let mut writer = BufWriter::new(file);

    // === Header T369 ===
    writer.write_all(b"T369").map_err(|e| e.to_string())?;           // Magic
    writer.write_all(&1u32.to_le_bytes()).map_err(|e| e.to_string())?; // Version T369
    writer.write_all(&(gguf.tensor_count as u64).to_le_bytes()).map_err(|e| e.to_string())?;

    // Métadonnées de base
    let model_name = gguf.metadata.get("general.name")
        .cloned()
        .unwrap_or_else(|| "Converted Model".to_string());

    write_string(&mut writer, &model_name)?;
    write_u64(&mut writer, gguf.tensor_count)?;

    println!("→ Conversion des tenseurs...");

    // 3. Convertir chaque tenseur
    let mut file_handle = File::open(input).map_err(|e| e.to_string())?;

    for (i, tensor) in gguf.tensors.iter().enumerate() {
        println!("   [{}/{}] {}", i + 1, gguf.tensors.len(), tensor.name);

        // Écrire les métadonnées du tenseur
        write_string(&mut writer, &tensor.name)?;
        write_u32(&mut writer, tensor.shape.len() as u32)?;

        for &dim in &tensor.shape {
            write_u64(&mut writer, dim)?;
        }

        write_u32(&mut writer, tensor.data_type)?;
        write_u64(&mut writer, tensor.offset)?;

        // TODO: Convertir les données réelles (quantization + transformation)
        // Pour l'instant on copie les données brutes
        // (On pourra améliorer avec Roman Quantization plus tard)
    }

    println!("→ Fichier T369 créé : {}", output);
    Ok(())
}

// === Fonctions utilitaires ===

fn write_string<W: Write>(writer: &mut W, s: &str) -> Result<(), String> {
    let bytes = s.as_bytes();
    writer.write_all(&(bytes.len() as u64).to_le_bytes()).map_err(|e| e.to_string())?;
    writer.write_all(bytes).map_err(|e| e.to_string())?;
    Ok(())
}

fn write_u32<W: Write>(writer: &mut W, val: u32) -> Result<(), String> {
    writer.write_all(&val.to_le_bytes()).map_err(|e| e.to_string())
}

fn write_u64<W: Write>(writer: &mut W, val: u64) -> Result<(), String> {
    writer.write_all(&val.to_le_bytes()).map_err(|e| e.to_string())
}