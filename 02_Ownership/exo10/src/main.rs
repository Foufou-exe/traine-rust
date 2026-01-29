// exo10/src/main.rs

// La fonction attend une référence mutable (&mut String)
fn completer(x: &mut String) {
    x.push_str(" complet"); // Modification de la chaîne via la référence
}

fn main() {
    // La variable doit être mutable pour pouvoir créer des références mutables vers elle
    let mut phrase = String::from("Exercice");
    
    // Appel de la fonction avec une référence mutable
    completer(&mut phrase);
    
    // La modification est visible ici
    println!("{}", phrase);
}