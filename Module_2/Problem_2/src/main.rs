
fn clone_and_modify(s: &String) -> String {
    // Creates a clone of s and adds "World"
    s.clone() + "World!"
}

fn main() {
    let s = String::from("Hello, ");
    let modified = clone_and_modify(&s);
    println!("Original: {}", s); // Should print: "Original: Hello, "
    println!("Modified: {}", modified); // Should print: "Modified: Hello, World!"
}
