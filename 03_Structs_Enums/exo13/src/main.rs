// exo13/src/main.rs

struct Livre {
    titre: String,
    auteur: String,
    pages: u32,
}

// Le bloc 'impl' permet de définir des fonctions associées à la structure
impl Livre {
    // Cette méthode prend une référence à l'instance (&self)
    // Elle peut ainsi lire les données de l'instance qui l'appelle
    fn description(&self) {
        println!("{} écrit par {} ({} pages)", self.titre, self.auteur, self.pages);
    }
}

fn main() {
    let mon_livre = Livre {
        titre: String::from("Dune"),
        auteur: String::from("Herbert"),
        pages: 800,
    };
    
    // Appel de la méthode définie dans le bloc impl
    mon_livre.description();
}