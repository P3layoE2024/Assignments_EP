
fn concat_strings(s1: &String, s2: &String) -> String {
    let mut ans = String::from(s1); // Creates a mutable string varuable to concat s1 and s2
    ans.push_str(s2); // Adds the borrows string s1
    ans
}

fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let result = concat_strings(&s1, &s2);
    println!("{}", result); // Should print: "Hello, World!"
}
