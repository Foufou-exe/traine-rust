# Exercice 32 : Course de Tâches (Select)

## Objectif
Gérer des scénarios de "course" où seule la première tâche qui termine nous intéresse (timeout, compétition entre sources, etc.).

## Concepts abordés
- La macro `tokio::select!`.
- Annulation implicite des tâches perdantes.
