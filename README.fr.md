# 🦀 Apprentissage de Rust - Traine Rust

<div align="center">

[EN](./README.md) | [FR](./README.fr.md)

</div>

Bienvenue dans ce dépôt dédié à l'apprentissage du langage Rust. Ce projet est structuré comme un parcours progressif, allant des concepts fondamentaux jusqu'aux notions avancées comme la concurrence et la gestion de la mémoire.

Chaque exercice est conçu pour isoler une notion précise, avec des explications détaillées et un code source abondamment commenté en français.

## 📂 Structure du Parcours

Le dépôt est organisé en chapitres thématiques pour faciliter la navigation :

### 1. [Les Bases (01_Bases)](./01_Bases)
*   Variables, immuabilité et mutabilité (`let`, `let mut`).
*   Types scalaires (entiers, booléens, caractères).
*   Fonctions et valeurs de retour.
*   Structures de contrôle (`if/else`, `loop`, `for`).

### 2. [Gestion de la Mémoire (02_Ownership)](./02_Ownership)
*   Ownership (Possession) et Move semantics.
*   Borrowing (Emprunt) avec les références immuables et mutables.
*   Lifetimes (Durées de vie) et annotations.

### 3. [Types de Données (03_Structs_Enums)](./03_Structs_Enums)
*   Structures (`struct`) et implémentations de méthodes (`impl`).
*   Énumérations (`enum`) et Pattern Matching (`match`).
*   L'énumération `Option<T>` pour la gestion de l'absence de valeur.

### 4. [Bibliothèque Standard (04_Collections_Erreurs)](./04_Collections_Erreurs)
*   Collections dynamiques : Vecteurs (`Vec<T>`) et `HashMap`.
*   Gestion d'erreurs récupérables avec `Result<T, E>`.
*   Utilisation de `unwrap` et `expect`.

### 5. [Concepts Avancés (05_Concepts_Avances)](./05_Concepts_Avances)
*   Organisation du code en modules (`mod`, `pub`).
*   Traits (Interfaces) et Génériques.
*   Closures et itérateurs fonctionnels.
*   Smart Pointers (`Box<T>`).

### 6. [Écosystème & Parallélisme (06_Concurrence_Tests)](./06_Concurrence_Tests)
*   Multi-threading avec `std::thread`.
*   Communication entre threads via les canaux (`mpsc`).
*   Écriture et exécution de tests unitaires.

### 7. [Programmation Asynchrone (07_Async_Tokio)](./07_Async_Tokio)
*   Introduction à l'asynchrone avec `Tokio` (`async`/`.await`).
*   Gestion des tâches concurrentes (`join!`, `select!`, `spawn`).
*   Synchronisation d'états partagés (`Arc`, `Mutex`).
*   Canaux de communication asynchrones (`mpsc`).
*   Entrées/Sorties non-bloquantes (Fichiers).

---

## 🚀 Comment utiliser ce dépôt

### Prérequis
*   Avoir [Rust](https://www.rust-lang.org/tools/install) installé (incluant `cargo`).

### Exécuter un exercice
Ce dépôt est configuré comme un **Cargo Workspace**. Vous pouvez exécuter n'importe quel exercice directement depuis la racine sans avoir à changer de dossier :

```bash
# Exemple pour lancer l'exercice 1
cargo run -p exo1

# Exemple pour lancer l'exercice 10
cargo run -p exo10
```

### Lancer les tests
Pour vérifier la validité de vos solutions (notamment pour l'exercice 28) :

```bash
# Lancer les tests d'un paquet spécifique
cargo test -p exo28

# Lancer tous les tests du workspace
cargo test --workspace
```

## 📝 Documentation
Chaque exercice possède son propre fichier `README.md` décrivant les objectifs pédagogiques et les points clés à retenir. Le code source est commenté pour expliquer le "pourquoi" derrière chaque instruction Rust.

---
*Bon voyage au pays de la sécurité mémoire !* 🦀
