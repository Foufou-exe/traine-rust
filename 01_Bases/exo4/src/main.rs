// exo4/src/main.rs

fn main() {
    let age: i32 = 18; 
    
    // 'if' est une expression en Rust : on peut stocker son résultat dans une variable.
    // Chaque branche du if/else doit retourner le même type de données (ici &str, une tranche de chaîne).
    let categorie = if age >= 18 {
        "Majeur"
    } else {
        "Mineur"
    };
    
    println!("Statut : {}", categorie);
}