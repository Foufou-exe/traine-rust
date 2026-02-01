# Chapitre 2 : La Gestion de la Mémoire (Ownership)

## 📚 Introduction Théorique

C'est ici que Rust se distingue de tous les autres langages. Rust n'a pas de Garbage Collector (comme Java/Python) et ne demande pas de gestion manuelle (comme C/C++). Il utilise un système de **règles de possession** vérifiées à la compilation.

### 1. Les Règles de l'Ownership
1.  Chaque valeur en Rust a une variable qui est son **propriétaire** (owner).
2.  Il ne peut y avoir qu'**un seul** propriétaire à la fois.
3.  Quand le propriétaire sort du scope (accolade fermante `}`), la valeur est **nettoyée** (dropped).

### 2. Le "Move" (Déplacement)
Pour les types complexes (comme `String` ou `Vec`), assigner une variable à une autre **déplace** la propriété. La première variable devient invalide.
```rust
let s1 = String::from("hello");
let s2 = s1; // s1 est mort, s2 est le nouveau propriétaire.
// println!("{}", s1); // ERREUR !
```

### 3. Le Borrowing (Emprunt)
Pour utiliser une valeur sans en prendre la propriété, on utilise des références (`&`).
*   **Référence immuable (`&T`)** : Lecture seule. On peut en avoir une infinité en même temps.
*   **Référence mutable (`&mut T`)** : Lecture/Écriture. On ne peut en avoir qu'**une seule** à la fois (et aucune immuable en parallèle).
*   *Règle d'or* : "Soit plusieurs lecteurs, soit un seul écrivain".

### 4. Les Slices
Les slices (tranches) sont des références vers une partie contiguë d'une collection (ex: une partie d'une String ou d'un tableau), sans copier les données.

---

## 🛠️ Plan des Exercices

### 🟢 Comprendre la Possession
*   **[Exo 07 - Ownership](./exo7)** : Le concept de propriétaire unique et le nettoyage automatique.
*   **[Exo 08 - Move Semantics](./exo8)** : Comprendre pourquoi et comment les données sont déplacées (Move) ou copiées (Copy).

### 🟡 Maîtriser les Emprunts
*   **[Exo 09 - Références](./exo9)** : Passer des valeurs à des fonctions sans perdre la propriété (Emprunt immuable).
*   **[Exo 10 - Mutable References](./exo10)** : Modifier une valeur empruntée (`&mut`). Comprendre les restrictions de concurrence.

### 🟠 Concepts Avancés
*   **[Exo 11 - Slices](./exo11)** : Travailler sur des sous-parties de chaînes de caractères (`&str`) ou de tableaux.

---

## 💡 Conseil
Si vous vous battez contre le "Borrow Checker", respirez un grand coup. Il vous empêche de faire des erreurs mémoire (dangling pointers, data races) qui seraient cauchemardesques à déboguer plus tard.
