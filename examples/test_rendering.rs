use minseo_code::render::TerminalRenderer;

fn main() {
    let renderer = TerminalRenderer::new();
    let test_text = "I'd be happy to help you create a new website! To get started, I need to know a bit more about what you have in mind.";

    println!("Test 1: Direct println");
    println!("{}", test_text);
    println!();

    println!("Test 2: Through render_markdown");
    let markdown = format!("{}\n\n**Tell me about your website:**", test_text);
    let rendered = renderer.render_markdown(&markdown);
    println!("{}", rendered);
    println!();

    println!("Test 3: Check for fragments");
    if test_text.contains("Joy") || test_text.contains("Hope") {
        println!("✗ Fragment detected in source text!");
    } else {
        println!("✓ No fragments in source text");
    }
}
