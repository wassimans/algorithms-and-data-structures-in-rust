# Algorithms and Data Structures in Rust (WIP)

<div align="center">
  <img src="assets/image.png" alt="Algorithms and Data Structures in Rust" width="600">
</div>

[![License: Unlicense](https://img.shields.io/badge/license-Unlicense-blue.svg)](http://unlicense.org/)

## About
This is a personal attempt at implementing some known algorithms and data structures using the Rust programming language. This is mostly for learning purposes.

The idea is to do some research on some data structure or algorithm, write some notes about it in this same README then start implementing it with some tests.

The structure of the Cargo project is very simple: a main Cargo workspace inside which there's cargo sub-projects for each data structure or algorithm. This gives us the ability to add tests specific to every sub-project for more modularity.

Since it's known that implementing data structures and algorithms in a verbose language like *Rust* is a harder than using more expressive languages like *Python*, I think it's still a doable thing if we use *generics*, this will keep the compiler happy while giving us some freedom to skip some verbosity in our final code.

## Patterns
In **Rust**, generally, when we want to implement a custom data structure, we start with a **struct**, and then define custom functions or methods for that struct.

In my implementations, I will try to use basic scalar, composite and collection data types provided by Rust in order to achieve maximum efficiency and compatibility. For example: we will use a *Vec* to implement our *Stack*. 

## Stack
- A *stack* is a *linear* data structure that contains an ordered collection of items.
- New items are added or removed from the top in LIFO manner.

### Exposed API

```mermaid
classDiagram
    class Stack~T~ {
        - Vec~T~ data
        + new() Stack~T~
        + push(item: T) void
        + pop() T
        + peek() T
        + is_empty() bool
        + size() usize
        + iter() Iterator~&T~
        + iter_mut() Iterator~&mut T~
        + into_iter() IntoIterator~T~
    }

    Stack~T~ --> Vec~T~ : uses tail as top
```
