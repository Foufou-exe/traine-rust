// exo23/src/main.rs

fn main() {
    let nombre_magique = 5;
    let mon_nombre = 10;

    // Définition d'une closure qui prend un paramètre 'x'.
    // Particularité : elle "capture" la variable 'nombre_magique' de l'environnement englobant.
    // Une fonction classique (fn) ne pourrait pas faire ça sans passer la variable en paramètre.
    let calculateur = |x| x * nombre_magique;
    
    // Appel de la closure comme une fonction
    let resultat = calculateur(mon_nombre);

    println!("{} x {} = {}", nombre_magique, mon_nombre, resultat);
}
