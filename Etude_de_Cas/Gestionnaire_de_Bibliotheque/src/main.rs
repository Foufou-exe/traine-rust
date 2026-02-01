/// Structure représentant un Livre.
struct Livre {
    titre: String,
    annee: u32,
    disponible: bool
}

/// Structure représentant la Bibliothèque, qui possède une collection de livres.
struct Bibliotheque {
    etageres: Vec<Livre>
}

impl Bibliotheque {
    /// Constructeur pour créer une nouvelle bibliothèque vide.
    fn new() -> Bibliotheque {
        Bibliotheque {
            etageres: Vec::new(),
        }
    }

    /// Ajoute un livre à la collection. Prend possession du livre.
    fn ajouter_livre(&mut self, livre: Livre) {
        self.etageres.push(livre);
    }

    /// Tente d'emprunter un livre par son titre.
    /// Retourne Some(message) si réussi, None si le livre est absent ou déjà emprunté.
    fn emprunter_livre(&mut self, titre_rechercher: &str) -> Option<String> {
        for livre in &mut self.etageres  {
            if titre_rechercher == livre.titre {
                if livre.disponible {
                    livre.disponible = false;
                    return Some(String::from("Livre emprunté !"));
                } else {
                    return None; // Déjà emprunté
                }
            }
        }
        None // Non trouvé
    }
}

fn main() {
    let mut ma_biblio = Bibliotheque::new();

    // Création de quelques livres
    let livre1 = Livre { titre: String::from("Le Seigneur des Anneaux"), annee: 2000, disponible: true };
    let livre2 = Livre { titre: String::from("Harry Potter"), annee: 2001, disponible: true };

    // Ajout à la bibliothèque
    ma_biblio.ajouter_livre(livre1);
    ma_biblio.ajouter_livre(livre2);

    // Scénarios de test
    println!("Action: Emprunter 'Le Seigneur des Anneaux'");
    let result = ma_biblio.emprunter_livre("Le Seigneur des Anneaux");
    println!("Résultat: {:?}", result);

    println!("Action: Emprunter 'Harry Potter'");
    let result2 = ma_biblio.emprunter_livre("Harry Potter");
    println!("Résultat: {:?}", result2);

    println!("Action: Tenter d'emprunter à nouveau 'Le Seigneur des Anneaux'");
    let result = ma_biblio.emprunter_livre("Le Seigneur des Anneaux");
    println!("Résultat: {:?}", result);
}