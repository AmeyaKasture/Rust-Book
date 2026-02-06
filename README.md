# 🦀 Rust Book – Learning, Experiments, and Deep Dives

This repository documents my journey of learning Rust by *doing*.

Instead of passively reading the Rust Book, this repo is structured as a hands-on playground where I explore the **intricacies of Rust’s core concepts**, experiment with edge cases, and build intuition through small, focused examples.

---

##  What this repository is about

The goal of this repo is to:
- Understand **why Rust works the way it does**, not just *how*
- Deep dive into Rust’s ownership model and borrow checker
- Explore real-world and contrived use cases to stress-test concepts
- Build strong mental models around safety, performance, and correctness

Each module/crate focuses on a specific idea and expands on it through experimentation.

---

## Core concepts explored

Some of the major Rust topics covered here include:

- **Ownership principles**
  - Moves, copies, and drops
  - Stack vs heap semantics

- **Borrowing and references**
  - Mutable vs immutable borrows
  - Borrow checker rules and violations
  - Lifetimes and why they exist

- **Mutability and safety**
  - Interior vs exterior mutability
  - Why Rust makes mutation explicit

- **Compiler-driven learning**
  - Understanding error messages
  - Learning by intentionally breaking the rules

- **Idiomatic Rust patterns**
  - Writing code the “Rust way”
  - Avoiding anti-patterns common to beginners

---

##  Structure

The repository is organized into multiple small Rust crates, each aligned with a chapter or concept from the Rust Book:

```
projects/
├── hello_world/
├── hello_cargo/
├── guessing_game/
├── ownership/
└── ...
```

Each crate:
- Is self-contained
- Focuses on one core idea
- Includes experiments and variations to explore edge cases

---

## Tooling & workflow

- **Cargo** for project management
- **rustfmt** for consistent formatting
- **Git** for tracking learning progress

Build artifacts (`target/`) are intentionally ignored to keep the repo clean and focused on learning.

---

## Why this repo exists

Rust has a steep learning curve, especially around ownership and borrowing. This repository exists to:

- Slow down and deeply understand Rust’s safety guarantees
- Treat compiler errors as learning signals, not obstacles
- Build confidence writing correct, idiomatic Rust

This is a living repository and will continue to evolve as my understanding of Rust grows.

---

## 🚧 Status

Ongoing. New experiments, refactors, and deeper dives are added continuously as I progress through Rust concepts.

---

🦀 *Learning Rust the hard (but rewarding) way — one borrow checker error at a time.*

