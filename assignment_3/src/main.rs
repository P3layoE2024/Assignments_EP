
/// Compare function, returns 0 if guess is equal, 1 if too high, -1 if too low
fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret{
        0
    }
    else if guess > secret {
        1
    }
    else {
        -1
    }
}


fn main() {
    // Hard coded secret number
    let secret:i32 = 67;
    // User guess, use to simulate input
    let mut guess:i32 = 47;
    // Holds number of guesses
    let mut guess_count = 1;
    // Guessing Game
    loop{
        // Call check_guess to verify guess
        let result:i32 = check_guess(guess, secret);
        if result == 1 {
            println!("Your guess is too high!");
        }
        else if result == -1 {
            println!("Your guess is too low!");
        }
        else {
            println!("You got it!");
            break; // Exit game number has been found
        }
        // Increment guess_count
        guess += 1;     // used to simluate another guess
        guess_count += 1;
    }
    // Display number of guesses
    println!("Total number of guesses: {}", guess_count);
}
