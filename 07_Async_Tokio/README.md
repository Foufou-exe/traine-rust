# Chapter 7: Async Programming with Tokio

[![Français](https://img.shields.io/badge/lang-fr-blue.svg)](./README.fr.md)

## 📚 Theoretical Introduction

Asynchronous programming is an execution model that allows a program to handle multiple tasks simultaneously without blocking the main execution thread. In Rust, async is **cooperative** and relies on the `Future` system.

### Why Async?

Imagine a web server receiving a request. To respond, it must query a database.
*   **Synchronous (Blocking)**: The server sends the query to the DB and *waits* (does nothing else) until it gets the answer. If 1000 users arrive, you need 1000 threads (very heavy).
*   **Asynchronous (Non-Blocking)**: The server sends the query and says "Notify me when it's done". Meanwhile, it handles other users. A single thread can handle thousands of connections.

### The Role of Tokio

Rust only provides the building blocks (`async`, `.await`, `Future`) in its standard library, but **no runtime** to execute this code.
This is where **Tokio** comes in. It is the most popular async runtime in Rust. It provides:
*   A scheduler to manage tasks.
*   Non-blocking I/O tools (Network, Files).
*   Timers, communication channels, etc.

---

## 🔑 Key Concepts

### 1. `async` and `.await`
*   **`async fn`**: Transforms a function into a state machine that returns a `Future`. The code inside doesn't execute immediately!
*   **`.await`**: This is the yielding point. It tells the runtime: "Pause this task until the result is ready, and go do something else in the meantime".

### 2. The Runtime
We generally use the `#[tokio::main]` macro to transform the classic `main` function into an async entry point capable of launching the runtime.

```rust
#[tokio::main]
async fn main() {
    // The async world starts here
}
```

### 3. Concurrency vs Parallelism
*   **Concurrency** (Async): Advancing multiple tasks on a single thread by alternating wait times (like a juggler with balls).
*   **Parallelism** (Threads): Doing multiple things exactly at the same time on multiple CPU cores.
*   *Tokio often does both: it is multithreaded by default.*

---

## 🛠️ Exercise Plan

This module is progressive. Follow the order to fully grasp the concepts:

### 🟢 Beginner Level: Fundamentals
*   **[Exo 29 - Hello Async](./exo29)**: Understanding the difference between `std::thread::sleep` (blocking) and `tokio::time::sleep` (non-blocking).
*   **[Exo 30 - Futures & Returns](./exo30)**: How to retrieve values from an async function.

### 🟡 Intermediate Level: Control Flow
*   **[Exo 31 - Join](./exo31)**: Launching multiple tasks **at the same time** and waiting for them all to finish (`tokio::join!`).
*   **[Exo 32 - Select](./exo32)**: The race! Waiting for the first task to finish and cancelling the others (`tokio::select!`).

### 🟠 Advanced Level: Communication & State
*   **[Exo 33 - Shared State (Mutex)](./exo33)**: Modifying a common variable from multiple tasks without memory corruption (`Arc<Mutex<T>>`).
*   **[Exo 34 - Channels](./exo34)**: Making tasks talk via messages (Producer / Consumer).

### 🔴 Expert Level: Input/Output (I/O)
*   **[Exo 35 - Async Files](./exo35)**: Reading and writing files without blocking the runtime (`tokio::fs`).
*   **[Exo 36 - Error Handling](./exo36)**: Using the `?` operator and `Result` in an async context.

---

## 💡 Tip for Success
Async in Rust changes how you think about program flow.
If the compiler tells you *"future cannot be shared between threads safely"*, it's often a lifetime issue or a non-`Send` type.
Take the time to read the comments in each exercise!
