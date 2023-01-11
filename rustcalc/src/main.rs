use std::env;
use std::process;
use std::io::{self, Write};

fn main() {
    helper();
    let mut user_input = String::new();

    match io::stdin().read_line(&mut user_input) {
        Ok(_) => match user_input.to_uppercase().trim() {
            "A" => Mode::Addition,
            "S" => Mode::Substraction,
            "M" => Mode::Multiplication,
            "D" => Mode::Division,
            _ => {
                println!("Invalid mode: {}", user_input);
                //main();
            },
        }
        Err(error) => println!("error: {error}"),
    }
}

struct Config {
    a: u64,
    b: u64,
}

enum Mode {
    Addition,
    Substraction,
    Multiplication,
    Division,
}

impl Mode {

}

fn helper() {
    println!("Choose a mode:\n - Addition [A]\n - Substration [S]\n - Multiplication [M]\n - Division [D]");
}

fn addition(a: u64, b: u64) -> u64 {
    a+b
}

fn substraction(a: u64, b: u64) -> u64 {
    a-b
}

fn multiplication(a: u64, b: u64) -> u64 {
    a*b
}

fn division(a: u64, b: u64) -> u64 {
    a/b
}
