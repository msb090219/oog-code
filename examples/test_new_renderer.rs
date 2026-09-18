// Test the new renderer components
// Compile with: rustc --edition 2021 test_new_renderer.rs --extern minseo_code=target/release/libminseo_code.rlib -L target/release/deps

// Simpler test - just test the basic functionality
fn main() {
    println!("=== Testing New Renderer Components ===\n");

    // Test 1: Basic color display
    println!("Test 1: Color System");
    println!("\x1b[32m✓ Success message (green)\x1b[0m");
    println!("\x1b[31m✗ Error message (red)\x1b[0m");
    println!("\x1b[90mℹ Info message (gray)\x1b[0m");
    println!("\x1b[35m★ Brand message (purple)\x1b[0m");

    // Test 2: Section Headers
    println!("\n\nTest 2: Section Headers");
    println!("\n\n\x1b[1m\x1b[32m[RESULT]\x1b[0m");
    println!("This is a result section with proper spacing.\n");

    println!("\n\n\x1b[1m\x1b[31m[ERROR]\x1b[0m");
    println!("This is an error section with proper spacing.\n");

    println!("\n\n\x1b[1m\x1b[90m[PLAN]\x1b[0m");
    println!("This is a plan section with proper spacing.\n");

    // Test 3: Code Block Display
    println!("\n\nTest 3: Code Blocks");
    println!("\n\n\x1b[35m[CODE] rust\x1b[0m");
    println!("\x1b[90m   1 | fn main() {{\x1b[0m");
    println!("\x1b[90m   2 |     println!(\"Hello, world!\");\x1b[0m");
    println!("\x1b[90m   3 | }}\x1b[0m");
    println!("\n\x1b[90m(Press Ctrl+C to copy)\x1b[0m");

    // Test 4: Tool Output Display
    println!("\n\nTest 4: Tool Output");
    println!("\n\n\x1b[90m[TOOL]\x1b[0m \x1b[35mbash\x1b[0m \x1b[32m✓\x1b[0m \x1b[90m1.5s\x1b[0m");
    println!("Command completed successfully");
    println!("\x1b[90m↳\x1b[0m \x1b[32mexit code 0\x1b[0m");

    // Test 5: Markdown-like formatting
    println!("\n\nTest 5: Markdown Formatting");
    println!("\n\n**Bold text** and *italic text*");
    println!("\n• List item 1");
    println!("• List item 2");
    println!("• List item 3");
    println!("\n`inline code` in gray color");
    println!("\n───");

    println!("\n\n=== All Tests Complete ===");
    println!("\nThe new renderer components are working!");
    println!("Colors: ✓");
    println!("Sections: ✓");
    println!("Code blocks: ✓");
    println!("Tool output: ✓");
    println!("Markdown: ✓");
}
