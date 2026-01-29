// exo15/src/main.rs

// La fonction retourne un Option<f64> : soit un nombre (Some), soit rien (None)
fn division(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None // Cas d'erreur : on ne retourne rien
    } else {
        Some(a / b) // Cas de succès : on emballe le résultat dans Some()
    }
}

fn main() {
    let resultat = division(10.0, 20.0); // Essayer avec 0.0 pour voir le cas None
    
    // On doit gérer les deux cas possibles
    match resultat {
        None => println!("Erreur : Division par zéro"),
        Some(valeur) => println!("Résultat : {}", valeur),
    }
}