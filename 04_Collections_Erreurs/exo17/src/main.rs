// exo17/src/main.rs

fn main() {
    let saisie = "42"; // Essayez de remplacer par "abc" pour voir le cas d'erreur
    
    // .parse() retourne un Result car la conversion peut échouer
    // On précise le type cible avec le "turbofish" ::<i32>
    let resultat = saisie.parse::<i32>();

    match resultat {
        Ok(nombre) => println!("Le double est : {}", nombre * 2),
        Err(e) => println!("Ce n'est pas un nombre valide. Erreur : ({})", e),
    }
}
