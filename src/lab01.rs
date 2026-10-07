//! Lab 1 — First Steps in Rust — exercises.
//!
//! Fill in each `todo!()` with a working implementation. Everything here
//! compiles as-is (a stub); running it will panic on the first `todo!()`
//! reached until you implement it. Full task descriptions: lab01-rust-basics.md.

#![allow(dead_code, unused_variables, unused_mut)]

// ---------------------------------------------------------------------------
// Task 1 — Formatting output
// ---------------------------------------------------------------------------
// Using ONE println! per line, print exactly:
//   Name: Ada, age: 36
//   Pi to 3 places: 3.142
//   |   right|left    | center |
//   Debug: (1, "two", 3.0)
//   0x ff, 0b 1010
// Hints: {}, {:?}, {:.3}, {:>8}, {:<8}, {:^8}, {:x}, {:b}. You can write
// variable names inline: println!("{name}").

fn task1_formatting() {
    let name = "Ada";
    let age = 31;
    println!("Name {name}, age {age}");
    println!("Pi to 3 places: {:.3}", std::f64::consts::PI);
    println!("|{:>8}|{:<8}|{:^8}|", "right", "left", "center");
    println!("{:?}", (1, "two", 3.0));
}

// ---------------------------------------------------------------------------
// Task 2 — Variables, mutability and shadowing
// ---------------------------------------------------------------------------
fn task2_variables() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("inner: {x}");
    }
    println!("outer: {x}");

    let mut count = 0;
    count += 1;
    println!("{count}");
}

// ---------------------------------------------------------------------------
// Task 3 — Scalar types and overflow
// ---------------------------------------------------------------------------
// factorial: return None if the result would not fit in a u64. Use
// checked_mul in a loop from 2..=n. Option is covered properly in Lab 3 —
// for now, Some(v) means "there is a value", None means "there is none".
//   factorial(0) == Some(1), factorial(5) == Some(120),
//   factorial(20) == Some(2432902008176640000), factorial(21) == None.
fn factorial(n: i32) -> Option<u64> {
    if n < 0 {
        return None;
    }
    let mut acc: u64 = 1;
    for i in 2..=n as u64 {
        acc = acc.checked_mul(i)?;
    }
    Some(acc)
}

// task3_scalars:
// 1. Print i8::MIN, i8::MAX, u8::MAX, i32::MAX, u64::MAX, f64::EPSILON.
// 2. As a comment: what happens with `let x: u8 = 255; let y = x + 1;` in a
//    debug build vs. a release build (`cargo run --release`)? Why?
// 3. For `250u8 + 10`, print the result of checked_add, wrapping_add,
//    saturating_add and overflowing_add.
// 4. Print factorial(20) and factorial(21) using {:?} (they're Options).
// 5. Casting with `as`: print what `300_i32 as u8`, `-1_i32 as u32`,
//    `3.99_f64 as i32` and `-3.99_f64 as u32` give, plus `u8::try_from(300)`.
//    As a comment: why is `as` considered dangerous, and what's the safer
//    alternative?
fn task3_scalars() {
    println!(
        "{} {} {} {} {} {}",
        i8::MIN,
        i8::MAX,
        u8::MAX,
        i32::MAX,
        u64::MAX,
        f64::EPSILON
    );

    // `let x: u8 = 255; let y = x + 1;` panics with "attempt to add with
    // overflow" in a debug build (overflow checks are on by default), but
    // silently wraps to 0 in a release build (checks are off by default).

    let x: u8 = 250;
    println!("{:?}", x.checked_add(10));
    println!("{}", x.wrapping_add(10));
    println!("{}", x.saturating_add(10));
    println!("{:?}", x.overflowing_add(10));

    println!("{:?}", factorial(20));
    println!("{:?}", factorial(21));

    println!("{}", 300_i32 as u8);
    println!("{}", -1_i32 as u32);
    println!("{}", 3.99_f64 as i32);
    println!("{}", -3.99_f64 as u32);
    println!("{:?}", u8::try_from(300));

    // `as` is dangerous because it silently truncates or saturates instead
    // of reporting failure — 300_i32 as u8 just drops the high bits. The
    // safer alternative is TryFrom/try_into, which returns a Result.
}

// ---------------------------------------------------------------------------
// Task 4 — Functions and expressions
// ---------------------------------------------------------------------------
// Almost everything in Rust is an expression. A block's value is its last
// expression, written WITHOUT a trailing semicolon.

// Write the body without using the `return` keyword.
fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

// Write the body without using the `return` keyword.
fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

// Return "negative", "zero" or "positive" depending on x. Use `if` as an
// expression (no `return`).
fn sign(x: i32) -> String {
    if x > 0 {
        "positive".to_string()
    } else if x == 0 {
        "zero".to_string()
    } else {
        "negative".to_string()
    }
}

// This one-liner currently would fail to compile if written as
// `x * x;` (a statement, value `()`) instead of `x * x` (an expression).
// Write it as an expression so it actually returns i32.
fn square(x: i32) -> i32 {
    x * x
}

