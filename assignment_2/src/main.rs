
// Function tht determins if 32-bit integer is even or odd, returns boolean
fn is_even(n:i32) -> bool {
    n%2 == 0
}

fn main() {
    // Array of 10 random integers
    let arr = [34, 7, 56, 89, 12, 43, 95, 28, 77, 60];

    // Determine if number in array is even or odd and display
    // and display Fizz for %3, Buzz for %5, or FizzBuzz for %3 and %5
    for i in 0..arr.len() {
        let by_3 = arr[i] % 3 == 0;
        let by_5 = arr[i] % 5 == 0;
        // Uses macro print!() to complete phrase for each number
        match is_even(arr[i]) {
            true => print!("{} is even", arr[i]),
            _ => print!("{} is odd", arr[i]),
        }
        match (by_3, by_5) {
            (true, true) => println!(", FizzBuzz!"),
            (true, false) => println!(", Fizz!"),
            (false, true) => println!(", Buzz!"),
            (false, false) => println!("!"),
        }
    }
    
    // Find sum of digits
    let mut sum:i32 = 0;
    let mut idx = 0;
    while idx < arr.len() {
        sum += arr[idx];
        idx += 1;
    }
    println!("The sum of all integers is {}", sum);

    // Find largets value from array
    let mut large_num:i32 = arr[0];
    for i in 1..arr.len() {
        if arr[i] > large_num {
            large_num = arr[i];
        }
    }
    println!("The largest value is {}", large_num);
}
