// exo18/src/main.rs

fn main() {
    let entree = "50"; // Si on met "abc", le programme plantera (panic)
    
    // .expect() tente d'extraire la valeur Ok.
    // Si c'est une Err, le programme s'arrête net avec le message fourni.
    let nombre = entree.parse::<i32>().expect("Erreur de lecture : Ce n'est pas un entier !");
    
    println!("Le nombre est : {}", nombre);
}
