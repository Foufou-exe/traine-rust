// exo26/src/main.rs

use std::thread;
use std::time::Duration;

fn main() {
    // On lance un nouveau thread qui va s'exécuter en parallèle.
    // La closure || contient le code du thread.
    let handle = thread::spawn(|| {
        // Simule un travail long (ex: téléchargement de 2 secondes)
        thread::sleep(Duration::from_secs(2));
        println!("Téléchargement terminé.");
    });

    // Le programme principal continue son exécution pendant que le thread travaille
    println!("Navigation dans l'interface...");

    // .join() bloque le thread principal jusqu'à ce que le thread secondaire ait fini.
    // Sans cela, le programme s'arrêterait potentiellement avant la fin du téléchargement.
    handle.join().unwrap();
}
