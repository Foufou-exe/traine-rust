// exo12/src/main.rs

// Définition de la structure 'Livre' qui regroupe 3 champs
struct Livre {
    titre: String,
    auteur: String,
    pages: u32,
}

fn main() {
    // Création d'une instance de la structure Livre
    let mon_livre = Livre {
        titre: String::from("1984"),
        auteur: String::from("Orwell"),
        pages: 328,
    };
    
    // Accès aux champs de la structure
    println!("Livre : {}, écrit par {} ({} pages)", mon_livre.titre, mon_livre.auteur, mon_livre.pages);
}