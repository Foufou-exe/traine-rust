// exo3/src/main.rs

// Déclaration d'une fonction qui prend un entier 32 bits (i32) et retourne un i32
fn carre(x: i32) -> i32 {
    // En Rust, la dernière expression d'un bloc est la valeur de retour
    // Notez l'absence de point-virgule ici (c'est une expression, pas une instruction)
    x * x
}

fn main() {
    let nombre = 5;
    
    // Appel de la fonction et stockage du résultat
    let resultat = carre(nombre);
    
    println!("Le carré de {} est {}", nombre, resultat);
}
