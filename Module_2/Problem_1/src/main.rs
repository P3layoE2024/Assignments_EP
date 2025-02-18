
fn concat_strings(s1: &String, s2: &String) -> String {
    let mut ans = String::from(s1);
    ans.push_str(s2);
    ans
}

fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let result = concat_strings(&s1, &s2);
    println!("{}", result); // Should print: "Hello, World!"
}
