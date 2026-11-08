// =====================================================================
// CHAPTER 3: Common Programming Concepts
// Book: https://doc.rust-lang.org/stable/book/ch03-00-common-programming-concepts.html
// Run:  cargo run --bin ch03_common_concepts
// =====================================================================
//
// Most of this chapter is like Java. These notes focus on what's DIFFERENT.
//
//   3.1 Variables and mutability -> const, shadowing vs mut
//   3.2 Data types               -> sized numbers, NO auto-conversion, tuples, arrays
//   3.3 Functions                -> STATEMENTS vs EXPRESSIONS (the big one!)
//   3.4 Comments                 -> same as Java
//   3.5 Control flow             -> `if` and `loop` give back values, `for` over ranges
//
// ---------------------------------------------------------------------
// 3.1 VARIABLES AND MUTABILITY
// ---------------------------------------------------------------------
//   let x = 5;       // locked (immutable)
//   let mut x = 5;   // unlocked (mutable)
//
// CONSTANTS (Java: static final)
//   const SECONDS_PER_HOUR: u32 = 60 * 60;
//   - always locked: `mut` is not allowed
//   - the TYPE is required
//   - NAME_IN_CAPITALS (same as Java)
//   - the value must be known at COMPILE time (no user input, no runtime values)
//   - can live outside of functions
//
// SHADOWING vs MUT
//   shadowing: `let` again -> a NEW variable. It CAN have a new type:
//       let spaces = "   ";          // text
//       let spaces = spaces.len();   // a number -> OK!
//   mut: the SAME variable. Its type can NEVER change:
//       let mut spaces = "   ";
//       spaces = spaces.len();       // ERROR: mismatched types
//   A shadow only lives until the end of its { } block.
//
// ---------------------------------------------------------------------
// 3.2 DATA TYPES
// ---------------------------------------------------------------------
// Rust is STATICALLY typed, like Java: every value has a type at compile time.
// Rust can often figure out the type itself (type inference).
// When it can't, you write it:  let guess: u32 = "42".parse().expect("...");
//
// INTEGERS: i = can be negative, u = unsigned (0 and up). Number = bits.
//   Rust          | Java  | Range (roughly)
//   --------------+-------+------------------------------------------
//   i8            | byte  | -128 ... 127
//   i16           | short | -32_768 ... 32_767
//   i32           | int   | -2 billion ... 2 billion   <- the DEFAULT
//   i64           | long  | very big
//   i128          | -     | huge
//   u8            | -     | 0 ... 255 (one byte of data)
//   u16 ... u128  | -     | 0 ... bigger and bigger
//   isize / usize | -     | the size of your CPU (64 bits on your Mac).
//                           usize is used for INDEXES and LENGTHS.
//
// NUMBER LITERALS
//   1_000_000    -> underscores only for reading (Java has this too)
//   0xff         -> hex           0o77 -> octal (Java writes 077)
//   0b1111_0000  -> binary        b'A' -> the byte 65 (u8 only)
//
// INTEGER OVERFLOW: a u8 holds max 255. What is 255 + 1?
//   Java: always wraps around silently (a hidden bug!)
//   Rust: Rust can see it at compile time -> compile error
//         debug build   (cargo run)       -> PANIC
//         release build (--release)       -> wraps around to 0
//   Want a specific behavior? Say it with a method:
//     255u8.wrapping_add(1)   -> 0     (wrap around)
//     255u8.saturating_add(1) -> 255   (stop at the max)
//     255u8.checked_add(1)    -> None  (= "no result", Chapter 6)
//
// FLOATS: f64 (the DEFAULT, Java: double) and f32 (Java: float)
//
// MATH: + - * / % like Java. Integer division cuts off:  7 / 2 = 3,  -5 / 3 = -1
//
// !! NO AUTOMATIC CONVERSION BETWEEN NUMBER TYPES !!
//   Java: long big = myInt;          // OK, Java widens automatically
//         double d = 5 + 2.0;        // OK
//   Rust: let big: i64 = my_i32;     // ERROR: mismatched types
//         let d = 5 + 2.0;           // ERROR: can't add an integer and a float
//   Convert yourself with `as`:
//         let big: i64 = my_i32 as i64;
//         let d = 5 as f64 + 2.0;
//
// BOOL: true / false (same as Java)
//
// CHAR: single quotes, like Java. But a Rust char is 4 bytes and can hold
//   ANY Unicode character: 'z', 'ä', '🦀'. (A Java char is 2 bytes, so an
//   emoji doesn't fit in one.) Double quotes "..." = a string, not a char.
//
// TUPLE = a fixed group of values. The types CAN be different.
//   (Java has no tuples. A record is the closest.)
//   let tup: (i32, f64, char) = (500, 6.5, 'x');
//   tup.0 -> 500     tup.1 -> 6.5     tup.2 -> 'x'
//   let (a, b, c) = tup;   // "destructuring": unpack into 3 variables
//   () = the EMPTY tuple, called "unit". It means "nothing" (Java: void).
//
// ARRAY = a fixed number of values of the SAME type.
//   let a = [1, 2, 3, 4, 5];
//   let a: [i32; 5] = [1, 2, 3, 4, 5];   // the type is [type; length]
//   let a = [3; 5];                      // = [3, 3, 3, 3, 3]
//   a[0] -> the first element (same as Java)
//   The length can NEVER change. A growable list is a Vec (Chapter 8,
//   Java: ArrayList).
//   Wrong index, like a[10]:
//     Rust can see it at compile time -> compile error
//     only known at runtime           -> PANIC "index out of bounds"
//     (Java: ArrayIndexOutOfBoundsException. C: reads random memory!)
//   To print a whole array, use {:?} ("debug print"):  println!("{a:?}");
//
// ---------------------------------------------------------------------
// 3.3 FUNCTIONS
// ---------------------------------------------------------------------
//   fn add(x: i32, y: i32) -> i32 {
//       x + y
//   }
//   - names in snake_case: print_measurement (Java: camelCase)
//   - every parameter NEEDS a type
//   - `-> i32` = the return type (Java writes it in front: int add(...))
//   - no `->` = returns () "nothing" (Java: void)
//   - the order doesn't matter: a function can be defined below where it's used
//   - Rust has NO overloading (two functions with the same name)
//     and NO default parameter values. (Java has overloading.)
//
// STATEMENTS vs EXPRESSIONS (the most important idea of this chapter!)
//   STATEMENT  = does something, gives back NO value.
//       let y = 6;              <- a statement
//       fn main() {}            <- a function definition is a statement too
//   EXPRESSION = gives back a VALUE.
//       5 + 6                   -> 11
//       add(2, 3)               -> 5
//       { let x = 3; x + 1 }    -> 4   (a whole { } block is an expression!)
//       if / match              -> a value (see 3.5)
//
//   So this is NOT allowed (Java and C allow x = y = 6):
//       let x = (let y = 6);    // ERROR: `let` gives back no value
//
//   THE SEMICOLON RULE:
//       x + 1      -> an expression: gives back the value
//       x + 1;     -> the `;` turns it into a statement: gives back nothing, ()
//
//   That's how a function returns a value: the LAST expression, WITHOUT `;`
//       fn plus_one(x: i32) -> i32 {
//           x + 1   // no `;` -> this is the return value
//       }
//   With `x + 1;` you get:
//       error[E0308]: mismatched types - expected `i32`, found `()`
//       help: remove this semicolon to return this value
//   (That's why your Chapter 1 experiment "no `;` after println!" worked!)
//
//   `return x;` also exists, to leave a function EARLY (like Java).
//   Rust style: use `return` only for early exits. At the end, just write the value.
//
// ---------------------------------------------------------------------
// 3.4 COMMENTS
// ---------------------------------------------------------------------
//   // a line comment (same as Java). Rust style: use // even for many lines.
//   /* ... */ also works, but it's rarely used.
//   /// a DOC comment: becomes documentation (Chapter 14, Java: /** ... */)
//
// ---------------------------------------------------------------------
// 3.5 CONTROL FLOW
// ---------------------------------------------------------------------
// IF: no ( ) around the condition, { } always required
//   if n < 5 { ... } else if n < 10 { ... } else { ... }
//   The condition MUST be a bool (same as Java):  `if n { }` -> ERROR
//
// `if` IS AN EXPRESSION -> it gives back a value. Rust has NO ternary `? :`
//   Java: int number = condition ? 5 : 6;
//   Rust: let number = if condition { 5 } else { 6 };
//   Both branches must give the SAME type:  if c { 5 } else { "six" } -> ERROR
//
// LOOP: repeat forever until `break` (Java: while (true))
//   `break` can give back a VALUE (Java can't do this):
//       let result = loop {
//           counter += 1;
//           if counter == 10 {
//               break counter * 2;   // the loop gives back 20
//           }
//       };
//
// LOOP LABELS: break out of an OUTER loop (Java has labels too)
//   'outer: loop {
//       loop {
//           break 'outer;   // leaves BOTH loops
//       }
//   }
//   A label starts with a single quote: 'name
//
// WHILE: same as Java, without ( )
//   while n != 0 { n -= 1; }
//
// FOR: walk through a collection (Java: for (int x : a))
//   for element in a { ... }
//   There is NO C-style  for (int i = 0; i < n; i++)  in Rust. Use a RANGE:
//   for i in 0..5 { }           // 0, 1, 2, 3, 4
//   for i in (1..4).rev() { }   // 3, 2, 1   (.rev() = reverse)
//   Tip: `for x in a` is safer than a[i] with an index (no wrong index possible).
//   Clippy even warns when you loop with an index you don't need.
// =====================================================================

