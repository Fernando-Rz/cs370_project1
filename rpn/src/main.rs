//! main.rs - this program reads postfix notation mathematical expressions from a file and solves them. 
//!           It will also write the solved infix expression out to a file along with the solutions from smallest to greatest. 
//! 
//! # Attributes 
//! 
//! **Authors** Fernando Rodriguez & Julia Januchowski
//! **Version** March 15th 2021 

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};
use std::process::exit; 

// A struct that symbolizes an postfix expression 
struct Expression {
    // A String that holds a single postfix expression 
    postfix: String,  
    // A vector of f64's used to calculate the numerical solution 
    expr: Vec<f64>,  
    // A vector of Strings, used to calculate the infix version of the expression 
    infix: Vec<String>,
}

//Here we implement associated functions and methods for the Expression 
impl Expression {
    /// A constructor for Expression, returns an Expression given a line of input
    /// 
    /// # Arguments 
    /// 
    /// * 'input_line' - a String containing a line of input from the file  
    fn new(input_line: String) -> Expression {
        Expression {
            postfix : input_line,
            expr: Vec::new(),  
            infix: Vec::new(), 
        }
    }
    
    /// Solves the postfix expressions for the current expression, using a helper method 
    ///
    /// # Arguments
    /// * 'self' - The current expression we are working on
    fn solve(&mut self) {
        let pfix = &self.postfix;
        let pfix = pfix.split_whitespace();
        //For every element in postfix 
        for element in pfix {
            // If the current element is not an operator try to parse it into a f64 if it does not 
            // parse it, it is not an operator or a number so throw an error.
            if !is_operator(element) {
                self.expr.push(element.parse::<f64>().expect("Invalid character found, cannot parse to f64"));
                self.infix.push(element.to_string());
            }
            //if it is an operator call the "perform_op" function to solve the current infix expression
            else {
                perform_op(element, &mut self.expr, &mut self.infix);
            }
        }
    }
}

/// Determines whether a given string slice is an operator 
///
/// # Arguments
/// * 'op' - The value to check 
/// 
/// # Returns
/// a boolean value - true if it is an operator
///                 - false if it is not an operator 
fn is_operator(op: &str) -> bool {
    if op == "+" || op == "-" || op == "*" || op == "/" {
        return true
    }
    else {
        return false
    }
}

/// Determines the type of operator, then preforms the operation as well as formats the infix expression
/// this is a helper to the solve method 
///
/// # Arguments
/// * 'input_val' - the current operator
/// * 'expr_stack' - The current expression vector
/// * 'infix_stack' - the current infix expression vector 
fn perform_op(input_val: &str, expr_stack: &mut Vec<f64>, infix_stack: &mut Vec<String>) {
    //operations that happen with all operators
    let op2 = expr_stack.pop().expect("Invalid input contained in file");    // get the last values and store them, then pop it off 
    let op1 = expr_stack.pop().expect("Invalid input contained in file");
    
    // holds the answer to an expression
    let answer : f64;

    let infix_expr_2 = infix_stack.pop().expect("Invalid input contained in file");
    let infix_expr_1 = infix_stack.pop().expect("Invalid input contained in file");
    
    // a match statement used to determine what operator is being used, and then used to peform that
    // operation and push it onto the expr_stack
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
    // pushes the correct infix expression onto the infix field of the expression
    let infix_string = format!("({} {} {})", infix_expr_1,input_val,infix_expr_2);
    expr_stack.push(answer);
    infix_stack.push(infix_string);
}

/// Builds a list of expressions by reading the current line of the input file and turning them into
/// Expressions and then pushes it onto a vector.
/// 
/// # Arguments
/// * 'file_name' - a string slice that holds the name of the input file
/// 
/// # Returns
/// a result vector holding postfix expressions or an error
/// 
fn build_expression_list(file_name: &str) -> Result<Vec<Expression>, Error>{
    //opens the file and panics if it is not possible
    let file = File::open(file_name).expect("Failed to open file, check directory");

    // Create a new buffered reader for the file 
    let reader = BufReader::new(file);
    let mut vec_expr: Vec<Expression> = Vec::new();
    // Iterates over all the lines in the file
    for line in reader.lines() {
        let line = line.unwrap();
        //If line has contents create an expression, otherwise, skip it.
        if line.len() > 0 {
            
            let express = Expression :: new(line);
            vec_expr.push(express);
        }
    }
    Ok(vec_expr)
}

/// Iterates through a vector of Expressions and calls the solve() method
/// 
/// # Arguments 
/// * 'exp_list' - A vector containing Expressions
fn solve_list(exp_list: &mut Vec<Expression>) {
    for i in exp_list {
        // call the method solve() for all expressions 
        i.solve();
        // if the expression is empty we add f64:MAX for error/printing purposes
        if i.expr.len() == 0{
            i.expr.push(f64::MAX)
        }
    }
}

/// Sorts the vector of expressions from smallest to biggest using bubble sort
/// 
/// # Arguments
/// * 'exp_list' - A vector containing Expressions
fn sort_list(exp_list: &mut Vec<Expression>) {
    for i in 0..exp_list.len() {
        for j in 0..(exp_list.len() - i -1) {
            if exp_list[j + 1].expr[0] < exp_list[j].expr[0] {
                exp_list.swap(j, j + 1 );
            }
        }
    }
}

/// Takes a reference to a string slice representing the output file and writes the solved infix 
/// expressions to it. 
/// 
/// # Arguments 
/// * 'file_name' - a string slice representing the output file 
/// * 'exp_list'  - a vector of expressions 
/// 
/// # Returns 
/// an empty result or error if the file cannot be created/written to properly
fn write_to_file(file_name: &str, exp_list: &mut Vec<Expression>) -> Result<(), Error> {
    let mut out_buffer = File::create(file_name).expect("Unable to create output file");
    
    for exp in exp_list{
        // if the value in the exp vector is f64::MAX we dont write its contents
        // becuase lines that are empty are set to MAX  
        if exp.expr[0] != f64::MAX {

            let mut infix_out = exp.infix.get(0).unwrap().to_string();
            infix_out.remove(0);                    // removes the first 
            infix_out.remove(infix_out.len()-1);    // and last parenthesis 
            let exp_out = exp.expr.get(0).unwrap().to_string();
            
            // writes to the file 
            out_buffer.write(&format!("{} = {}\n", infix_out, exp_out).into_bytes()).expect("Could not write to file");
        }
    }
    Ok(())
 }

/// Parses and handles command line arguments and contains the logic to run the program
fn main() {
    // gets the command line arguments 
    let args: Vec<String> = env::args().collect();
    
    // if there isnt enough or too many command line arguments, print a usage message and quit 
    if args.len() != 3 {
        println!("Usage: cargo run [input file] [output file]");
        exit(1);
    }
    
    // result will be the vector that holds all the expressions
    let result = &mut build_expression_list(&args[1]).unwrap();     // build the vector of expressions 
    solve_list(result);                                             // solve each expression
    sort_list(result);                                              // sort it 
    write_to_file(&args[2], result).expect("Could not write to the file");
}
