extern crate rand;

use rand::random;
use std::io::BufferedReader;
use std::io;

fn main() {
    println!("I'm thinking of a number between 1 and 100. Can you guess it?");
    let x = random::<int>();
    let mut reader = BufferedReader::new(io::stdin());
    while true {
        let line_result = reader.read_line();
        if line_result.is_ok() {
            let line = line_result.unwrap();
            let num = from_str::<int>(line.slice_to(line.len() - 1));
            match num {
                Some(number_string) => if number_string == x {println!("Right!"); break;} else {println!("Wrong!")},
                None                => println!("Doesn't count.")
            }
        }
        else {
            println!("An error occurred.");
            break;
        }
    }
}
