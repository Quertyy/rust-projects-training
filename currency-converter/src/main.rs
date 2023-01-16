use std::env;
use std::io::{self, Write};
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();

}

struct Config {
    from: f32,
    to: f32,
    amount: f32,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 4 {
            return Err("Not enough args");
        }
        let from = args[1].clone();
        let to= args[2].clone();
        let amount= args[3].clone();
    }
}