// Rust Ownership System Examples
// This file demonstrates the core concepts of Rust's ownership system

fn main() {
    println!("=== Rust Ownership System Examples ===\n");

    // ============================================
    // 1. BASIC OWNERSHIP
    // ============================================
    basic_ownership();
    
    // ============================================
    // 2. MOVE SEMANTICS
    // ============================================
    move_semantics();
    
    // ============================================
    // 3. CLONING
    // ============================================
    cloning();
    
    // ============================================
    // 4. BORROWING - REFERENCES
    // ============================================
    borrowing();
    
    // ============================================
    // 5. MUTABLE REFERENCES
    // ============================================
    mutable_references();
    
    // ============================================
    // 6. SLICE REFERENCES
    // ============================================
    slice_references();
}

// ============================================
// 1. BASIC OWNERSHIP
// ============================================
fn basic_ownership() {
    println!("1. BASIC OWNERSHIP");
    
    // String is allocated on the heap
    let s1 = String::from("hello");
    println!("s1 = {}", s1);
    
    // s1 is MOVED to s2, not copied
    let s2 = s1;
    // println!("{}", s1); // ERROR: value borrowed here after move
    
    println!("s2 = {}", s2);
    println!("s1 is no longer valid - ownership moved to s2\n");
    
    // When s2 goes out of scope, it's dropped and memory is freed
}

// ============================================
// 2. MOVE SEMANTICS
// ============================================
fn move_semantics() {
    println!("2. MOVE SEMANTICS");
    
    let s = String::from("ownership");
    take_ownership(s); // s is moved into the function
    // println!("{}", s); // ERROR: s was moved
    
    // Can't use s anymore, but function returned it
    let s = give_ownership();
    println!("Got ownership of: {}\n", s);
    
    // Demonstrating moves with function parameters
    let x = 5;
    makes_copy(x); // x is copied, not moved (i32 implements Copy)
    println!("x is still valid: {}", x);
}

fn take_ownership(some_string: String) {
    println!("Function took ownership of: {}", some_string);
} // some_string is dropped here, memory is freed

fn give_ownership() -> String {
    let some_string = String::from("yours");
    some_string // ownership is moved to calling function
}

fn makes_copy(some_integer: i32) {
    println!("Function copied: {}", some_integer);
} // some_integer goes out of scope, but nothing special happens

// ============================================
// 3. CLONING
// ============================================
fn cloning() {
    println!("3. CLONING");
    
    let s1 = String::from("clone me");
    let s2 = s1.clone(); // Deep copy - expensive but explicit
    
    println!("s1 = {}, s2 = {}", s1, s2);
    println!("Both s1 and s2 are valid (deep copy)\n");
    
    // Types that implement Copy trait are copied automatically
    let x = 5;
    let y = x; // Copy, not move
    println!("x = {}, y = {} (both valid)", x, y);
}

// ============================================
// 4. BORROWING - REFERENCES
// ============================================
fn borrowing() {
    println!("4. BORROWING - REFERENCES");
    
    let s1 = String::from("hello world");
    
    // Borrow s1 - create a reference
    let len = calculate_length(&s1);
    
    println!("The length of '{}' is {}", s1, len);
    println!("s1 is still valid - we only borrowed it\n");
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope, but since it doesn't have ownership, nothing happens

// ============================================
// 5. MUTABLE REFERENCES
// ============================================
fn mutable_references() {
    println!("5. MUTABLE REFERENCES");
    
    let mut s = String::from("hello");
    
    // Mutable borrow
    change(&mut s);
    println!("After change: {}\n", s);
    
    // Multiple mutable references - restricted by Rust
    let mut s2 = String::from("test");
    
    let r1 = &mut s2;
    // let r2 = &mut s2; // ERROR: cannot borrow as mutable more than once
    println!("First mutable ref: {}", r1);
    
    // But we can have multiple immutable references
    let r3 = &s2;
    let r4 = &s2;
    println!("Multiple immutable refs work: {} and {}", r3, r4);
    
    // Or one mutable reference (but not both)
    let r5 = &mut s2;
    println!("Mutable ref after immutable refs: {}\n", r5);
    
    // Dangling references are prevented at compile time
    // let reference_to_nothing = dangle(); // Would not compile
    let valid_ref = no_dangle();
    println!("Valid reference: {}", valid_ref);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// This would not compile - prevents dangling references
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // ERROR: returns a reference to data that will be dropped
// }

fn no_dangle() -> String {
    let s = String::from("hello");
    s // ownership is moved, so no dangle
}

// ============================================
// 6. SLICE REFERENCES
// ============================================
fn slice_references() {
    println!("6. SLICE REFERENCES");
    
    let s = String::from("hello world");
    
    let hello = &s[0..5]; // [0, 5)
    let world = &s[6..11]; // [6, 11)
    
    println!("Slices: '{}' and '{}'", hello, world);
    
    // String slices as parameters
    let word = first_word(&s);
    println!("First word: {}", word);
    
    // String literals are slices
    let literal = "Hello, world!";
    println!("String literal is a slice: {}", literal);
    
    // Array slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("Array slice: {:?}", slice);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}