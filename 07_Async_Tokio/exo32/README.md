# Exercise 32: Task Racing (Select)

[![Français](https://img.shields.io/badge/lang-fr-blue.svg)](./README.fr.md)

## Objective
Handle race scenarios where only the first completed task matters (timeout, competition).

## Concepts Covered
- The `tokio::select!` macro.
- Implicit cancellation of losing tasks.
