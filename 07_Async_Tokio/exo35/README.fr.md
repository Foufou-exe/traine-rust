# Exercice 35 : Système de Fichiers (File IO)

## Objectif
Manipuler des fichiers (lecture/écriture) de manière asynchrone pour ne pas bloquer le serveur pendant les accès disque.

## Concepts abordés
- `tokio::fs` (équivalent asynchrone de `std::fs`).
- Opérations `write`, `read_to_string`, `remove_file`.
