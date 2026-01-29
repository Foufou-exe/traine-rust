// exo5/src/main.rs

fn main() {
    let mut compteur: i32 = 0;

    // 'loop' crée une boucle infinie
    loop {
        compteur += 1; // Incrémentation du compteur
        println!("{}", compteur);

        // Condition de sortie
        if compteur == 10 {
            break; // Arrête la boucle immédiatement
        }
    }
}