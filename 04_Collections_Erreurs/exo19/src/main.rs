// exo19/src/main.rs

// HashMap n'est pas inclus par défaut, il faut l'importer
use std::collections::HashMap;

fn main() {
    let mut stock = HashMap::new();

    // Insertion de données (Clé: String -> Valeur: i32)
    stock.insert(String::from("Chaise"), 5);
    stock.insert(String::from("Table"), 2);
    
    // Suppression d'une entrée par sa clé
    stock.remove("Chaise");

    // .get() retourne une Option<&V> car la clé peut ne pas exister.
    // Ici on utilise unwrap() car on suppose que "Table" existe.
    // Note : Pour un code robuste, on utiliserait un match ou if let.
    println!("Il reste {} Tables", stock.get("Table").unwrap());
}
