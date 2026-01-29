// exo25/src/main.rs

// L'attribut derive(Debug) permet d'afficher la structure avec {:?}
#[derive(Debug)]
enum Liste {
    // Le variant Element contient une valeur (i32) et... une autre Liste !
    // Comme une Liste peut être infinie, Rust ne peut pas connaître sa taille à la compilation.
    // On utilise Box<Liste> (un pointeur) car la taille d'un pointeur est connue et fixe.
    Element(i32, Box<Liste>),
    Vide,
}

fn main() {
    // Création d'une liste chaînée : 10 -> Vide
    let ma_liste = Liste::Element(10, Box::new(Liste::Vide));
    
    println!("{:?}", ma_liste);
}
