use std::sync::Arc;
use tokio::sync::Mutex; // Note: On utilise le Mutex de Tokio, pas celui de std pour ne pas bloquer le runtime

#[tokio::main]
async fn main() {
    // 1. Création de la donnée partagée.
    // Mutex : Verrouille l'accès (une seule personne à la fois).
    // Arc : Permet d'avoir plusieurs propriétaires de ce Mutex (nécessaire pour le passer à plusieurs threads/tâches).
    let compteur_global = Arc::new(Mutex::new(0));
    
    let mut handles = Vec::new();

    for i in 0..5 {
        // 2. On clone l'Arc pour créer une nouvelle "référence" vers la même mémoire.
        // C'est très léger (juste un compteur qui s'incrémente).
        let cpt_compteur = Arc::clone(&compteur_global);

        // 3. `tokio::spawn` lance une tâche en arrière-plan (green thread).
        // Le `move` transfère la propriété de `cpt_compteur` (le clone) à l'intérieur de la tâche.
        let handle = tokio::spawn(async move {
            // 4. Verrouillage asynchrone. Si c'est déjà verrouillé, la tâche "dort" (await)
            // sans bloquer le thread OS.
            let mut num = cpt_compteur.lock().await;
            
            // Section critique : on modifie la donnée
            *num += 1;
            println!("Guichet {} : Compteur à {}", i , num);
            
            // Le verrou est relâché automatiquement ici quand `num` sort du scope.
        });
        handles.push(handle)
    }

    // 5. On attend que toutes les tâches spawnées soient terminées.
    for handle in handles {
        handle.await.unwrap();
    }
}