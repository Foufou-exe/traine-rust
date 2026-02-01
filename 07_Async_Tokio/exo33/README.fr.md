# Exercice 33 : État Partagé (Mutex)

## Objectif
Comprendre comment partager de la mémoire (une variable) entre plusieurs tâches asynchrones en toute sécurité.

## Concepts abordés
- `Arc` (Atomic Reference Counting) : Pour que plusieurs tâches "possèdent" la variable.
- `tokio::sync::Mutex` : Pour s'assurer qu'une seule tâche modifie la variable à la fois.
- `tokio::spawn` : Pour lancer des tâches en arrière-plan (thread vert).
