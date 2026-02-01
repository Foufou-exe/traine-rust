# Exercise 33: Shared State (Mutex)

[![Français](https://img.shields.io/badge/lang-fr-blue.svg)](./README.fr.md)

## Objective
Safely share memory (a variable) between multiple async tasks.

## Concepts Covered
- `Arc` (Atomic Reference Counting): For shared ownership.
- `tokio::sync::Mutex`: For exclusive access (async-aware).
- `tokio::spawn`: Spawning background tasks.
