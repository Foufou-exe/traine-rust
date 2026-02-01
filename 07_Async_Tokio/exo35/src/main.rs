use tokio::fs; // On utilise le module fs de tokio, pas std::fs !

#[tokio::main]
async fn main() {
    // Écriture asynchrone.
    // Pendant que le disque écrit, le thread peut faire autre chose.
    // .unwrap() est utilisé ici pour la simplicité (panique en cas d'erreur).
    fs::write("data.txt", "Hello Tokio!").await.unwrap();
    println!("Fichier écrit.");

    // Lecture asynchrone.
    let contenu = fs::read_to_string("data.txt").await.unwrap();
    println!("Contenu lu : {}", contenu);

    // Suppression asynchrone.
    fs::remove_file("data.txt").await.unwrap();
    println!("Fichier supprimé.");
}
