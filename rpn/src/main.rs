//! main.rs - this program reads postfix notation mathematical expressions from a file and solves them. 
//!           It will also print out the expressions re-written in infix notation along with the solutions. 
//! 
//! # Attributes 
//! 
//! **Authors** Fernando Rodriguez & Julia Januchowski
//! **Version** March 15th 2021 

use std::env;
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
     fn solve(&mut self) {
         let pfix = &self.postfix;
         
         let pfix = pfix.split_whitespace();
         println!("this is the line after a split: {:?}", pfix);
     }

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

fn is_white_space(c: char) -> bool {
    if c == ' ' {
        return true
    }
    else {
        return false
    }
}

fn build_expression_list(file_name: &str) -> Result<Vec<Expression>, Error>{
    //opens the file and panics if it is not possible
    let file = File::open(file_name).expect("Failed to open file, check directory");

    // Create a new buffered reader for the file 
    let reader = BufReader::new(file);
    let mut vec_expr: Vec<Expression> = Vec::new();
    // Iterates over all the lines in the file
    for line in reader.lines() {
        let line = line.unwrap();
        println!("Line: {}",line);
        //If line has contents create an expression, otherwise, skip it.
        if line.len() > 0 {
            
            let express = Expression :: new(line);
            vec_expr.push(express);
        }
    }
    Ok(vec_expr)
}

// we need to check for: white space, newline, tab character, invalid characters 
// Needs a syntax check
// add -> Result<Vec<Expression>, E> once this is debugged 
//fn build_expression_list(file_name: &str) {
//    //opens the file and panics if it is not possible
//    let file = File::open(file_name).expect("Failed to open file, check directory");
//
    // Create a new buffered reader for the file 
//    let reader = BufReader::new(file);
//    let mut vec_expr: Vec<Expression> = Vec::new();

//    println!("Contents of the file:");
//    for line in reader.lines() {
//        line.unwrap().split_whitespace();
//        println!("-----Here starts a new line------");
//        let mut postfix_epr = String::from("");

//        for c in line.expect("lines failed").chars() {
//           if is_operator(c) {
//               println!("Character: {}", c);
//               postfix_epr.push(c);
//           }
//           if !is_operator(c) {
//               if !is_white_space(c){
//                postfix_epr.push(c);
//                println!("Character: {}", c);
//               }
//           }
//        }
        // Here is where we create new Expressions 
//        if postfix_epr != "" {
//            println!("this is the expression: {}", postfix_epr);
//            let expression = Expression::new(postfix_epr);
//            vec_expr.push(expression);
//        }
//    }

//    println!("Vector length {}", vec_expr.len());
//}

// fn solve_list() {

// }

// fn sort_list() {

// }

// fn write_to_file() {

// }

fn main() {
    let args: Vec<String> = env::args().collect();

    let result = build_expression_list(&args[1]);
}
