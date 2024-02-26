pub mod expression;
pub mod settings;

use std::{
    collections::HashMap,
    env,
    io::{self, Write},
    process,
};

use crate::expression::Expression;

fn main() {
    let example_expr: String = "0 & !0 | 3 & !3".into();
    // Args
    let mut verbose: bool = false;
    for argument in env::args() {
        if argument.eq("-v".into()) {
            verbose = true;
        }
    }
    //Input
    println!("\nExpression evaluator\n");
    println!("You can use the following operations:");
    println!("\t{} - and", settings::AND);
    println!("\t{} - or", settings::OR);
    println!("\t{} - negate", settings::NEG);
    println!();
    println!("And use variables in the following format:");
    println!("\t0 - x0");
    println!("\t1 - x1");
    println!("\t...");
    println!();
    println!("Example: {}", example_expr);
    print!("Expression: ");
    io::stdout().flush().unwrap();
    let mut expr_str = String::new();
    if let Err(e) = io::stdin().read_line(&mut expr_str) {
        println!("{}", e);
        process::exit(1);
    }
    expr_str = expr_str.trim_end().into(); // Remove whitespaces from end
    if expr_str.len() == 0 {
        println!("Expression not provided, using example: {}", example_expr);
        expr_str = example_expr;
    }
    expr_str = expr_str.chars().rev().collect(); // Reverse because of pop() function
    let str_len: usize = expr_str.len();

    // Parse
    let mut evaluation_table: HashMap<u32, bool> = HashMap::new();
    let expression: Box<Expression> =
        Expression::parse(&mut expr_str, str_len, &mut evaluation_table);
    println!("Parsed expression: {:?}", expression);

    // Evaluate
    let keys: Vec<u32> = evaluation_table.keys().cloned().collect();
    println!(
        "Trying all {:?} combinations...",
        i32::pow(2, keys.len() as u32)
    );
    if !expression.combinate(&mut evaluation_table, &keys, 0, verbose) {
        println!("This expression is always FALSE!");
    } else {
        println!("This expression can be TRUE!");
    }
}
