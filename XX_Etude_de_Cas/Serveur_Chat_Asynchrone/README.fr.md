# Étude de Cas : Serveur Chat Asynchrone

[![English](https://img.shields.io/badge/lang-en-red.svg)](./README.md)

## Objectif
Construire un serveur de chat multi-utilisateurs en temps réel en utilisant le runtime Tokio.

## Concepts Abordés
- **Réseau TCP** : Écoute et acceptation de connexions (`TcpListener`).
- **Tâches Tokio** : Gérer plusieurs clients simultanément avec `tokio::spawn`.
- **Canaux de Diffusion (Broadcast)** : Envoyer des messages à tous les utilisateurs connectés.
- **Select Asynchrone** : Gérer plusieurs flux asynchrones en même temps (`tokio::select!`).
- **I/O Splitting** : Séparer les opérations de lecture et d'écriture sur un socket.

## Comment l'utiliser
1. Lancez le serveur : `cargo run -p Serveur_Chat_Asynchrone`
2. Connectez-vous depuis un ou plusieurs terminaux : `telnet 127.0.0.1 8080`
