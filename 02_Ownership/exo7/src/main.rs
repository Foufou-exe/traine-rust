// exo7/src/main.rs

fn main() {
    let chaine1 = String::from("Rust"); // Création d'une String (allouée sur le tas)
    
    // Sans .clone(), chaine1 serait "déplacée" (moved) vers chaine2 et ne serait plus utilisable.
    // .clone() crée une copie profonde des données pour que chaine2 ait sa propre copie.
    let chaine2 = chaine1.clone(); 

    // On peut maintenant utiliser les deux variables car elles possèdent chacune leurs données.
    println!("chaine1: {}, chaine2: {}", chaine1, chaine2);
}