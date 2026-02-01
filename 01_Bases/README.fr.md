# Chapitre 1 : Les Bases de Rust

## 📚 Introduction Théorique

Rust est un langage de programmation système moderne qui met l'accent sur la sécurité, la vitesse et la concurrence. Ce premier chapitre pose les fondations syntaxiques nécessaires pour écrire vos premiers programmes.

### 1. Variables et Mutabilité
Par défaut, en Rust, les variables sont **immuables**. Une fois une valeur assignée, elle ne peut plus changer. C'est un choix de conception pour la sécurité et la clarté.
*   `let x = 5;` : `x` ne peut pas changer.
*   `let mut x = 5;` : `x` peut être modifié (`mut` pour mutable).

### 2. Typage Fort et Statique
Rust doit connaître le type de toutes les variables à la compilation. Cependant, grâce à l'**inférence de type**, le compilateur devine souvent le type pour vous.
*   **Scalaires** : Entiers (`i32`, `u64`), Flottants (`f64`), Booléens (`bool`), Caractères (`char`).
*   **Composés** : Tuples `(i32, f64)` et Tableaux `[i32; 5]` (taille fixe).

### 3. Fonctions
Les fonctions sont déclarées avec `fn`. En Rust, la dernière expression d'un bloc est implicitement retournée (sans point-virgule final).
```rust
fn addition(a: i32, b: i32) -> i32 {
    a + b  // Pas de point-virgule = return
}
```

### 4. Contrôle de Flux
*   **`if`** : C'est une expression, pas juste une instruction. On peut faire `let y = if x > 5 { 10 } else { 0 };`.
*   **`loop`** : Une boucle infinie (plus idiomatique que `while true`).
*   **`while`** : Boucle tant qu'une condition est vraie.
*   **`for`** : Pour itérer sur une collection ou une plage (`0..5`).

---

## 🛠️ Plan des Exercices

### 🟢 Les Fondamentaux
*   **[Exo 01 - Hello World](./exo1)** : Structure minimale d'un programme et macro `println!`.
*   **[Exo 02 - Variables](./exo2)** : Déclaration, mutabilité (`mut`) et shadowing.
*   **[Exo 03 - Types de Données](./exo3)** : Manipulation des nombres, booléens et tuples.

### 🟡 Logique & Fonctions
*   **[Exo 04 - Fonctions](./exo4)** : Paramètres, valeurs de retour et expressions.
*   **[Exo 05 - Conditions](./exo5)** : Utilisation de `if/else` comme expression pour assigner des valeurs.
*   **[Exo 06 - Boucles](./exo6)** : Maîtriser `loop`, `while` et `for`.

---

## 💡 Conseil
En Rust, le compilateur est votre ami (parfois strict). Lisez attentivement ses messages d'erreur, ils expliquent souvent exactement comment corriger le code !
