// exo27/src/main.rs

use std::sync::mpsc;
use std::thread;

fn main() {
    // Création d'un canal : tx (émetteur) et rx (récepteur)
    let (tx, rx) = mpsc::channel();

    // On lance un thread pour envoyer un message.
    // 'move' est nécessaire pour transférer l'ownership de 'tx' dans le thread.
    thread::spawn(move || {
        let msg = String::from("Ping");
        // Envoi du message dans le canal
        tx.send(msg).unwrap();
    });

    // Le thread principal attend de recevoir un message (opération bloquante)
    let res = rx.recv().unwrap();
    println!("Le serveur a répondu : {}", res);
}
