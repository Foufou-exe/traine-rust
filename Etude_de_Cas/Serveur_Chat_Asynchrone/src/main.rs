use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::broadcast;

/// Étude de cas : Serveur de Chat Multi-utilisateurs Asynchrone.
/// Ce projet combine la gestion réseau TCP, le broadcast asynchrone et la concurrence.
#[tokio::main]
async fn main() {
    // 1. Création d'un canal de diffusion (broadcast).
    // Tout message envoyé sur 'tx' sera reçu par tous les abonnés 'rx'.
    // Le buffer est de 10 messages.
    let (tx, _rx) = broadcast::channel(10);

    // 2. Liaison du serveur TCP sur l'adresse locale.
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("🚀 Serveur de chat démarré sur 127.0.0.1:8080");
    println!("Pour vous connecter : telnet 127.0.0.1 8080");

    loop {
        // 3. Boucle d'acceptation des connexions.
        // Chaque nouvelle connexion produit un 'socket'.
        let (mut socket, addr) = listener.accept().await.unwrap();
        println!("👤 Nouveau client connecté : {:?}", addr);

        // 4. Préparation pour le client.
        // On s'abonne au canal de diffusion général.
        let mut rx = tx.subscribe();
        // On clone l'émetteur pour que le client puisse lui-même envoyer des messages.
        let tx = tx.clone();

        // 5. Spawn d'une tâche dédiée au client (Green thread).
        tokio::spawn(async move {
            // Séparation du socket en lecture et écriture.
            let (reader, mut writer) = socket.split();
            let mut readers = BufReader::new(reader);
            let mut line = String::new();

            loop {
                // 6. Gestion multi-flux avec select!.
                // On attend soit un message venant du client, soit un message venant du chat global.
                tokio::select! {
                    // Flux A : Le client nous envoie un message
                    result = readers.read_line(&mut line) => {
                        match result {
                            Ok(0) => break, // Client déconnecté
                            Ok(_) => {
                                // On diffuse le message à tout le monde (incluant l'expéditeur, géré plus bas)
                                tx.send((line.clone(), addr)).unwrap();
                                line.clear();
                            },
                            Err(_) => break
                        }
                    }
                    // Flux B : On reçoit un message du broadcast global
                    msg_result = rx.recv() => {
                        match msg_result {
                            Ok((msg, other_addr)) => {
                                // On n'envoie le message au client que s'il ne vient pas de lui-même
                                if addr != other_addr {
                                    writer.write_all(msg.as_bytes()).await.unwrap();
                                }
                            },
                            Err(_) => {}
                        }
                    }
                }
            }
            println!("👋 Client déconnecté : {:?}", addr);
        });
    }
}