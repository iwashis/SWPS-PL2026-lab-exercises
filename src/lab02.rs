//! Lab 2 — Ownership, Borrowing and Structs — exercises.
//!
//! Fill in each `todo!()` with a working implementation. Everything here
//! compiles as-is (a stub); running it will panic on the first `todo!()`

#![allow(dead_code, unused_variables, unused_mut)]

// ---------------------------------------------------------------------------
// Task 1 — Predict: move, copy, clone
// ---------------------------------------------------------------------------
// For each snippet below, decide whether it compiles, and if so what it
// prints. Several of them are rejected by the compiler on purpose, so they
// can't live in this file as real code.
// (a)
// let a = 5;
// let b = a;
// println!("{a} {b}");
//
// (b)
// let s1 = String::from("hi");
// let s2 = s1;
// println!("{s1} {s2}");
//
// (c)
// let s1 = String::from("hi");
// let s2 = s1.clone();
// println!("{s1} {s2}");
//
// (d)
// fn take(s: String) -> usize { s.len() }
// let s = String::from("hello");
// let n = take(s);
// println!("{n} {s}");
//
// (e)
// let t = (1, 2.0, 'x');
// let u = t;
// println!("{t:?} {u:?}");
//
// (f)
// let v = vec![1, 2, 3];
// for x in v { print!("{x} "); }
// println!("{}", v.len());
//
// ---------------------------------------------------------------------------
// Task 2 — Fix the borrow checker
// ---------------------------------------------------------------------------
// Each snippet in the comments below is rejected by the compiler as shown.
// Read the full error (`rustc --explain E0382` etc.), then fix it WITHOUT
// simply adding `.clone()` everywhere, and explain your fix in a comment.

// (a) E0382: borrow of moved value.
//     fn print_len(s: String) { println!("{}", s.len()); }
//     fn main() {
//         let s = String::from("hello");
//         print_len(s);
//         println!("{s}");
//     }
fn print_len(s: &String) {
    println!("{}", s.len());
}
fn task2() {
    let s = String::from("hello");
    print_len(&s);
    println!("{s}");
}

// (e) E0596: cannot borrow as mutable.
//     fn add_exclamation(s: &String) { s.push('!'); }
fn add_exclamation(s: &mut String) {
    s.push('!');
}

// (c) Call make_greeting("Ada") and print the result.
// (d) E0499: `let mut s = String::from("abc"); let r1 = &mut s; let r2 =
//     &mut s; r1.push('d'); r2.push('e'); println!("{s}");` — make the two
//     mutable borrows not overlap.
// (e) Call add_exclamation on a mutable String and print it.
fn task2_fixes() {
    todo!()
}

// ---------------------------------------------------------------------------
// Task 3 — Strings and slices
// ---------------------------------------------------------------------------
// `String` is an owned, growable, heap-allocated UTF-8 buffer. `&str` is a
// borrowed view into UTF-8 data. Functions that only read text should take
// `&str` — they then accept both `&String` and literals.

// first_word("hello world") == "hello", first_word("single") == "single"
fn first_word(s: &str) -> &str {
    todo!()
}

// count_vowels("AbcdE") == 2. Count aeiou, ignoring case. Use s.chars().
fn count_vowels(s: &str) -> usize {
    todo!()
}

// reverse_words("the quick brown fox") == "fox brown quick the".
// Use split_whitespace.
fn reverse_words(s: &str) -> String {
    todo!()
}

// capitalize("rust") == "Rust", capitalize("") == "".
fn capitalize(s: &str) -> String {
    todo!()
}

// is_palindrome("A man, a plan, a canal: Panama") == true. Ignore case and
// anything that is not alphanumeric.
fn is_palindrome(s: &str) -> bool {
    todo!()
}

// task3_strings:
// Call all five functions above with the example values and print the
// results.
// Questions (answer as comments):
// - let s = "zażółć"; what are s.len() and s.chars().count()? Why differ?
// - Why does s[0] not compile for a String, while &s[0..1] compiles? What
//   happens with &"żółw"[0..1]?
// - Why does the signature first_word(s: &str) -> &str guarantee that the
//   result cannot outlive s?
fn task3_strings() {
    todo!()
}

// ---------------------------------------------------------------------------
// Task 4 — Vectors and slices
// ---------------------------------------------------------------------------

// running_sum(&[1, 2, 3, 4]) == vec![1, 3, 6, 10].
fn running_sum(values: &[i32]) -> Vec<i32> {
    todo!()
}

// Multiply every element by 2 in place. Use values.iter_mut() (or
// `for x in values`) and *x *= 2.
fn double_in_place(values: &mut [i32]) {
    todo!()
}

// Keep the first occurrence of each value, preserving order:
// [3, 1, 3, 2, 1] -> [3, 1, 2]. Hint: build a new Vec, check `contains`,
// then `*values = result` (also look at `retain`).
fn remove_duplicates(values: &mut Vec<i32>) {
    todo!()
}

// largest(&[4, 9, 2]) == Some(9), largest(&[]) == None.
fn largest(values: &[i32]) -> Option<i32> {
    todo!()
}

