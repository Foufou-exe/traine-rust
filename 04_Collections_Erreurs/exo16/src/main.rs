// exo16/src/main.rs

fn main() {
    // Création d'un vecteur vide mutable pour stocker des entiers (i32)
    let mut notes: Vec<i32> = Vec::new();
    let mut total: i32 = 0;

    // Ajout dynamique d'éléments
    notes.push(10);
    notes.push(15);
    notes.push(20);

    // On itère sur une référence du vecteur (&notes) pour ne pas déplacer l'ownership (move)
    for n in &notes {
        // n est une référence (&i32), on doit le déréférencer (*) pour accéder à la valeur et l'additionner
        total += *n;
    }

    println!("Total des notes : {}", total);
}
