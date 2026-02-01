# Chapitre 5 : Concepts Avancés

## 📚 Introduction Théorique

Une fois les bases maîtrisées, ces concepts vous permettent d'écrire du code Rust idiomatique, réutilisable et puissant. C'est ici que l'on passe du statut de "débutant" à "intermédiaire".

### 1. Génériques et Traits
*   **Génériques (`<T>`)** : Écrire du code qui fonctionne avec plusieurs types (ex: `Vec<T>` fonctionne pour `Vec<i32>` et `Vec<String>`).
*   **Traits** : Définissent un comportement commun (comme les interfaces en Java/C#).
    *   Exemple : Le trait `Display` permet d'afficher un objet. Si ma struct implémente `Display`, elle peut être affichée.
    *   Bounds : `fn afficher<T: Display>(item: T)` -> "J'accepte n'importe quel type T, tant qu'il est affichable".

### 2. Programmation Fonctionnelle
*   **Closures** : Des fonctions anonymes qui peuvent "capturer" leur environnement. `|x| x + 1`.
*   **Itérateurs** : Permettent de traiter des séquences d'éléments (map, filter, fold) de manière paresseuse et efficace.

### 3. Smart Pointers (Pointeurs Intelligents)
Contrairement aux références (`&`), les Smart Pointers possèdent souvent la donnée qu'ils pointent.
*   **`Box<T>`** : Alloue des données sur le tas (heap) au lieu de la pile (stack). Utile pour les types récursifs ou de taille inconnue.
*   D'autres existent (`Rc`, `RefCell`) pour des cas plus spécifiques.

---

## 🛠️ Plan des Exercices

### 🟢 Abstraction
*   **[Exo 20 - Génériques](./exo20)** : Créer des fonctions et structs capables de gérer n'importe quel type.
*   **[Exo 21 - Traits](./exo21)** : Définir et implémenter des comportements partagés.
*   **[Exo 22 - Lifetimes](./exo22)** : (Le boss de fin) Comprendre et annoter les durées de vie des références.

### 🟡 Fonctionnel & Mémoire
*   **[Exo 23 - Closures](./exo23)** : Utiliser des fonctions anonymes et comprendre la capture de variables.
*   **[Exo 24 - Itérateurs](./exo24)** : Manipuler des collections avec `map`, `filter` et `collect`.
*   **[Exo 25 - Smart Pointers](./exo25)** : Utiliser `Box<T>` pour gérer la mémoire manuellement (mais sûrement).

---

## 💡 Conseil
Les Traits sont le ciment de l'écosystème Rust. Presque tout passe par eux (`Debug`, `Clone`, `Iterator`, `Future`). Les comprendre est essentiel pour lire la documentation des bibliothèques.