// A constant can live outside of functions. Type required, NAME_IN_CAPITALS.
const SECONDS_PER_HOUR: u32 = 60 * 60;

fn main() {
    variables();
    data_types();
    functions();
    control_flow();

    // ---------------- YOUR TASK ----------------
    // Call your task functions here (see the bottom of this file).
}

// ---------------------------------------------------------------------
// 3.1 VARIABLES
// ---------------------------------------------------------------------
fn variables() {
    println!("--- 3.1 Variables ---");

    let mut x = 5; // `mut`: we will change it
    println!("x = {x}");
    x = 6;
    println!("x = {x}");

    println!("One hour = {SECONDS_PER_HOUR} seconds");

    // Shadowing: a NEW variable with the same name
    let y = 5;
    let y = y + 1; // y is now 6
    {
        let y = y * 2; // only inside these { }: y = 12
        println!("inner y = {y}");
    }
    println!("outer y = {y}"); // back to 6: the inner shadow is gone

    // Shadowing can change the TYPE (mut can't)
    let spaces = "   "; // text
    let spaces = spaces.len(); // now a number
    println!("spaces = {spaces}");
}

// ---------------------------------------------------------------------
// 3.2 DATA TYPES
// ---------------------------------------------------------------------
fn data_types() {
    println!("--- 3.2 Data types ---");

    // Integers
    let a = 98_222; // i32 (the default)
    let b: u8 = 255; // u8: 0 ... 255
    let c: i64 = 9_000_000_000; // too big for i32 -> i64
    println!("a = {a}, b = {b}, c = {c}");

    let hex = 0xff;
    let binary = 0b1111_0000;
    let byte = b'A';
    println!("0xff = {hex}, 0b1111_0000 = {binary}, b'A' = {byte}");

    // Overflow: say what you want to happen
    let max: u8 = 255;
    println!("255 wrapping_add 1   = {}", max.wrapping_add(1)); // 0
    println!("255 saturating_add 1 = {}", max.saturating_add(1)); // 255
    // `max + 1` here -> compile error (Rust can see both values).
    // With a value from user input -> PANIC in a debug build.

    // Floats and math
    let f = 2.5; // f64 (the default)
    let g: f32 = 3.0; // f32
    let quotient = 7 / 2; // 3 (cut off, not rounded)
    let negative = -5 / 3; // -1
    let remainder = 43 % 5; // 3
    println!("f = {f}, g = {g}, 7 / 2 = {quotient}, -5 / 3 = {negative}, 43 % 5 = {remainder}");

    // No automatic conversion: use `as`
    let small: i32 = 5;
    let big: i64 = small as i64; // `let big: i64 = small;` would be an ERROR
    let mixed = small as f64 + 0.5; // `small + 0.5` would be an ERROR
    println!("big = {big}, mixed = {mixed}");

    // bool and char
    let is_fun: bool = true;
    let letter = 'ä';
    let crab = '🦀'; // a char can hold ANY Unicode character
    println!("is_fun = {is_fun}, letter = {letter}, crab = {crab}");

    // Tuple: different types, fixed size
    let tup: (i32, f64, char) = (500, 6.5, 'x');
    println!("tup.0 = {}, tup.1 = {}, tup.2 = {}", tup.0, tup.1, tup.2);
    let (t0, t1, t2) = tup; // destructuring: unpack into 3 variables
    println!("unpacked: {t0}, {t1}, {t2}");

    // Array: same type, fixed size
    let months = ["January", "February", "March"];
    let threes = [3; 5]; // [3, 3, 3, 3, 3]
    println!("first month = {}, length = {}", months[0], months.len());
    println!("threes = {threes:?}"); // {:?} = debug print, needed for arrays
    // months[10] -> compile error here (Rust can see it).
    // With an index from user input -> PANIC "index out of bounds".
}

