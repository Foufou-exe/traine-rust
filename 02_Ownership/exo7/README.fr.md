# Exercice 7 : Ownership (Possession) et Clone

## Objectif
Comprendre le concept central de Rust : l'Ownership. Quand une variable est assignée à une autre, la propriété est transférée (Move), sauf pour les types simples (Copy). Pour les types complexes comme `String`, il faut cloner si on veut dupliquer les données.

## Notions Abordées
*   Règle : Une valeur a un seul propriétaire à la fois.
*   Move semantics (Déplacement) pour les types alloués sur le tas (`String`).
*   Méthode `.clone()` pour copier les données en profondeur.
