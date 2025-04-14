// Task 3
fn process_vector<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{   
    vec.into_iter().map(f).collect()
    //let mut result = Vec::new();
    //for x in vec {
    //    result.push(f(x));
    //}
    //result
}

fn main() {
    let numbers = vec![1, 2, 3];

    let doubled = process_vector(numbers.clone(), |x| {
        // Implement: multiply each number by 2
        return 2 * x;
    });

    let replaced = process_vector(numbers, |x| {
        // Implement: if number > 2, replace with 0, else keep number
        if x < 2 {return 0}
        else {return x}
    });

    println!("Doubled: {:?}", doubled);
    println!("Replaced: {:?}", replaced);
}