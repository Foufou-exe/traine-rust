# 🦀 Learning Rust - Traine Rust

[![Français](https://img.shields.io/badge/lang-fr-blue.svg)](./README.fr.md)

Welcome to this repository dedicated to learning the Rust programming language. This project is structured as a progressive journey, starting from fundamental concepts to advanced topics like concurrency and memory management.

Each exercise is designed to isolate a specific concept, with detailed explanations and heavily commented source code.

## 📂 Course Structure

The repository is organized into thematic chapters to facilitate navigation:

### 1. [The Basics (01_Bases)](./01_Bases)
*   Variables, immutability, and mutability (`let`, `let mut`).
*   Scalar types (integers, booleans, characters).
*   Functions and return values.
*   Control flow (`if/else`, `loop`, `for`).

### 2. [Memory Management (02_Ownership)](./02_Ownership)
*   Ownership and Move semantics.
*   Borrowing with immutable and mutable references.
*   Lifetimes and annotations.

### 3. [Data Types (03_Structs_Enums)](./03_Structs_Enums)
*   Structures (`struct`) and method implementations (`impl`).
*   Enumerations (`enum`) and Pattern Matching (`match`).
*   The `Option<T>` enum for handling value absence.

### 4. [Standard Library (04_Collections_Erreurs)](./04_Collections_Erreurs)
*   Dynamic collections: Vectors (`Vec<T>`) and `HashMap`.
*   Recoverable error handling with `Result<T, E>`.
*   Using `unwrap` and `expect`.

### 5. [Advanced Concepts (05_Concepts_Avances)](./05_Concepts_Avances)
*   Code organization with modules (`mod`, `pub`).
*   Traits (Interfaces) and Generics.
*   Closures and functional iterators.
*   Smart Pointers (`Box<T>`).

### 6. [Ecosystem & Parallelism (06_Concurrence_Tests)](./06_Concurrence_Tests)
*   Multi-threading with `std::thread`.
*   Inter-thread communication via channels (`mpsc`).
*   Writing and executing unit tests.

### 7. [Async Programming (07_Async_Tokio)](./07_Async_Tokio)
*   Introduction to async with `Tokio` (`async`/`.await`).
*   Concurrent task management (`join!`, `select!`, `spawn`).
*   Shared state synchronization (`Arc`, `Mutex`).
*   Async communication channels (`mpsc`).
*   Non-blocking I/O (Files).

---

## 🚀 How to use this repo

### Prerequisites
*   Have [Rust](https://www.rust-lang.org/tools/install) installed (including `cargo`).

### Running an exercise
This repository is configured as a **Cargo Workspace**. You can run any exercise directly from the root without changing directories:

```bash
# Example to run exercise 1
cargo run -p exo1

# Example to run exercise 10
cargo run -p exo10
```

### Running tests
To verify your solutions (especially for exercise 28):

```bash
# Run tests for a specific package
cargo test -p exo28

# Run all workspace tests
cargo test --workspace
```

## 📝 Documentation
Each exercise has its own `README.md` describing the learning objectives and key takeaways. The source code is commented to explain the "why" behind each Rust instruction.

---
*Safe travels in the land of memory safety!* 🦀
