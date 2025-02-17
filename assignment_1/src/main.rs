
/// The freezing point of water in Fahrenheit
const FREEZING_POINT:f64 = 32.0;

/// Converts Fahrenheit to Celsius, returns value as 64-bit float
fn fahrenheit_to_celsius(f: f64) -> f64 {
    (5.0 / 9.0) * (f - FREEZING_POINT)
}

/// Converts Celsius to Fahrenheit, returns value as 64-bit float
fn celsius_to_fahrenheit(c: f64) -> f64 {
    (9.0 / 5.0) * c + FREEZING_POINT
}

fn main() {
    // Temperature 76 in Fahrenheit
    let mut temp:f64 = 76.0;

    // Convert tempurature and next 5 degrees to Celsius
    println!("{} Fahrenheit is {} Celsius", temp, fahrenheit_to_celsius(temp));
    for _ in 0..5 {
        temp = temp + 1.0;
        println!("{} Fahrenheit is {} Celsius", temp, fahrenheit_to_celsius(temp));
    }
}
