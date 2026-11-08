// =====================================================================
// CHAPTER 1: Getting Started - the tools, Hello World, Cargo
// Book: https://doc.rust-lang.org/stable/book/ch01-00-getting-started.html
// Run:  cargo run --bin ch01_hello_world
// =====================================================================
//
// ---------------------------------------------------------------------
// 1.1 THE 3 TOOLS
// ---------------------------------------------------------------------
//   Tool    | Job                                 | Java equivalent
//   --------+-------------------------------------+---------------------
//   rustup  | installs and updates Rust           | downloading a new JDK
//   rustc   | the COMPILER: .rs file -> program   | javac
//   cargo   | build tool + package manager        | Maven / Gradle
//
//   rustc --version   -> e.g. "rustc 1.99.1 (58cf8f9c3 2026-10-13)"
//                        = version (MAJOR.MINOR.PATCH), commit id, release date
//   rustup update     -> get the newest Rust (a new version every 6 weeks)
//   rustup doc        -> open the Rust docs + this book, OFFLINE
//
//   PATH = the list of folders your computer searches when you type a command.
//   Rust lives in ~/.cargo/bin. Programs read PATH only when they START,
//   so after installing Rust, quit VS Code (Cmd+Q) and open it again.
//
// ---------------------------------------------------------------------
// 1.2 COMPILE and RUN are TWO SEPARATE STEPS
// ---------------------------------------------------------------------
//   rustc main.rs   -> COMPILE: makes a program file called `main`
//   ./main          -> RUN it
//
//   Java: javac Main.java -> Main.class (bytecode) -> needs the JVM to run.
//   Rust: rustc main.rs   -> a NATIVE program that your CPU runs directly.
//         A friend with the same kind of computer (same OS + CPU type)
//         can run it WITHOUT installing Rust.
//         This is called "ahead-of-time compilation".
//
// ---------------------------------------------------------------------
// 1.3 CARGO
// ---------------------------------------------------------------------
//   cargo new NAME          -> create a new project (and a git repo)
//   cargo build             -> compile          -> target/debug/NAME
//   cargo run               -> compile + run, in one command
//   cargo check             -> only check for errors (fast, makes no program)
//   cargo build --release   -> optimized: slower to compile, MUCH faster program
//                              -> target/release/NAME (for users / speed tests)
//   cargo fmt               -> fix the formatting of your code automatically
//
//   Project files:
//   Cargo.toml  -> project settings + libraries (Java: pom.xml)
//   Cargo.lock  -> exact versions of the libraries (made by Cargo, never edit)
//   src/        -> your code
//   target/     -> compiled programs. Never commit it to git: it's big,
//                  made for YOUR machine, and `cargo build` can recreate it.
// =====================================================================

// `fn`   = "here comes a function". Java has no keyword for this.
// `main` = the ENTRY POINT: the first code that runs (same as Java).
//          No class around it! Rust has no classes at all.
// `()`   = this function takes no inputs (Java: String[] args).
fn main() {
    // `println!` prints a line (Java: System.out.println).
    // The `!` means it is a MACRO, not a function.
    // Macro = code that writes more code for you at compile time (Chapter 20).
    // Style: indent with 4 spaces, not a tab. `;` ends the line, like Java.
    println!("Hello, world!");
}

// ---------------------------------------------------------------------
// EXPERIMENTS I DID (break it on purpose)
// ---------------------------------------------------------------------
// 1. `println("...")` without the `!`
//    -> error[E0423]: cannot find function `println` in this scope
//    -> help: use `!` to invoke the macro      <- the compiler shows the fix!
//    Tip: `rustc --explain E0423` explains any error code with examples.
//
// 2. No `;` after `println!(...)`
//    -> it still compiles! The last line of a function WITHOUT `;` is the
//       function's RESULT (its return value). `println!` gives back `()`,
//       which means "nothing" (like Java's `void`). `main` also returns `()`,
//       so the types match. Example (Chapter 3 explains this fully):
//           fn five() -> i32 {
//               5 // no `;` and no `return` -> this is the result
//           }
//
// 3. Rename `main` to `start`
//    -> error[E0601]: `main` function not found in crate `learning_rust`
//    Notice: `learning-rust` became `learning_rust`, because names in code
//    can't contain `-`.
