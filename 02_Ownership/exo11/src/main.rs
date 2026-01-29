// exo11/src/main.rs

// Cette fonction prend deux références de chaînes et en retourne une.
// Rust a besoin de savoir combien de temps la référence retournée est valide.
// L'annotation <'a> indique que la référence retournée vivra aussi longtemps
// que la plus courte des durées de vie des deux paramètres passés.
fn meilleur_film<'a>(film1: &'a str, film2: &'a str) -> &'a str {
    if film1.len() > film2.len() {
        film1
    } else {
        film2
    }
}

fn main() {
    let film1 = String::from("Avatar");
    let film2 = String::from("Titanic");
    
    // Appel de la fonction. Rust vérifie ici que les durées de vie sont compatibles.
    let film = meilleur_film(&film1, &film2);
    
    println!("Le film au titre le plus long est : {}", film);
}