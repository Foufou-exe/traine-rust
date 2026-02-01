# Exercice 25 : Smart Pointers (Box)

## Objectif
Utiliser `Box<T>` pour allouer des données sur le tas (Heap) plutôt que sur la pile (Stack). C'est indispensable pour les structures de données récursives.

## Notions Abordées
*   `Box::new(v)` : Alloue `v` sur le tas et retourne un pointeur.
*   Structures récursives : Une taille infinie potentielle à la compilation est impossible sur la pile. `Box` a une taille fixe (celle d'un pointeur), ce qui permet de définir des types comme `Liste` qui contient une `Liste`.
