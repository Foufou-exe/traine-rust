# Exercice 27 : Les Canaux (Channels)

## Objectif
Communiquer entre threads de manière sécurisée par échange de messages.

## Notions Abordées
*   `mpsc::channel()` : Crée un canal (Multiple Producer, Single Consumer).
*   `tx` (Transmitter) : Pour envoyer des données.
*   `rx` (Receiver) : Pour recevoir des données.
*   `move` : Pour transférer la propriété de `tx` dans le thread.
