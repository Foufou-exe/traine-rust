// exo14/src/main.rs

// Définition d'une énumération avec 3 variantes possibles
enum Machine {
    Eteinte,
    Demarrage,
    Prete,
}

fn main() {
    let etat = Machine::Prete;

    // L'expression 'match' permet de comparer 'etat' à des modèles (patterns)
    // C'est très puissant et garantit que tous les cas sont traités
    match etat {
        Machine::Eteinte => println!("La machine ne répond pas."),
        Machine::Demarrage => println!("Initialisation en cours..."),
        Machine::Prete => println!("Système fonctionnel."),
    }
}