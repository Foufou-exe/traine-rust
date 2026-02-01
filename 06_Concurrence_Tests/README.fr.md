# Chapitre 6 : Concurrence et Tests

## 📚 Introduction Théorique

Rust est célèbre pour sa promesse de "Fearless Concurrency" (Concurrence sans peur). Ses règles de propriété empêchent mathématiquement les *Data Races* (accès concurrent non synchronisé) à la compilation.

### 1. Threads (`std::thread`)
Un thread est un fil d'exécution indépendant géré par l'OS.
*   `thread::spawn` lance un nouveau thread.
*   `join()` attend qu'il finisse.
*   **Move** : Pour utiliser des données du thread principal dans un thread enfant, il faut souvent les déplacer (`move`) pour garantir qu'elles vivent assez longtemps.

### 2. Communication (`mpsc`)
"Do not communicate by sharing memory; instead, share memory by communicating." (Go Proverb, adopté par Rust).
*   **Channel** : Un tuyau avec un émetteur (`Sender`) et un récepteur (`Receiver`).
*   On envoie des données d'un thread à l'autre en toute sécurité.

### 3. Tests
Rust intègre les tests au cœur du langage.
*   `#[test]` : Attribut qui marque une fonction comme étant un test.
*   `assert!`, `assert_eq!` : Macros pour vérifier les résultats.
*   `cargo test` : Commande magique pour tout lancer.

---

## 🛠️ Plan des Exercices

### 🟢 Concurrence Système
*   **[Exo 26 - Threads](./exo26)** : Créer des threads et synchroniser leur fin avec `join`.
*   **[Exo 27 - Canaux](./exo27)** : Faire communiquer deux threads via un canal `mpsc`.

### 🟡 Qualité Code
*   **[Exo 28 - Tests Unitaires](./exo28)** : Écrire des tests pour garantir que votre code fonctionne et ne régresse pas.

---

## 💡 Conseil
Si le compilateur refuse votre code multithreadé, remerciez-le. En C++, ce même code aurait probablement compilé et causé un crash aléatoire en production 3 mois plus tard.
