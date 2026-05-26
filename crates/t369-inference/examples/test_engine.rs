use t369_inference::T369Inference;

fn main() {
    println!("🚀 Test du moteur T369Inference\n");

    let mut inference = match T369Inference::new() {
        Ok(i) => i,
        Err(e) => {
            eprintln!("❌ Erreur création moteur: {}", e);
            return;
        }
    };

    let prompt = "Explique-moi ce qu'est une IA décentralisée en une phrase.";

    println!("📝 Prompt : {}\n", prompt);
    println!("⏳ Génération en cours...\n");

    match inference.generate(prompt, 64) {
        Ok(response) => {
            println!("✅ Réponse générée :\n");
            println!("{}\n", response);
        }
        Err(e) => {
            eprintln!("❌ Erreur génération: {}", e);
        }
    }

    println!("🎉 Test terminé avec succès !");
}
