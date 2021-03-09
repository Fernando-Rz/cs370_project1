//! main.rs - this program reads postfix notation mathematical expressions from a file and solves them. 
//!           It will also print out the expressions re-written in infix notation along with the solutions. 
//! 
//! # Attributes 
//! 
//! **Authors** Fernando Rodriguez & Julia Januchowski
//! **Version** March 15th 2021 

use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};
use std::process::exit; 

// This struct respresents the object "Expression"
struct Expression {
    postfix: String,  
    expr: Vec<f64>, // might need to change this 
    infix: Vec<String>,
}

//Here we implement associated methods 
impl Expression { 
    fn new(input_line: String) -> Expression {
        Expression {
            postfix : input_line,
            expr: Vec::new(),  
            infix: Vec::new(), 
        }
    }

    //solves the postfix expression 
    // TODO: write this function 
    // fn solve(&mut self) {
    //     self.postfix = ""
    // }

    // need to print out the vectors in a different way or it will error 
    // fn to_string(&self) -> String {
    //     format!("{} - {} - {}", self.postfix, self.expr, self.infix)
    // }
}

fn is_operator(op: char) -> bool {
    if op == '+' || op == '-' || op == '*' || op == '/' {
        return true 
    }
    else {
        return false 
    }
}

// Needs a syntax check
// add -> Result<Vec<Expression>, E> once this is debugged 
fn build_expression_list(file_name: &str) {
    //opens the file and panics if it is not possible
    let file = File::open(file_name).expect("Failed to open file, check directory");

    // Create a new buffered reader for the file 
    let reader = BufReader::new(file);

    println!("Contents of the file:");
    for line in reader.lines() {
        println!("-----Here starts a new line------");
        for c in line.expect("lines failed").chars() {
           println!("Character: {}", c);
        }
    }
}

// fn solve_list() {

// }

// fn sort_list() {

// }

// fn write_to_file() {

// }

fn main() {
    println!("Hello, world!");
    build_expression_list("ex.dat")
}
