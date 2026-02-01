use tokio::fs;

// Signature de main modifiée pour retourner un Result.
// Box<dyn std::error::Error> est un type "fourre-tout" qui accepte n'importe quelle erreur standard.
// C'est très pratique pour les petits programmes ou les scripts.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    // L'opérateur `?` fonctionne aussi dans les blocs async.
    // Si `write` échoue, l'erreur est immédiatement retournée hors de `main`.
    fs::write("data.txt", "Hello Tokio!").await?;
    println!("Fichier écrit.");

    let contenu = fs::read_to_string("data.txt").await?;
    println!("Contenu : {}", contenu);

    fs::remove_file("data.txt").await?;
    
    // Si on arrive ici, tout s'est bien passé.
    Ok(())
}
