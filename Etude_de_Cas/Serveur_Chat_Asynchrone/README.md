# Case Study: Async Chat Server

[![Français](https://img.shields.io/badge/lang-fr-blue.svg)](./README.fr.md)

## Objective
Build a real-time multi-user chat server using the Tokio runtime.

## Concepts Covered
- **TCP Networking**: Listening for and accepting connections (`TcpListener`).
- **Tokio Tasks**: Managing multiple clients concurrently with `tokio::spawn`.
- **Broadcast Channels**: Dispatching messages to all connected users.
- **Async Select**: Handling multiple asynchronous streams simultaneously (`tokio::select!`).
- **I/O Splitting**: Separating reading and writing operations on a single socket.

## How to Run
1. Start the server: `cargo run -p Serveur_Chat_Asynchrone`
2. Connect from one or more terminals: `telnet 127.0.0.1 8080`
