# Chapitre 4 : Collections et Gestion d'Erreurs

## 📚 Introduction Théorique

Les programmes réels doivent gérer des listes de données de taille variable et faire face à des situations imprévues (fichiers manquants, erreurs réseau). Ce chapitre couvre les outils standards pour ces besoins.

### 1. Les Collections
Stockées dans le tas (heap), leur taille peut changer à l'exécution.
*   **Vecteur (`Vec<T>`)** : Liste ordonnée d'éléments du même type. Accès rapide par index.
*   **String** : Une collection d'octets UTF-8. Plus complexe qu'il n'y paraît à cause de l'encodage.
*   **HashMap (`HashMap<K, V>`)** : Stockage Clé-Valeur. Accès rapide par clé.

### 2. Gestion d'Erreurs
Rust distingue deux types d'erreurs :
1.  **Irrécupérables (`panic!`)** : Bug grave (index hors limites). Le programme s'arrête.
2.  **Récupérables (`Result<T, E>`)** : Échec prévu (fichier non trouvé). On doit le gérer.

### 3. L'enum `Result`
```rust
enum Result<T, E> {
    Ok(T),  // Succès avec la valeur
    Err(E), // Échec avec l'erreur
}
```
*   `unwrap()` / `expect()` : "Donne-moi la valeur ou crashe si c'est une erreur" (à utiliser avec prudence).
*   L'opérateur `?` : "Si erreur, retourne-la tout de suite, sinon donne-moi la valeur". Permet d'enchaîner les opérations proprement.

---

## 🛠️ Plan des Exercices

### 🟢 Stocker des Données
*   **[Exo 16 - Vecteurs](./exo16)** : Créer, modifier et itérer sur des listes dynamiques.
*   **[Exo 17 - HashMap](./exo17)** : Associer des clés et des valeurs, gérer les cas où la clé n'existe pas.

### 🟡 Gérer l'Imprévu
*   **[Exo 18 - Panic vs Result](./exo18)** : Comprendre quand planter (`panic!`) et quand retourner une erreur (`Result`).
*   **[Exo 19 - Propagation d'Erreur](./exo19)** : Écrire du code robuste qui fait remonter les erreurs avec l'opérateur `?`.

---

## 💡 Conseil
Ne parsemez pas votre code de `.unwrap()`. C'est tentant pour aller vite, mais c'est une bombe à retardement. Préférez `match` ou `?` pour gérer les cas d'erreur explicitement.
