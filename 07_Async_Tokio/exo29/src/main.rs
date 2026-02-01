use tokio::time::sleep;
use std::time::Duration;

/// Point d'entrée asynchrone.
/// La macro #[tokio::main] transforme cette fonction async en une machine à états
/// gérée par le runtime Tokio. C'est ce qui permet d'utiliser `.await`.
#[tokio::main]
async fn main() {
    println!("Début du traitement");

    // `tokio::time::sleep` est une pause NON-BLOQUANTE.
    // Contrairement à `std::thread::sleep` qui figerait tout le thread (et donc toutes les tâches dessus),
    // celle-ci rend la main au runtime. Le runtime peut exécuter d'autres tâches
    // pendant ces 3 secondes (s'il y en avait).
    // Le `.await` indique : "Mets cette tâche en pause jusqu'à ce que le timer soit fini".
    sleep(Duration::from_secs(3)).await;

    println!("Fin du traitement");
}