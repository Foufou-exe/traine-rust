// exo9/src/main.rs

// La fonction prend une référence vers une String (&String).
// Elle "emprunte" la valeur mais ne la possède pas. Elle ne sera pas droppée à la fin de la fonction.
fn calculer_taille(x: &String) -> usize {
    x.len() // On accède aux propriétés de la valeur empruntée
}

fn main() {
    let mon_texte = String::from("Rust");  // Création d'une chaîne
    
    // On passe une référence (&mon_texte) à la fonction.
    // Cela permet de garder l'ownership de 'mon_texte' dans le main.
    let taille = calculer_taille(&mon_texte); 
    
    println!("Le texte {} fait {} caractères.", mon_texte, taille); // mon_texte est toujours valide
}