# Chapter 2: Memory Management (Ownership)

[![Français](https://img.shields.io/badge/lang-fr-blue.svg)](./README.fr.md)

## 📚 Theoretical Introduction

This is where Rust stands out from other languages. Rust has no Garbage Collector (like Java/Python) and requires no manual management (like C/C++). It uses a system of **ownership rules** checked at compile time.

### 1. Ownership Rules
1.  Each value in Rust has a variable that is its **owner**.
2.  There can only be **one** owner at a time.
3.  When the owner goes out of scope (closing brace `}`), the value is **dropped** (cleaned up).

### 2. The "Move"
For complex types (like `String` or `Vec`), assigning one variable to another **moves** the ownership. The first variable becomes invalid.
```rust
let s1 = String::from("hello");
let s2 = s1; // s1 is dead, s2 is the new owner.
// println!("{}", s1); // ERROR!
```

### 3. Borrowing
To use a value without taking ownership, we use references (`&`).
*   **Immutable Reference (`&T`)**: Read-only. You can have infinite immutable references simultaneously.
*   **Mutable Reference (`&mut T`)**: Read/Write. You can have **only one** at a time (and no immutable ones in parallel).
*   *Golden Rule*: "Either many readers, or one writer".

### 4. Slices
Slices are references to a contiguous part of a collection (e.g., part of a String or an array) without copying data.

---

## 🛠️ Exercise Plan

### 🟢 Understanding Ownership
*   **[Exo 07 - Ownership](./exo7)**: The unique owner concept and automatic cleanup.
*   **[Exo 08 - Move Semantics](./exo8)**: Understanding why and how data is moved (Move) or copied (Copy).

### 🟡 Mastering Borrows
*   **[Exo 09 - References](./exo9)**: Passing values to functions without losing ownership (Immutable Borrow).
*   **[Exo 10 - Mutable References](./exo10)**: Modifying a borrowed value (`&mut`). Understanding concurrency restrictions.

### 🟠 Advanced Concepts
*   **[Exo 11 - Slices](./exo11)**: Working on sub-parts of strings (`&str`) or arrays.

---

## 💡 Tip
If you are fighting the "Borrow Checker", take a deep breath. It prevents memory errors (dangling pointers, data races) that would be nightmares to debug later.
