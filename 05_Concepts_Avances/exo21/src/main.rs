// exo21/src/main.rs

// Définition d'un trait 'Vehicule' qui impose la méthode 'vitesse'
trait Vehicule {
    fn vitesse(&self);
}

// Une structure vide (Unit Struct)
struct Voiture;

// Implémentation du trait 'Vehicule' pour la structure 'Voiture'
impl Vehicule for Voiture {
    fn vitesse(&self) {
        println!("La voiture roule à 100 km/h");
    }
}

fn main() {
    let ma_voiture = Voiture;
    
    // On peut appeler la méthode car Voiture implémente le trait Vehicule
    ma_voiture.vitesse();
}
