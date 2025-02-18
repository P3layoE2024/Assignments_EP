
//#[allow(unused_variables, unused_mut)]
fn sum(total: &mut i32, low: i32, high: i32) {
    // Write your code here!
    *total = 0;
    for i in low..=high {
        *total += i;
    }
}

fn main() {
    // create necessary variables and test your function for low 0 high 100
    let mut total = 0;
    let low:i32 = 0;
    let high:i32 = 100;
    // total should be 5050
    sum(&mut total, low, high);
    println!("The total sum from {} to {} is {}", low, high, total);
}