// ---------------------------------------------------------------------
// 3.3 FUNCTIONS
// ---------------------------------------------------------------------
fn functions() {
    println!("--- 3.3 Functions ---");

    print_measurement(5, 'h');

    let five = plus_one(4);
    println!("plus_one(4) = {five}");

    // A { } block is an expression: its last line (without `;`) is its value
    let y = {
        let x = 3;
        x + 1
    };
    println!("block value = {y}");
}

// Every parameter needs a type. No `->` = returns () "nothing".
fn print_measurement(value: i32, unit: char) {
    println!("The measurement is: {value}{unit}");
}

// `-> i32` = returns an i32. The last expression (without `;`) is the result.
fn plus_one(x: i32) -> i32 {
    x + 1 // adding `;` here -> ERROR: expected `i32`, found `()`
}

// ---------------------------------------------------------------------
// 3.5 CONTROL FLOW
// ---------------------------------------------------------------------
fn control_flow() {
    println!("--- 3.5 Control flow ---");

    // if / else if / else: no ( ), { } required
    let number = 6;
    if number % 4 == 0 {
        println!("{number} is divisible by 4");
    } else if number % 3 == 0 {
        println!("{number} is divisible by 3");
    } else {
        println!("{number} is not divisible by 4 or 3");
    }

    // `if` gives back a value (Rust has no `? :` ternary)
    let temperature = 22;
    let feeling = if temperature > 25 { "hot" } else { "nice" };
    println!("{temperature} degrees feels {feeling}");

    // `loop` can give back a value with `break value`
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("loop result = {result}");

    // Loop label: `break 'outer` leaves the OUTER loop
    let mut count = 0;
    'outer: loop {
        let mut remaining = 10;
        loop {
            if remaining == 9 {
                break; // leaves only the inner loop
            }
            if count == 2 {
                break 'outer; // leaves BOTH loops
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("end count = {count}");

    // while: same as Java, without ( )
    let mut n = 3;
    while n != 0 {
        println!("{n}!");
        n -= 1;
    }
    println!("LIFTOFF!");

    // for over an array (Java: for (int x : a))
    let a = [10, 20, 30];
    for element in a {
        println!("the value is: {element}");
    }

    // for over a range, reversed: 3, 2, 1
    for i in (1..4).rev() {
        println!("{i}...");
    }
}

// =====================================================================
// YOUR TASK (write it yourself - no copy-paste!)
// =====================================================================
// 1. A function that converts Fahrenheit to Celsius:
//        fn fahrenheit_to_celsius(f: f64) -> f64
//    Formula: C = (F - 32) * 5 / 9
//    In main, print:   100°F = 37.8°C   and   32°F = 0.0°C
//    Hint: {c:.1} prints a number with 1 digit after the dot.
//    Hint: remember, NO automatic conversion between integers and floats!
//
// 2. A function that gives the n-th Fibonacci number:
//        fn fibonacci(n: u32) -> u64
//    Fibonacci: 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, ...
//    (each number = the two numbers before it, added together)
//    fibonacci(0) = 0, fibonacci(1) = 1, fibonacci(10) = 55
//    In main, print the first 11 numbers with a `for` loop over a range.
//
// 3. Bonus: use `if` as an expression, for example print
//    "cold" / "nice" / "hot" next to each temperature.
//
// Rules: zero warnings in `cargo clippy`. No `return` at the end of a function.
// =====================================================================
