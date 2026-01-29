// exo20/src/main.rs

// Définition d'un module nommé 'outils'
mod outils {
    // Cette fonction est précédée de 'pub', elle est donc accessible depuis l'extérieur du module
    pub fn dire_bonjour() {
        println!("Bonjour depuis le module");
    }

    // Cette fonction n'a pas 'pub', elle est privée (visible uniquement dans le module 'outils')
    // Le compilateur émettra un avertissement car elle n'est jamais utilisée (dead code)
    fn au_revoir() {
        println!("Tu ne peux pas me voir !");
    }
}

fn main() {
    // Appel d'une fonction publique du module
    outils::dire_bonjour();
    
    // La ligne suivante provoquerait une erreur de compilation car au_revoir est privée
    // outils::au_revoir(); 
}
