# XOR-Cipher

> My first practice project to learn Rust.

I built this command-line tool to get my hands dirty with Rust syntax. It encrypts and decrypts files using the simple XOR operator.
My main focus was understanding **Ownership**, **Borrowing**, and handling **File I/O** in a memory-safe way.

## ⚠️ Disclaimer
This is purely for **educational purposes**. The XOR cipher is **not secure** for sensitive data.
I wrote this to learn, so the code might not be "production-ready" yet.

*Feedback is highly appreciated! If you see any non-idiomatic Rust code, please let me know so I can improve.*

## Features
- **Memory Efficient:** I used buffered reading/writing so it doesn't load the whole file into RAM (good for large files).
- **User Friendly:** Added a progress bar to show status during operation.

## Prerequisites
You need to have `Rust` and `Cargo` installed.

```bash
# Clone the repository
git clone https://github.com/MSerox/xor-cipher.git

# Navigate to the folder
cd xor-cipher

# Build release version
cargo build --release
```

## How to Run
It's an interactive tool. Just run it and follow the instructions:

```bash
cargo run --release
```

## What I Learned
Building this helped me understand:

- Error Handling: How to move away from unwrap() and use Result, match, and custom enums properly.

- Buffers: Reading files in chunks to keep memory usage low.

- CLI Experience: How to interact with the user and render a progress bar in the terminal.


Distributed under the [MIT License](LICENSE).
