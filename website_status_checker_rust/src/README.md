# Website Status Checker Rust

The following program can find the status of a given website and display if it was successful or if there was an error. The program displays the result on the terminal and crete a JSON file.

The current program has functionality of a single url providede by the command line interface. The functionality that works without error is 'cargo run [site]'. 

There is implementation to read from a file, adjust workers, and tries attempted, but code was not completed.

# Implementation

The program works by creating a thread to test weather the url given had a Ok status or an error status. The site calls on function parse_args to create the 4 necessary variables used in main, (urls [String Vector], workers [default = 1], timeout [default = 5], and retires[default = 0]).

The function check_website recieves the Client declared in main, which enforces the timeout for each check. The function returns the result as on OK status or an Error status if the functino exceeds the number of tries ir could attempt. 

The resultant struct from the function is displayed on the terminal and is also written to a JSON file as 'status.json'. 

# Example

cargo build --release

'Ok EXAMPLE'

cargo run https://www.google.com

'Err EXAMPLE' Misspelling of yahoo.com

cargo run https://www.yaho.com