// task4_expressions:
// Call celsius_to_fahrenheit(100.0), fahrenheit_to_celsius(212.0),
// sign(-3) and square(7), printing the results.
// Then: use a block expression to compute a value, e.g.
//   let y = { let a = 3; a * a + 1 };
// and print y. As a comment: what would y be if the block's last line ended
// with a semicolon instead?
fn task4_expressions() {
    println!("{}", celsius_to_fahrenheit(100.0));
    println!("{}", fahrenheit_to_celsius(212.0));
    println!("{}", sign(-3));
    println!("{}", square(7));

    let y = {
        let a = 3;
        a * a + 1
    };
    println!("{y}");

    // With a trailing semicolon on `a * a + 1;`, the block's value becomes
    // `()` instead of `i32`, so `let y = { ... };` would fail to compile —
    // `y`'s type couldn't be inferred as i32.
}

// ---------------------------------------------------------------------------
// Task 5 — Control flow
// ---------------------------------------------------------------------------
// Each function below practises a different construct. Implement all of
// them, then have task5_control_flow print FizzBuzz for 1..=20.

// Use `match` on the tuple (n % 3, n % 5).
// 3 -> "Fizz", 10 -> "Buzz", 15 -> "FizzBuzz", 7 -> "7"
fn fizzbuzz(n: u32) -> String {
    match (n % 3, n % 5) {
        (0, 0) => "FizzBuzz".to_string(),
        (0, _) => "Fizz".to_string(),
        (_, 0) => "Buzz".to_string(),
        _ => n.to_string(),
    }
}

// Use `while`. 1 -> 0, 6 -> 8, 27 -> 111.
fn collatz_steps(mut n: u64) -> u32 {
    let mut steps = 0;
    while n != 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        steps += 1;
    }
    steps
}

// Use `loop` or `while` — Euclid's algorithm. gcd(48, 18) == 6.
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

// Use `for` over a range with an early `return`.
// is_prime(2) == true, is_prime(97) == true, is_prime(1) == false.
fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=(n as f64).sqrt() as u64 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

// Use `for` with tuple assignment: (a, b) = (b, a + b).
// fib(0) == 0, fib(10) == 55, fib(50) == 12586269025.
fn fib(n: u32) -> u64 {
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 0..n {
        (a, b) = (b, a + b);
    }
    a
}

// Use `match` with inclusive ranges, e.g. 90..=100.
// 95 -> 'A', 72 -> 'C', 30 -> 'F'. Cover every possible u32 value: what
// happens if you remove one arm?
fn grade(points: u32) -> char {
    match points {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        _ => 'F',
    }
}

// Print fizzbuzz(1..=20) with a `for` loop, then call the functions above
// with the example values from their comments and print the results.
// Also try: for i in (1..=5).rev() {...} and for i in (0..20).step_by(5) {...}.
// As a comment: what's the difference between 1..10 and 1..=10?
fn task5_control_flow() {
    for i in 1..=20u32 {
        println!("{}", fizzbuzz(i));
    }

    println!(
        "{} {} {}",
        collatz_steps(1),
        collatz_steps(6),
        collatz_steps(27)
    );
    println!("{}", gcd(48, 18));
    println!("{} {} {}", is_prime(2), is_prime(97), is_prime(1));
    println!("{} {} {}", fib(0), fib(10), fib(50));
    println!("{} {} {}", grade(95), grade(72), grade(30));

    for i in (1..=5).rev() {
        print!("{i} ");
    }
    println!();

    for i in (0..20).step_by(5) {
        print!("{i} ");
    }
    println!();

    // 1..10 is a half-open range (excludes 10); 1..=10 is inclusive and
    // includes 10.
}

// ---------------------------------------------------------------------------
// Task 6 — loop with a value, labelled loops
// ---------------------------------------------------------------------------

// `loop` can return a value via `break value`.
// first_power_of_two_above(5) == 8, first_power_of_two_above(8) == 16.
fn first_power_of_two_above(n: u64) -> u64 {
    let mut p = 1u64;
    loop {
        p *= 2;
        if p > n {
            break p;
        }
    }
}

// ---------------------------------------------------------------------------
// Task 7 — Tuples and arrays
// ---------------------------------------------------------------------------

// Return (min, max, average) of values.
// min_max_avg(vec![3, -1, 7, 2]) == (-1, 7, 2.75)
fn min_max_avg(values: Vec<i32>) -> (i32, i32, f64) {
    let min = *values.iter().min().unwrap();
    let max = *values.iter().max().unwrap();
    let avg = values.iter().sum::<i32>() as f64 / values.len() as f64;
    (min, max, avg)
}

// Swap the two elements of the tuple, returning them in the opposite order
// and with types swapped accordingly.
fn swap_pair(p: (i32, String)) -> (String, i32) {
    (p.1, p.0)
}

pub fn run() {
    task1_formatting();
    task2_variables();
    task3_scalars();
    task4_expressions();
    task5_control_flow();
    println!(
        "first power of two above 5: {}",
        first_power_of_two_above(5)
    );
}
