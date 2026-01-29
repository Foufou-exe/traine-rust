// exo8/src/main.rs

// Cette fonction prend la possession (ownership) de la String passée en paramètre.
// À la fin de la fonction, 'x' sera droppée (libérée).
fn afficher_message(x: String) {
    println!("{}", x);
}

fn main() {
    let mon_texte = String::from("Rust est strict"); 
    
    // On passe un clone de 'mon_texte' à la fonction.
    // Si on passait 'mon_texte' directement, on ne pourrait plus l'utiliser dans le println! suivant.
    afficher_message(mon_texte.clone());
    
    // Grâce au clone, 'mon_texte' est toujours valide ici.
    println!("Dans main : {}", mon_texte);
}