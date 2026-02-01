use tokio::time::sleep;
use std::time::Duration;

#[tokio::main]
async fn main() {
    // Création d'un canal MPSC (Multi-Producer, Single-Consumer).
    // tx : Transmetteur (l'émetteur).
    // rx : Récepteur.
    // 32 : Taille du buffer (nombre de messages stockés avant que l'envoi ne bloque).
    let (tx, mut rx) = tokio::sync::mpsc::channel(32);

    // On clone l'émetteur pour la deuxième tâche (car tx est déplacé).
    let tx2 = tx.clone();

    // Tâche 1 : Le Serveur (Producteur 1)
    let _serveur = tokio::spawn(async move {
        // tx est déplacé ici grâce au `move`
        tx.send("Serveur démarré").await.unwrap(); // .unwrap() car l'envoi peut échouer si le receveur est fermé
    });

    // Tâche 2 : La BDD (Producteur 2)
    let _bdd = tokio::spawn(async move {
        sleep(Duration::from_secs(1)).await;
        // tx2 est déplacé ici
        tx2.send("Connexion DB OK").await.unwrap();
    });

    // Le consommateur (ici le thread principal) lit les messages.
    // La boucle `while let Some` continue tant que le canal est ouvert et qu'il y a des émetteurs actifs.
    // Dès que tous les `tx` sont droppés (finis), `rx.recv()` renverra `None` et la boucle s'arrêtera.
    while let Some(message) = rx.recv().await {
        println!("LOG: {}", message);
    }
    
    // Note : La boucle s'arrête ici car _serveur et _bdd finissent leur exécution,
    // ce qui "drop" leurs `tx`. Le `rx` détecte qu'il n'y a plus d'émetteurs.
}