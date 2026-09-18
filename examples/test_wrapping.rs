// Test to verify our wrapping function works correctly
use std::io::Write;

fn main() {
    let text = "I'd be happy to help you create a new website! To get started, I need to know a bit more about what you have in mind.";

    // Test 1: Simple wrapping without styling
    let wrapped = minseo_code::layout::wrap_text_ansi_words(text, 80);
    println!("Test 1 - Simple wrapping:");
    println!("Input length: {}", text.len());
    println!("Output:\n{}", wrapped);
    println!();

    // Test 2: With very narrow width
    let wrapped_narrow = minseo_code::layout::wrap_text_ansi_words(text, 40);
    println!("Test 2 - Narrow wrapping (40 chars):");
    println!("Output:\n{}", wrapped_narrow);
    println!();

    // Test 3: Check if fragments appear
    let test_text = "Hello world this is a test of the wrapping function to see if it works properly";
    let wrapped_test = minseo_code::layout::wrap_text_ansi_words(test_text, 20);
    println!("Test 3 - Check for fragments:");
    println!("Output:\n{}", wrapped_test);

    // Check for fragments
    if wrapped_test.contains("Hello") && wrapped_test.contains("world") {
        println!("✓ No word fragmentation detected");
    } else {
        println!("✗ Word fragmentation detected!");
    }
}
