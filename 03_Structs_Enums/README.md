# Chapter 3: Data Structuring

[![Français](https://img.shields.io/badge/lang-fr-blue.svg)](./README.fr.md)

## 📚 Theoretical Introduction

To build complex software, you need to organize data. Rust provides two main tools for this: `structs` ("AND" product types) and `enums` ("OR" sum types).

### 1. Structures (`struct`)
Allow grouping multiple related values under a single name.
*   **Classic Struct**: Named fields (`struct User { name: String, age: u8 }`).
*   **Tuple Struct**: Unnamed fields, identified by position (`struct Color(i32, i32, i32)`).
*   **Unit Struct**: No fields (`struct AlwaysEqual;`), useful for traits.

### 2. Methods (`impl`)
We don't write methods *inside* the struct, but in a separate `impl` block.
*   `&self`: Read method.
*   `&mut self`: Method that modifies the instance.
*   No `self`: Associated function (like a constructor `new`).

### 3. Enumerations (`enum`)
Allow defining a type that can be one of the listed variants. Unlike many languages, Rust enums can **contain data**.
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 }, // Contains an anonymous struct
    Write(String),           // Contains a String
}
```

### 4. Pattern Matching (`match`)
Rust's most powerful control flow tool. It forces you to handle **all** possible cases of an enumeration.
*   `Option<T>`: The standard enum for handling value absence (`Some(value)` or `None`). Replaces `null`.

---

## 🛠️ Exercise Plan

### 🟢 Structs & Methods
*   **[Exo 12 - Defining Structs](./exo12)**: Creating and instantiating classic structures.
*   **[Exo 13 - Methods](./exo13)**: Adding logic to structures via `impl` blocks and using `self`.

### 🟡 Enums & Logic
*   **[Exo 14 - Enumerations](./exo14)**: Creating types that can take multiple forms and associating data with them.
*   **[Exo 15 - Option & Match](./exo15)**: Discovering `Option<T>` to avoid null errors and using `match` to unwrap values.

---

## 💡 Tip
The `Option` enum is ubiquitous in Rust. Get used to thinking in terms of "Possible Value" vs "Nothing", rather than checking if a variable is "null".
