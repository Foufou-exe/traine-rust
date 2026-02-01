# Exercice 29 : Introduction à l'Asynchrone

## Objectif
Comprendre la différence fondamentale entre le code bloquant (synchrone) et le code non-bloquant (asynchrone).

## Concepts abordés
- La macro `#[tokio::main]` qui initialise le runtime.
- La syntaxe `async` / `.await`.
- La fonction `tokio::time::sleep` vs `std::thread::sleep`.
