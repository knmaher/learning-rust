// =====================================================================
// CHAPTER 0: Introduction - Why does Rust exist?
// Book: https://doc.rust-lang.org/stable/book/ch00-00-introduction.html
// Run:  cargo run --bin ch00_introduction
// =====================================================================
//
// ---------------------------------------------------------------------
// 1. THE BIG PROBLEM: two "camps" of languages
// ---------------------------------------------------------------------
//
//   Camp A: CONTROL & SPEED   (C, C++)
//     + very fast, you control the memory yourself
//     - gives up SAFETY: one memory mistake = crash or security hole
//
//   Camp B: SAFETY & COMFORT  (Java, Python, JavaScript, Go)
//     + hard to crash, easy to write
//     - gives up SPEED and CONTROL: a garbage collector (a "cleaner")
//       uses CPU time, and you can't decide when it runs
//
//   Rust = the speed of Camp A + the safety of Camp B.
//
// ---------------------------------------------------------------------
// 2. HOW? The hotel analogy
// ---------------------------------------------------------------------
//   Memory = a hotel. Your data = guests in rooms.
//
//   Language | Who frees the memory?           | Who checks your memory rules?
//   ---------+---------------------------------+------------------------------
//   C / C++  | YOU, by hand                    | NOBODY -> leaks, "use after free"
//   Java     | the CLEANER (garbage collector) | not needed, the cleaner does it
//   Rust     | freed automatically, at the     | the INSPECTOR (the compiler),
//            | right moment                    | BEFORE the program runs
//
//   The inspector is Rust's new idea.
//   Its rules are called OWNERSHIP and BORROWING (Chapter 4).
//
// ---------------------------------------------------------------------
// 3. COMPILE TIME vs RUNTIME  (the most important idea!)
// ---------------------------------------------------------------------
//   compile time = the compiler reads your code and builds the program.
//                  Nothing is running yet.
//   runtime      = the program is running (on your or a user's computer).
//
//   Bug found at compile time -> costs you a few seconds.
//   Bug found at runtime      -> can crash in front of a user.
//
//   Java: errors from `javac`      = compile time
//         NullPointerException     = runtime
//
// ---------------------------------------------------------------------
// 4. HIGH-LEVEL vs LOW-LEVEL
// ---------------------------------------------------------------------
//   low-level  = close to the MACHINE: you handle memory and bytes yourself.
//                Like a manual car: more control, more ways to make mistakes.
//   high-level = close to HUMANS: the language hides the machine details.
//                Like an automatic car: easier, but it decides some things for you.
//
//   From machine to human:
//     machine code -> assembly -> C -> C++ / Rust -> Java / Go -> Python / JS
//   Rust can go low AND high.
//
// ---------------------------------------------------------------------
// 5. ZERO-COST ABSTRACTIONS
// ---------------------------------------------------------------------
//   You can write nice high-level code, and it runs as fast as low-level code.
//   "Zero cost" = zero cost at RUNTIME. It does NOT mean "easy to learn"!
//
// ---------------------------------------------------------------------
// 6. MINDSET FOR THE WHOLE COURSE
// ---------------------------------------------------------------------
//   The compiler will say "no" to you A LOT. That is not failure:
//   every "no" is a bug that will never reach runtime.
//   Always read the error message to the bottom - look for the "help:" line.
// =====================================================================

fn main() {
    // High-level code that runs at low-level speed (a zero-cost abstraction).
    // You'll learn arrays `[...]` (Chapter 3) and `.iter()` (Chapter 13) later.
    let numbers = [1, 2, 3];
    let sum: i32 = numbers.iter().sum();
    println!("1 + 2 + 3 = {sum}");
    // No malloc, no free, no garbage collector:
    // Rust cleans up `numbers` by itself at the end of `main`.

    // TRY IT: the compiler catches this mistake BEFORE the program runs.
    // Remove the `//` in front of the next line, then run `cargo check`.
    // let age: u32 = "thirty";
    //
    // You get: error[E0308]: mismatched types - expected `u32`, found `&str`
    // In Python, the same mistake only crashes when that line RUNS.
}
