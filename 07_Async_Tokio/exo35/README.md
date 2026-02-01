# Exercise 35: Async File I/O

[![Français](https://img.shields.io/badge/lang-fr-blue.svg)](./README.fr.md)

## Objective
Manipulate files (read/write) asynchronously to avoid blocking the server.

## Concepts Covered
- `tokio::fs` (Async equivalent of `std::fs`).
- `write`, `read_to_string`, `remove_file` operations.
