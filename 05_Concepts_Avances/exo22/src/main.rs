// exo22/src/main.rs

// La structure Couple peut contenir deux éléments de n'importe quel type 'T'
// Mais les deux éléments 'a' et 'b' doivent être du MÊME type.
struct Couple<T> {
    a: T,
    b: T,
}

fn main() {
    // Ici T est inféré comme étant i32 (entier)
    let couple_int = Couple { a: 5, b: 5 };
    
    // Ici T est inféré comme étant f64 (flottant)
    let couple_float = Couple { a: 5.5, b: 1.5 };

    println!("Int: {}, Float: {}", couple_int.a, couple_float.b);
}
