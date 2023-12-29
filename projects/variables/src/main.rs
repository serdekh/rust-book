use std::io;
use std::process;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    match io::stdin().read_line(&mut index) {
        Err(error) => {
            println!("Error: Failed to read line: {error}.");
            process::exit(1);
        }
        _ => {}
    }
        
    let index: usize = match index.trim().parse() {
        Ok(number) => number,
        Err(error) => {
            println!("Error: Failed to convert the input string to a number: {error}");
            process::exit(1);
        }
    };

    if index >= 5 {
        println!("Error: Invalid bounds. The index should be from 0 up to 4 (included).");
        process::exit(1);
    }

    println!("Array value: {}", a[index]);
}