// task4_vectors:
// Build v = vec![3, 1, 3, 2, 1], call remove_duplicates then
// double_in_place on it, then print v, running_sum(&v) and largest(&v).
// Question (answer as a comment): a function takes &[i32] and you have a
// Vec<i32> — why can you pass &v? (look up "deref coercion")
fn task4_vectors() {
    todo!()
}

// ---------------------------------------------------------------------------
// Task 5 — Structs and methods
// ---------------------------------------------------------------------------
// What does each derive give you? Remove `Copy` and see what breaks.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    const ORIGIN: Point = Point { x: 0.0, y: 0.0 };

    fn new(x: f64, y: f64) -> Self {
        todo!()
    }

    fn distance(&self, other: &Point) -> f64 {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Rectangle {
    top_left: Point,
    width: f64,
    height: f64,
}

impl Rectangle {
    fn new(top_left: Point, width: f64, height: f64) -> Self {
        todo!()
    }

    // Associated function, not a method.
    fn square(top_left: Point, size: f64) -> Self {
        todo!()
    }

    fn area(&self) -> f64 {
        todo!()
    }

    // y grows downwards, like on a screen.
    fn contains(&self, p: Point) -> bool {
        todo!()
    }

    // Compare sizes only.
    fn can_hold(&self, other: &Rectangle) -> bool {
        todo!()
    }

    fn scale(&mut self, factor: f64) {
        todo!()
    }
}

// Newtype pattern. What mistake does this prevent compared with passing
// bare f64s around?
struct Meters(f64);
struct Feet(f64);

fn to_feet(m: Meters) -> Feet {
    todo!()
}

// task5_structs:
// 1. Create a Point and print its distance to Point::ORIGIN.
// 2. Create a Rectangle and a square Rectangle (Rectangle::square), print
//    area(), contains() and can_hold().
// 3. Scale the rectangle by 2.0 and print it with {r:?}.
// 4. Struct update syntax: let r2 = Rectangle { width: 10.0, ..r1 }; print
//    both r1 and r2. Can you still use r1 afterwards? Why? (Hint: are all
//    fields Copy?)
// 5. Print to_feet(Meters(10.0)).0.
// As a comment: explain the three kinds of receivers — self, &self, &mut
// self — and give an example where you'd want `self` (hint: "consuming"
// methods, such as turning a builder into its final value).
fn task5_structs() {
    todo!()
}

// ---------------------------------------------------------------------------
// Task 6 — Owning data in structs: a library
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
struct Book {
    title: String,
    author: String,
    year: u16,
}

impl Book {
    fn new(title: &str, author: &str, year: u16) -> Self {
        todo!()
    }
}

struct Library {
    books: Vec<Book>,
}

impl Library {
    fn new() -> Self {
        todo!()
    }

    // The library TAKES OWNERSHIP of the book.
    fn add(&mut self, book: Book) {
        todo!()
    }

    // Borrowed views of all titles.
    fn titles(&self) -> Vec<&str> {
        todo!()
    }

    // Borrowed references to matching books.
    fn by_author(&self, author: &str) -> Vec<&Book> {
        todo!()
    }

    fn oldest(&self) -> Option<&Book> {
        todo!()
    }

    // Gives ownership back to the caller.
    fn remove(&mut self, title: &str) -> Option<Book> {
        todo!()
    }
}

// task6_library:
// Build a Library, add a few books, then print titles(), by_author(...),
// oldest() and remove(...).
// Question (answer as a comment): after `let t = lib.titles();`, can you
// call `lib.add(...)` and then print `t`? Why not?
fn task6_library() {
    todo!()
}

// ---------------------------------------------------------------------------
// Task 7 — The VM value stack
// ---------------------------------------------------------------------------
// Your virtual machine will be a stack machine: instructions pop their
// operands from the stack and push the result.

#[derive(Debug)]
struct Stack {
    items: Vec<i64>,
}

impl Stack {
    fn new() -> Self {
        todo!()
    }

    fn push(&mut self, v: i64) {
        todo!()
    }

    fn pop(&mut self) -> Option<i64> {
        todo!()
    }

    fn peek(&self) -> Option<&i64> {
        todo!()
    }

    fn len(&self) -> usize {
        todo!()
    }

    fn is_empty(&self) -> bool {
        todo!()
    }

    // Pop b, pop a, push a + b. Return false (stack unchanged) if there are
    // not enough operands.
    fn add(&mut self) -> bool {
        todo!()
    }

    // Push a - b (mind the order!).
    fn sub(&mut self) -> bool {
        todo!()
    }

    fn mul(&mut self) -> bool {
        todo!()
    }

    // Duplicate the top value.
    fn dup(&mut self) -> bool {
        todo!()
    }

    // Swap the two top values.
    fn swap(&mut self) -> bool {
        todo!()
    }
}

// task7_stack:
// Compute (2 + 3) * 4 - 1 using only push and the instruction methods
// above, printing the stack with {:?} after each step. Then dup, push(0),
// swap, and pop everything while printing as you go.
// Reflect (as a comment): returning bool is not very expressive — the
// caller cannot tell WHY something failed, and it's easy to ignore the
// result. In Lab 3 you will replace it with Result<(), VmError>.
fn task7_stack() {
    todo!()
}

pub fn run() {
    task2_fixes();
    task3_strings();
    task4_vectors();
    task5_structs();
    task6_library();
    task7_stack();
}
