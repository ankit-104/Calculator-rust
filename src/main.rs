use Calculator::calculator::calculator;
use std::io::{self,Write};

fn main() {
    let mut calculator=calculator::new();
    println!("Enter the expression you want to calculate :");
    io::stdout().flush().unwrap();
    let mut input=String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input=input.trim();
    match calculator.evaluate(input){
        Ok(value) => println!("Result : {value}"),
        Err(error) => println!("Error : {error}"),
    }
}
