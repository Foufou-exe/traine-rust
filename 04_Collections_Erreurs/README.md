# Chapter 4: Collections and Error Handling

[![Français](https://img.shields.io/badge/lang-fr-blue.svg)](./README.fr.md)

## 📚 Theoretical Introduction

Real-world programs must handle variable-sized data lists and cope with unforeseen situations (missing files, network errors). This chapter covers the standard tools for these needs.

### 1. Collections
Stored on the heap, their size can change at runtime.
*   **Vector (`Vec<T>`)**: Ordered list of elements of the same type. Fast access by index.
*   **String**: A collection of UTF-8 bytes. More complex than it looks due to encoding.
*   **HashMap (`HashMap<K, V>`)**: Key-Value storage. Fast access by key.

### 2. Error Handling
Rust distinguishes two types of errors:
1.  **Unrecoverable (`panic!`)**: Serious bug (index out of bounds). The program stops.
2.  **Recoverable (`Result<T, E>`)**: Expected failure (file not found). We must handle it.

### 3. The `Result` Enum
```rust
enum Result<T, E> {
    Ok(T),  // Success with the value
    Err(E), // Failure with the error
}
```
*   `unwrap()` / `expect()`: "Give me the value or crash if it's an error" (use with caution).
*   The `?` operator: "If error, return it immediately, otherwise give me the value". Allows chaining operations cleanly.

---

## 🛠️ Exercise Plan

### 🟢 Storing Data
*   **[Exo 16 - Vectors](./exo16)**: Creating, modifying, and iterating over dynamic lists.
*   **[Exo 17 - HashMap](./exo17)**: Associating keys and values, handling cases where the key doesn't exist.

### 🟡 Handling the Unexpected
*   **[Exo 18 - Panic vs Result](./exo18)**: Understanding when to crash (`panic!`) and when to return an error (`Result`).
*   **[Exo 19 - Error Propagation](./exo19)**: Writing robust code that bubbles up errors with the `?` operator.

---

## 💡 Tip
Don't sprinkle your code with `.unwrap()`. It's tempting for speed, but it's a ticking time bomb. Prefer `match` or `?` to handle error cases explicitly.
