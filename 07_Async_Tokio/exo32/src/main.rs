use tokio::time::sleep;
use std::time::Duration;

// Une tâche lente
async fn tortue() -> String {
    sleep(Duration::from_secs(2)).await;
    String::from("Tortue")
}

// Une tâche rapide
async fn lievre() -> String {
    sleep(Duration::from_secs(1)).await;
    String::from("Lievre")
}

#[tokio::main]
async fn main() {
    // `tokio::select!` permet d'attendre sur plusieurs branches async en même temps.
    // Dès que L'UNE des branches termine, le bloc de code associé est exécuté.
    // IMPORTANT : Toutes les autres branches en cours sont immédiatement ANNULÉES (dropped).
    tokio::select! {
        // Si la tortue finit en premier (improbable ici)
        vainqueur_tortue = tortue() => {
                println!("Le vainqueur est : {}", vainqueur_tortue)
        }
        // Si le lièvre finit en premier
        vainqueur_lievre = lievre() => {
            println!("Le vainqueur est : {}", vainqueur_lievre)
        }
    }
}