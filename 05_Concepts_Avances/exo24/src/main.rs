// exo24/src/main.rs

fn main() {
    // Création d'un vecteur simple (macro vec!)
    let nombres = vec![1, 2, 3];
    
    // Chaîne d'opérations sur l'itérateur :
    // 1. .iter() : On parcourt les éléments.
    // 2. .map() : On applique une fonction (closure) à chaque élément (x + 1).
    // 3. .collect() : On rassemble les résultats dans un nouveau vecteur.
    let resultat: Vec<i32> = nombres.iter().map(|x| x + 1).collect();

    // Affichage avec le formateur de debug {:?}
    println!("{:?}", resultat);
}
