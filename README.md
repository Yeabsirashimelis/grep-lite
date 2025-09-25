# grep-lite

**grep-lite** is a lightweight command-line tool written in Rust for searching text patterns in files or standard input. It is inspired by the classic Unix `grep` utility and supports regular expressions for flexible searching.

---

## Features

- Search for text patterns using **regular expressions**.
- Read from **one file** or **standard input (stdin)**.
- Efficient line-by-line processing using Rust's `BufReader`.
- Cross-platform and fast.
- Simple and easy-to-use CLI with automatic `--help` and `--version`.

---

## Installation

You need **Rust** installed on your system. Then clone the repository and build it:

```bash
git clone https://github.com/Yeabsirashimelis/grep-lite.git
cd grep-lite
cargo build --release
