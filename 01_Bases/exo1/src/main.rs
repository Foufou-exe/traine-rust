// exo1/src/main.rs

fn main() {
    // En Rust, les variables sont immuables par défaut.
    // Le mot-clé 'mut' est nécessaire pour autoriser la modification de la valeur par la suite.
    let mut annee = 2023;
    println!("Nous sommes en {}", annee);

    // On modifie la valeur de la variable 'annee'.
    // Cela ne serait pas possible sans 'mut' lors de la déclaration.
    annee = 2024;
    println!("Maintenant, nous sommes en {}", annee);
}