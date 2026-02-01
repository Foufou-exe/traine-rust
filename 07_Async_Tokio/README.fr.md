# Chapitre 7 : Programmation Asynchrone avec Tokio

## 📚 Introduction Théorique

La programmation asynchrone est un modèle d'exécution qui permet à un programme de gérer plusieurs tâches simultanément sans bloquer le fil d'exécution principal (thread). En Rust, l'asynchrone est **coopératif** et repose sur le système de `Future`.

### Pourquoi l'Asynchrone ?

Imaginez un serveur web qui reçoit une requête. Pour y répondre, il doit interroger une base de données.
*   **En Synchrone (Bloquant)** : Le serveur envoie la requête à la BDD et *attend* (ne fait rien d'autre) jusqu'à avoir la réponse. Si 1000 utilisateurs arrivent, il faut 1000 threads (très lourd).
*   **En Asynchrone (Non-Bloquant)** : Le serveur envoie la requête et dit "Préviens-moi quand c'est fini". En attendant, il s'occupe des autres utilisateurs. Un seul thread peut gérer des milliers de connexions.

### Le Rôle de Tokio

Rust fournit uniquement les briques de base (`async`, `.await`, `Future`) dans sa bibliothèque standard, mais **pas de runtime** pour exécuter ce code.
C'est là qu'intervient **Tokio**. C'est le runtime asynchrone le plus populaire en Rust. Il fournit :
*   Un planificateur (scheduler) pour gérer les tâches.
*   Des outils d'E/S (Réseau, Fichiers) non-bloquants.
*   Des timers, des canaux de communication, etc.

---

## 🔑 Concepts Clés

### 1. `async` et `.await`
*   **`async fn`** : Transforme une fonction en une machine à états qui retourne une `Future`. Le code à l'intérieur ne s'exécute pas tout de suite !
*   **`.await`** : C'est le point de bascule. Il dit au runtime : "Mets cette tâche en pause tant que le résultat n'est pas prêt, et va faire autre chose en attendant".

### 2. Le Runtime
On utilise généralement la macro `#[tokio::main]` pour transformer la fonction `main` classique en un point d'entrée asynchrone capable de lancer le runtime.

```rust
#[tokio::main]
async fn main() {
    // Le monde async commence ici
}
```

### 3. Concurrence vs Parallélisme
*   **Concurrence** (Async) : Avancer plusieurs tâches sur un même thread en alternant les temps d'attente (comme un jongleur avec des balles).
*   **Parallélisme** (Threads) : Faire plusieurs choses exactement en même temps sur plusieurs cœurs CPU.
*   *Tokio fait souvent les deux : il est multithreadé par défaut.*

---

## 🛠️ Plan des Exercices

Ce module est progressif. Suivez l'ordre pour bien assimiler les concepts :

### 🟢 Niveau Débutant : Les Fondamentaux
*   **[Exo 29 - Hello Async](./exo29)** : Comprendre la différence entre `std::thread::sleep` (bloquant) et `tokio::time::sleep` (non-bloquant).
*   **[Exo 30 - Futures & Retours](./exo30)** : Comment récupérer des valeurs depuis une fonction asynchrone.

### 🟡 Niveau Intermédiaire : Flux de Contrôle
*   **[Exo 31 - Join](./exo31)** : Lancer plusieurs tâches **en même temps** et attendre qu'elles finissent toutes (`tokio::join!`).
*   **[Exo 32 - Select](./exo32)** : La course ! Attendre la première tâche qui finit et annuler les autres (`tokio::select!`).

### 🟠 Niveau Avancé : Communication & État
*   **[Exo 33 - État Partagé (Mutex)](./exo33)** : Modifier une variable commune depuis plusieurs tâches sans corruption mémoire (`Arc<Mutex<T>>`).
*   **[Exo 34 - Canaux (Channels)](./exo34)** : Faire dialoguer des tâches via des messages (Producteur / Consommateur).

### 🔴 Niveau Expert : Entrées/Sorties (I/O)
*   **[Exo 35 - Fichiers Async](./exo35)** : Lire et écrire des fichiers sans bloquer le runtime (`tokio::fs`).
*   **[Exo 36 - Gestion d'Erreurs](./exo36)** : Utiliser l'opérateur `?` et `Result` dans un contexte asynchrone.

---

## 💡 Conseil pour réussir
L'asynchrone en Rust change la façon de penser le flux du programme.
Si le compilateur vous dit *"future cannot be shared between threads safely"*, c'est souvent un problème de durée de vie ou de type non `Send`.
Prenez le temps de lire les commentaires dans chaque exercice !
