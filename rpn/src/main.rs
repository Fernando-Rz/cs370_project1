use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};
use std::process::exit; 

// This struct respresents the object "Expression"
struct Expression {
    postfix: String,  
    expr: Vec<f64>, //
    infix: Vec<String>,
}

// Here we implement associated functions and methods 
impl Expression {
    fn new(postfix: String, expr: Vec<f64>, infix: Vec<String>) -> Expression {
        Expression {
            postfix : postfix,
            expr : expr,
            infix : infix,
        }
    }
}



fn main() {
    println!("Hello, world!");
}
