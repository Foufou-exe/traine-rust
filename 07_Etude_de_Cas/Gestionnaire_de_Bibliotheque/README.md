# Gestionnaire de Bibliothèque

Ton objectif est de créer un petit système de gestion de bibliothèque.  
Tu vas devoir combiner : **Structs**, **Impl**, **Vec**, **Option** (ou **Result**) et **String**.

## Cahier des charges

### Structure Livre

- Champs : `titre` (String), `annee` (u32), `disponible` (bool).
- Le champ `disponible` doit être `true` à la création.

### Structure Bibliotheque

- Champs : `etageres` (un `Vec<Livre>`).

### Implémentation Bibliotheque

- `new()` : Crée une bibliothèque vide.
- `ajouter_livre(&mut self, ...)` : Ajoute un livre dans le vecteur.
- `emprunter_livre(&mut self, titre_recherche: &str) -> Option<String>` :
  - Parcourt le vecteur pour trouver le livre par son titre.
  - Si le livre existe **ET** est disponible : passe `disponible` à `false` et retourne `Some("Livre emprunté !")`.
  - Si le livre n'existe pas ou n'est pas dispo : retourne `None`.

## Dans le main

- Crée la bibliothèque.
- Ajoute 2 livres (ex: "Le Seigneur des Anneaux", "Harry Potter").
- Tente d'emprunter "Harry Potter" → Affiche "Succès" ou "Echec" via un `match`.
- Tente d'emprunter "Harry Potter" une deuxième fois (devrait échouer car indisponible).
- Tente d'emprunter un livre qui n'existe pas (devrait échouer).

---

Ce défi est plus long que les autres. Prends ton temps.  
C'est le moment de prouver que tu maîtrises Rust.
