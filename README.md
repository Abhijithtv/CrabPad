# 🦀 CrabPad

> A high-performance terminal-based text editor written in Rust, powered by a Rope (Binary Tree) data structure for efficient editing of large files.

CrabPad is an experimental text editor built from scratch to explore how modern text editors manage text efficiently. Instead of storing the entire file as a single string, CrabPad uses a **Rope data structure** implemented as a **Binary Tree**, allowing insertions and deletions in **O(log N)** time.

This project focuses on performance, data structures, and understanding how editors work internally.

---

## ✨ Features

- ⚡ Fast text insertion and deletion using a Rope data structure
- 🌲 Binary Tree based text storage
- 📈 O(log N) editing operations on large files
- 🖥️ Terminal-based text editor
- 🦀 Written entirely in Rust
- 🧩 Modular architecture for future editor features

---

## Why Rope?

Most simple text editors store the entire document as one large string.

```
Hello World...
```

For large files, inserting or deleting text requires shifting large portions of memory.

A Rope stores text as a balanced binary tree.

```
              Root
            /      \
      "Hello "    "World"
```

When editing:

- Only a small portion of the tree changes
- Most nodes remain untouched
- Editing becomes **O(log N)** instead of **O(N)**

This makes Rope ideal for large documents.

---
## Getting Started

### Clone

```bash
git clone https://github.com/Abhijithtv/CrabPad.git
cd CrabPad
```

### Build

```bash
cargo build
```

### Run

```bash
cargo run
```

---

## Data Structure

The editor stores text using a Rope.

### Leaf Node

Contains actual characters.

```text
Leaf
-----
"Hello "
```

### Internal Node

Stores:

- Left child
- Right child
- Character count of the left subtree

Example:

```
             (11)
            /    \
       (6)        (5)
    "Hello "    "World"
```

The stored character counts allow the editor to quickly locate the correct position during insertions, deletions, and navigation.
---

## Performance

| Operation | Complexity |
|----------|------------|
| Insert | **O(log N)** |
| Delete | **O(log N)** |
| Split | **O(log N)** |
| Merge | **O(log N)** |
| Lookup | **O(log N)** |

---

## Learning Objectives

CrabPad is also a learning project to better understand:

- Text editor internals
- Rope data structures
- Balanced binary trees
- Memory-efficient editing
- Rust ownership and borrowing
- Terminal rendering
- Systems programming

---

## Tech Stack

- Rust
- Cargo
- Terminal Rendering
- Rope Data Structure
- Binary Tree

---
## Contributing

Contributions, suggestions, and discussions are welcome.

Feel free to open an issue or submit a pull request.

---

## License

This project is licensed under the MIT License.

---

## Author

**Abhijith T V**

GitHub: https://github.com/Abhijithtv
