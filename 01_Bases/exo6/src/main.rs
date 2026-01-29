// exo6/src/main.rs

fn main() {
    // La boucle 'for' est très utilisée en Rust pour itérer sur des séquences.
    // Ici, on utilise une plage (range) de 1 à 5 inclus (..=).
    // Si on avait utilisé '1..5', la boucle serait allée de 1 à 4.
    for x in 1..=5 {
        println!("Tour numéro: {}", x);
    }
}