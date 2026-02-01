# Chapter 5: Advanced Concepts

[![Français](https://img.shields.io/badge/lang-fr-blue.svg)](./README.fr.md)

## 📚 Theoretical Introduction

Once the basics are mastered, these concepts allow you to write idiomatic, reusable, and powerful Rust code. This is where you move from "beginner" to "intermediate" status.

### 1. Generics and Traits
*   **Generics (`<T>`)**: Writing code that works with multiple types (e.g., `Vec<T>` works for `Vec<i32>` and `Vec<String>`).
*   **Traits**: Define common behavior (like interfaces in Java/C#).
    *   Example: The `Display` trait allows an object to be printed. If my struct implements `Display`, it can be printed.
    *   Bounds: `fn display<T: Display>(item: T)` -> "I accept any type T, as long as it is printable".

### 2. Functional Programming
*   **Closures**: Anonymous functions that can "capture" their environment. `|x| x + 1`.
*   **Iterators**: Allow processing sequences of elements (map, filter, fold) lazily and efficiently.

### 3. Smart Pointers
Unlike references (`&`), Smart Pointers often own the data they point to.
*   **`Box<T>`**: Allocates data on the heap instead of the stack. Useful for recursive types or unknown sizes.
*   Others exist (`Rc`, `RefCell`) for more specific cases.

---

## 🛠️ Exercise Plan

### 🟢 Abstraction
*   **[Exo 20 - Generics](./exo20)**: Creating functions and structs capable of handling any type.
*   **[Exo 21 - Traits](./exo21)**: Defining and implementing shared behaviors.
*   **[Exo 22 - Lifetimes](./exo22)**: (The final boss) Understanding and annotating reference lifetimes.

### 🟡 Functional & Memory
*   **[Exo 23 - Closures](./exo23)**: Using anonymous functions and understanding variable capture.
*   **[Exo 24 - Iterators](./exo24)**: Manipulating collections with `map`, `filter`, and `collect`.
*   **[Exo 25 - Smart Pointers](./exo25)**: Using `Box<T>` to manage memory manually (but safely).

---

## 💡 Tip
Traits are the cement of the Rust ecosystem. Almost everything goes through them (`Debug`, `Clone`, `Iterator`, `Future`). Understanding them is essential for reading library documentation.
