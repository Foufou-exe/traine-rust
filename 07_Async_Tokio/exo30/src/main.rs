use tokio::time::sleep;
use std::time::Duration;

/// Une fonction asynchrone qui simule un téléchargement.
/// En Rust, appeler cette fonction ne l'exécute pas immédiatement !
/// Elle retourne une "Future" (une promesse de résultat futur).
async fn telecharger_fichier() -> String {
    // Simulation d'un travail réseau long (1 seconde)
    sleep(Duration::from_secs(1)).await;
    
    // Retour de la donnée une fois le "travail" terminé
    String::from("Fichier.txt")
}

#[tokio::main]
async fn main() {
    // 1. Lancement de la tâche : `mon_future` contient l'état de l'exécution,
    // mais le code à l'intérieur de `telecharger_fichier` n'a pas encore vraiment démarré ou fini.
    let mon_future = telecharger_fichier();
    
    println!("Demande envoyée, en attente...");

    // 2. Point de synchronisation : `.await` demande au runtime d'avancer l'exécution
    // de `mon_future` jusqu'à son terme et d'en extraire la valeur de retour.
    // Le programme "attend" ici sans bloquer le thread système.
    let resultat = mon_future.await;

    println!("Reçu : {}", resultat);
}