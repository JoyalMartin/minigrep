use std::env;
use std::fs;
use std::process;

#[derive(Debug)]
struct Config {
    filename: String,
    query: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
           return  Err("Not enough arguments");
        }
        Ok(Config{filename: args[1].clone(), query: args[2].clone() })
    }
}



fn main() {
    let arguments: Vec<String> = env::args().collect();
    let config_result =  Config::new(&arguments).unwrap_or_else(| mes |{
        println!("{}", mes);
        process::exit(1);
    });
    
    let filename = r"D:\Rust\minigrep\src\poem.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong in reading the file");
    println!("contents: {}", contents);
    println!("config parameters:  {:?}",config_result)
}
