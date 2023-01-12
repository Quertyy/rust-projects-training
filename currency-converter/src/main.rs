use std::env;
use std::io::{self, Write};

fn main() {
    println!("Choose your currency !");
    let mut user_input = String::new();
    
    match io::stdin().read_line(&mut user_input) {
        Ok(_) => {
            println!("Vous avez choisi : {user_input}");
        }
        Err(error) => println!("error: {error}"),
    }

}

enum Currency {
    USD(f32),
    EUR(f32),
    GBP(f32),
}