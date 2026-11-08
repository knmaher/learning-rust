# Learning Rust 🦀

My code and notes while reading [The Rust Book](https://doc.rust-lang.org/stable/book/).
Every chapter is its own small program in `src/bin/`, with simple notes as comments at the top of the file.

## Run a chapter

```bash
cargo run --bin ch02_guessing_game
```

Change the name to the chapter you want. Run `cargo run` alone to see the list of all chapters.

## Chapters

| Chapter | File | Topic | Status |
|---|---|---|---|
| 0 | [`ch00_introduction.rs`](src/bin/ch00_introduction.rs) | Why Rust exists, compile time vs runtime | ✅ Done |
| 1 | [`ch01_hello_world.rs`](src/bin/ch01_hello_world.rs) | The tools, Hello World, Cargo | ✅ Done |
| 2 | [`ch02_guessing_game.rs`](src/bin/ch02_guessing_game.rs) | Guessing game | 🚧 Parts 1–2 done, Parts 3–4 to do |

## How this project is built

```
learning-rust/
├── Cargo.toml          ← settings + libraries, shared by all chapters
├── Cargo.lock          ← exact library versions (made by Cargo)
├── README.md           ← this file
└── src/
    └── bin/            ← one program per chapter
        ├── ch00_introduction.rs
        ├── ch01_hello_world.rs
        └── ch02_guessing_game.rs
```

- Every `.rs` file in `src/bin/` is **its own program** with its own `fn main()`. Cargo finds them automatically.
- All chapters share **one** `Cargo.toml`, so they can all use the same crates (like `rand`).
- Java comparison: one Maven project with several classes that each have a `main` method. You choose which one to run.

## Useful commands

| Command | What it does |
|---|---|
| `cargo run --bin NAME` | Build and run one chapter |
| `cargo check` | Check for errors quickly (makes no program) |
| `cargo build` | Compile all chapters |
| `cargo build --release` | Optimized build (slower to compile, faster program) |
| `cargo fmt` | Fix the formatting of all files |
| `cargo doc --open` | Open the docs of all crates I use |
| `rustc --explain E0423` | Explain an error code |

## Class rules

1. Type the code yourself. No copy-paste.
2. Write a prediction before you run an experiment.
3. Read every error message to the bottom and look for `help:`.
4. Keep zero warnings.
