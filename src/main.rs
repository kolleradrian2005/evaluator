pub mod expression;
pub mod settings;

use std::collections::HashMap;

use crate::expression::Expression;

fn main() {
    //Input
    let mut expr_str: String = "0 & !0 | 3 & !3".into();
    expr_str = expr_str.chars().rev().collect(); // Reverse because of pop() function
    let str_len: usize = expr_str.len();

    // Parse
    let mut evaluation_table: HashMap<u32, bool> = HashMap::new();
    let expression: Box<Expression> =
        Expression::parse(&mut expr_str, str_len, &mut evaluation_table);
    println!("Expression: {:?}", expression);

    // Evaluate
    let keys: Vec<u32> = evaluation_table.keys().cloned().collect();
    println!(
        "Trying all {:?} combinations...",
        i32::pow(2, keys.len() as u32)
    );
    if !expression.combinate(&mut evaluation_table, &keys, 0) {
        println!("This expression is always FALSE!");
    } else {
        println!("This expression can be TRUE!");
    }
}
