# Chapter 1: Rust Basics

[![Français](https://img.shields.io/badge/lang-fr-blue.svg)](./README.fr.md)

## 📚 Theoretical Introduction

Rust is a modern systems programming language focusing on safety, speed, and concurrency. This first chapter lays the syntactic foundations needed to write your first programs.

### 1. Variables and Mutability
By default, in Rust, variables are **immutable**. Once a value is assigned, it cannot change. This is a design choice for safety and clarity.
*   `let x = 5;`: `x` cannot change.
*   `let mut x = 5;`: `x` can be modified (`mut` for mutable).

### 2. Strong and Static Typing
Rust needs to know the type of all variables at compile time. However, thanks to **type inference**, the compiler often guesses the type for you.
*   **Scalars**: Integers (`i32`, `u64`), Floats (`f64`), Booleans (`bool`), Characters (`char`).
*   **Compounds**: Tuples `(i32, f64)` and Arrays `[i32; 5]` (fixed size).

### 3. Functions
Functions are declared with `fn`. In Rust, the last expression of a block is implicitly returned (without a trailing semicolon).
```rust
fn addition(a: i32, b: i32) -> i32 {
    a + b  // No semicolon = return
}
```

### 4. Control Flow
*   **`if`**: It is an expression, not just a statement. You can do `let y = if x > 5 { 10 } else { 0 };`.
*   **`loop`**: An infinite loop (more idiomatic than `while true`).
*   **`while`**: Loop while a condition is true.
*   **`for`**: To iterate over a collection or a range (`0..5`).

---

## 🛠️ Exercise Plan

### 🟢 The Fundamentals
*   **[Exo 01 - Hello World](./exo1)**: Minimal program structure and the `println!` macro.
*   **[Exo 02 - Variables](./exo2)**: Declaration, mutability (`mut`), and shadowing.
*   **[Exo 03 - Data Types](./exo3)**: Manipulating numbers, booleans, and tuples.

### 🟡 Logic & Functions
*   **[Exo 04 - Functions](./exo4)**: Parameters, return values, and expressions.
*   **[Exo 05 - Conditions](./exo5)**: Using `if/else` as an expression to assign values.
*   **[Exo 06 - Loops](./exo6)**: Mastering `loop`, `while`, and `for`.

---

## 💡 Tip
In Rust, the compiler is your friend (sometimes a strict one). Read its error messages carefully; they often explain exactly how to fix the code!
