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

struct Expression {
    postfix: String,  
    expr: Vec<f64>,  
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

    fn solve(&mut self) {
         let pfix = &self.postfix;
         
         let pfix = pfix.split_whitespace();
         for element in pfix {
            if !is_operator(element) {
                self.expr.push(element.parse::<f64>().expect("Cannot parse value to f64"));
                self.infix.push(element.to_string());
            }
            else {
                get_type(element, &mut self.expr, &mut self.infix);
                println!("Vector is {:?}", self.infix);
            }
         }
    }
}

fn is_operator(op: &str) -> bool {
    if op == "+" || op == "-" || op == "*" || op == "/" {
        return true
    }
    else {
        return false
    }
}
 
fn get_type(input_val: &str, expr_stack: &mut Vec<f64>, infix_stack: &mut Vec<String>) {
    //operations that happen with all operators
    let op2 = expr_stack[expr_stack.len()-1];
    expr_stack.pop();
    let op1 = expr_stack[expr_stack.len()-1];
    expr_stack.pop();
    
    let mut answer = 0.0;
    //let mut infix_expr = String::from("");
    let infix_expr_2 = infix_stack.pop().expect("Invalid input contained in file");
    let infix_expr_1 = infix_stack.pop().expect("Invalid input contained in file");

    match input_val {
        "+" => {
            answer = op1 + op2;
            }
        "-" => {
            answer = op1 - op2;
            }
        "*" => {
            answer = op1 * op2;
            }
        _ => {
            answer = op1 / op2;
            
            }
    }
    let infix_string = format!("({} {} {})", infix_expr_1,input_val,infix_expr_2);
    expr_stack.push(answer);
    infix_stack.push(infix_string);
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

//here we just iterate over the vector of expressions and call solve() on all of them 
fn solve_list(exp_list: Vec<Expression>) {
    for mut i in exp_list {
        i.solve();
        println!("Solved one vector");
    }
}

//iterate over expressions and sort based on expr[0]
// fn sort_list() {

// }

// fn write_to_file() {

// }

fn main() {
    let args: Vec<String> = env::args().collect();
    //change to 3 once we get writing to a file working 
    if args.len() != 2 {
        println!("Usage: cargo run [input file] [output file]");
        exit(1);
    }
    
    let result = build_expression_list(&args[1]).unwrap();
    solve_list(result)
}
