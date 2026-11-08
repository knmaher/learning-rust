// =====================================================================
// CHAPTER 2: Programming a Guessing Game
// Book: https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html
// Run:  cargo run --bin ch02_guessing_game
// =====================================================================
//
// THE GAME: the program picks a secret number from 1 to 100.
// The player guesses. The program answers "Too small!", "Too big!" or "You win!".
//
// This chapter is a TASTE of many things. Each one gets its own chapter later.
//
//   Part 1: read the player's guess      [done]
//   Part 2: make a secret random number  [done]
//   Part 3: compare guess and secret     [to do]
//   Part 4: keep asking until correct    [to do]
//
// ---------------------------------------------------------------------
// PART 1 NOTES: variables and input
// ---------------------------------------------------------------------
// VARIABLES ARE LOCKED BY DEFAULT (the opposite of Java!)
//   let apples = 5;       // locked: can never change
//   let mut bananas = 5;  // `mut` = unlocked: can change
//   Java: changeable by default, add `final` to lock.
//   Rust: locked by default, add `mut` to unlock.   (Chapter 3)
//
// `::` means "LOOK INSIDE"
//   std::io      = the `io` module inside `std` (Rust's standard library)
//   String::new  = the `new` function inside the type `String`
//                  (Java: a static method, like String.valueOf(5))
//   `.` is for methods called on a VALUE, like guess.len()
//   Java uses `.` for both. Rust uses `::` for the type, `.` for the value.
//
// A Rust `String` is GROWABLE text (more like Java's StringBuilder).
//
// `&mut guess` = LEND `guess` to a function and allow it to CHANGE it.
//   `&` = a reference: lend it, don't give it away.   (Chapter 4)
//
// RESULT = a box with ONE of two things:
//   Ok(value) -> it worked
//   Err(why)  -> it failed
//   .expect("msg"): if Ok -> continue. If Err -> crash (panic) with "msg".
//   Java forces try/catch for IOException. Rust has NO exceptions:
//   errors are VALUES that you must handle.   (Chapter 9)
//
// WARNING vs ERROR
//   error   (red)    -> the program is NOT built. You MUST fix it.
//   warning (yellow) -> the program IS built, but you probably made a mistake.
//   My rule: keep ZERO warnings.
//   To ignore a Result on purpose, say so clearly:  let _ = ...;
//
// read_line KEEPS the Enter key as an invisible character: `\n` (newline).
//   Type 40 + Enter -> guess = "40\n" (3 bytes).
//   Java's readLine() removes it. Rust's read_line() does NOT!
//   .len() counts BYTES:  "7\n".len() = 2,  "ä".len() = 2,  "🦀".len() = 4
//
// ---------------------------------------------------------------------
// PART 2 NOTES: crates and random numbers
// ---------------------------------------------------------------------
// CRATE = a package of Rust code (Java: a .jar)
//   binary crate  = has `main`, you can run it (like this program)
//   library crate = no `main`, other programs use it (like `rand`)
//   crates.io     = the website where crates come from (Java: Maven Central)
//
// VERSIONS (see the comments in Cargo.toml)
//   rand = "0.10.1" -> "at least 0.10.1, but stay inside 0.10.x"
//   Cargo.toml = what I WANT (a range).  Cargo.lock = what I actually GOT (exact).
//   Cargo.lock makes sure everyone (me, a friend, a build server)
//   gets the SAME versions -> the same program everywhere.
//   `cargo update` -> get newer versions (still inside the allowed range).
//
// TRAIT = a list of methods a type promises to have (Java: interface)
//   RULE: you can only use a trait's methods if the trait is imported with `use`.
//   If Rust says "no method found" but the method exists -> you forgot a `use`.
//   (Chapter 10)
//
// RANGES
//   1..=100 -> 1 to 100, WITH 100   (the `=` means "and the end too")
//   1..100  -> 1 to 99, WITHOUT 100
//   1..=1   -> only 1
//   1..1    -> EMPTY (start at 1, stop BEFORE 1) -> random_range PANICS
//
// PANIC = Rust's word for crashing while the program runs
//         (Java: an uncaught exception).
//   The message shows file:line:column of the crash.
//   RUST_BACKTRACE=1 shows the list of function calls (Java: stack trace).
//   The compiler can't catch everything: `1..1` is a valid range TYPE,
//   it only turns out to be empty when the program RUNS.
// =====================================================================

use std::io; // bring in the input/output tools (Java: import)

use rand::prelude::*; // `*` = everything (Java: import java.util.*;)
// rand's prelude contains the trait that gives us `random_range`.

fn main() {
    println!("Guess the number!");

    // ---------------- PART 2: the secret number ----------------
    // rand::rng()             -> "give me a dice" (a random number generator)
    // .random_range(1..=100)  -> roll it: a number from 1 to 100
    // No `mut`: the secret number must never change.
    let secret_number = rand::rng().random_range(1..=100);

    // Only for testing! Delete this line at the end of the chapter.
    println!("The secret number is: {secret_number}");

    // ---------------- PART 1: read the player's guess ----------------
    println!("Please input your guess.");

    // An empty, growable String. `mut` because read_line will change it.
    let mut guess = String::new();

    io::stdin() // the keyboard input (Java: System.in)
        .read_line(&mut guess) // read a line and ADD it to `guess` (keeps the \n!)
        .expect("Failed to read line"); // Ok -> continue. Err -> crash with this message.

    // {guess} puts the value inside the text (Java: "You guessed: " + guess)
    println!("You guessed: {guess}");

    // ---------------- PART 3: compare guess and secret ----------------
    // (to do - you will type it yourself)

    // ---------------- PART 4: keep asking until correct ----------------
    // (to do - you will type it yourself)
}
