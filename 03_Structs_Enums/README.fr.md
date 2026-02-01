# Chapitre 3 : Structuration des Données

## 📚 Introduction Théorique

Pour construire des logiciels complexes, il faut organiser les données. Rust propose deux outils principaux pour cela : les `structs` (données produits "ET") et les `enums` (données sommes "OU").

### 1. Structures (`struct`)
Permettent de regrouper plusieurs valeurs liées sous un même nom.
*   **Struct Classique** : Champs nommés (`struct User { name: String, age: u8 }`).
*   **Struct Tuple** : Champs sans nom, identifiés par position (`struct Color(i32, i32, i32)`).
*   **Unit Struct** : Sans champ (`struct AlwaysEqual;`), utile pour les traits.

### 2. Méthodes (`impl`)
On n'écrit pas de méthodes *dans* la struct, mais dans un bloc `impl` séparé.
*   `&self` : Méthode de lecture.
*   `&mut self` : Méthode qui modifie l'instance.
*   Sans `self` : Fonction associée (comme un constructeur `new`).

### 3. Énumérations (`enum`)
Permettent de définir un type qui peut être l'une des variantes listées. Contrairement à beaucoup de langages, les enums Rust peuvent **contenir des données**.
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 }, // Contient une struct anonyme
    Write(String),           // Contient une String
}
```

### 4. Le Pattern Matching (`match`)
L'outil le plus puissant de Rust pour le contrôle de flux. Il force à gérer **tous** les cas possibles d'une énumération.
*   `Option<T>` : L'enum standard pour gérer l'absence de valeur (`Some(valeur)` ou `None`). Remplace le `null`.

---

## 🛠️ Plan des Exercices

### 🟢 Structures & Méthodes
*   **[Exo 12 - Définir des Structs](./exo12)** : Créer et instancier des structures classiques.
*   **[Exo 13 - Méthodes](./exo13)** : Ajouter de la logique aux structures via les blocs `impl` et utiliser `self`.

### 🟡 Enums & Logique
*   **[Exo 14 - Énumérations](./exo14)** : Créer des types pouvant prendre plusieurs formes et y associer des données.
*   **[Exo 15 - Option & Match](./exo15)** : Découvrir `Option<T>` pour éviter les erreurs de nullité et utiliser `match` pour déballer les valeurs.

---

## 💡 Conseil
L'enum `Option` est omniprésent en Rust. Habituez-vous à penser en termes de "Valeur possible" vs "Rien", plutôt que de vérifier si une variable est "null".
