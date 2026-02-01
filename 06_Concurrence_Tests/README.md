# Chapter 6: Concurrency and Tests

[![Français](https://img.shields.io/badge/lang-fr-blue.svg)](./README.fr.md)

## 📚 Theoretical Introduction

Rust is famous for its "Fearless Concurrency" promise. Its ownership rules mathematically prevent *Data Races* (unsynchronized concurrent access) at compile time.

### 1. Threads (`std::thread`)
A thread is an independent execution flow managed by the OS.
*   `thread::spawn` launches a new thread.
*   `join()` waits for it to finish.
*   **Move**: To use data from the main thread in a child thread, you often need to move them (`move`) to guarantee they live long enough.

### 2. Communication (`mpsc`)
"Do not communicate by sharing memory; instead, share memory by communicating." (Go Proverb, adopted by Rust).
*   **Channel**: A pipe with a sender (`Sender`) and a receiver (`Receiver`).
*   We send data from one thread to another safely.

### 3. Tests
Rust integrates tests into the core of the language.
*   `#[test]`: Attribute marking a function as a test.
*   `assert!`, `assert_eq!`: Macros to verify results.
*   `cargo test`: Magic command to run everything.

---

## 🛠️ Exercise Plan

### 🟢 System Concurrency
*   **[Exo 26 - Threads](./exo26)**: Creating threads and synchronizing their completion with `join`.
*   **[Exo 27 - Channels](./exo27)**: Communicating between two threads via an `mpsc` channel.

### 🟡 Code Quality
*   **[Exo 28 - Unit Tests](./exo28)**: Writing tests to ensure your code works and doesn't regress.

---

## 💡 Tip
If the compiler refuses your multithreaded code, thank it. In C++, this same code would likely have compiled and caused a random crash in production 3 months later.
