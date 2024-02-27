use std::{collections::HashMap, fmt};

use crate::settings::{AND, NEG, OR};

pub enum Expression {
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    Neg(Box<Expression>),
    Value(u32), // 0 for x0, 1 for x1, etc
}

enum Operator {
    And,
    Or,
}

impl fmt::Debug for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::And(a, b) => write!(f, "({:?} & {:?})", a, b),
            Expression::Or(a, b) => write!(f, "({:?} | {:?})", a, b),
            Expression::Neg(a) => write!(f, "!{:?}", a),
            Expression::Value(a) => write!(f, "{:?}", a),
        }
    }
}

impl Expression {
    pub fn evaluate(&self, evaluation_table: &HashMap<u32, bool>) -> bool {
        match self {
            Expression::And(a, b) => a.evaluate(evaluation_table) && b.evaluate(evaluation_table),
            Expression::Or(a, b) => a.evaluate(evaluation_table) || b.evaluate(evaluation_table),
            Expression::Neg(a) => !a.evaluate(evaluation_table),
            Expression::Value(a) => *evaluation_table.get(a).unwrap(),
        }
    }

    pub fn combinate(
        &self,
        evaluation_table: &mut HashMap<u32, bool>,
        keys: &Vec<u32>,
        index: usize,
        verbose: bool,
    ) -> bool {
        if index < keys.len() {
            let key = keys[index];
            evaluation_table.insert(key, true);
            if self.combinate(evaluation_table, keys, index + 1, verbose) {
                return true;
            }
            evaluation_table.insert(key, false);
            self.combinate(evaluation_table, keys, index + 1, verbose)
        } else {
            let result = self.evaluate(evaluation_table);
            if verbose {
                for (key, value) in evaluation_table.iter() {
                    print!("x{:?}: {:?}\t", key, value);
                }
                println!("\t-> {:?}", result);
            }
            return result;
        }
    }

    fn parse_variable(
        variable: &mut String,
        evaluation_table: &mut HashMap<u32, bool>,
        neg: &mut bool,
        expressions: &mut Vec<Box<Expression>>,
        pos: usize,
    ) {
        if variable.len() == 0 {
            return;
        }
        if let Ok(num) = variable.parse::<u32>() {
            evaluation_table.insert(num, false);
            let mut expr = Box::new(Expression::Value(num));
            // Negate if needed
            if *neg {
                expr = Box::new(Expression::Neg(expr));
                *neg = false;
            }
            expressions.push(expr);
            variable.clear();
        } else {
            panic!(
                "Could not parse: Variable {} is not a number at position {:?}",
                variable, pos
            );
        }
    }

    pub fn parse(
        str: &mut String,
        initial_length: usize,
        evaluation_table: &mut HashMap<u32, bool>,
    ) -> Box<Expression> {
        // Stacks
        let mut expressions: Vec<Box<Expression>> = Vec::new();
        let mut operators: Vec<Box<Operator>> = Vec::new();
        let mut neg = false;

        // For error indictaion, indices characters from one
        let mut pos = initial_length - str.len();

        let mut variable = String::new();

        // Parse till ')' or end of string
        while let Some(c) = str.pop() {
            pos += 1;
            match c {
                NEG => {
                    Self::parse_variable(
                        &mut variable,
                        evaluation_table,
                        &mut neg,
                        &mut expressions,
                        pos,
                    );
                    // There should be an operator or ')'
                    if operators.len() < expressions.len() {
                        panic!("Could not parse: Unexpected negation at position {:?}", pos);
                    }
                    neg = !neg;
                }
                AND => {
                    Self::parse_variable(
                        &mut variable,
                        evaluation_table,
                        &mut neg,
                        &mut expressions,
                        pos,
                    );
                    // There should be an expression
                    if operators.len() >= expressions.len() {
                        panic!(
                            "Could not parse: Unexpected {:?} operator at position {:?}",
                            c, pos
                        );
                    }
                    operators.push(Box::new(Operator::And));
                }
                OR => {
                    Self::parse_variable(
                        &mut variable,
                        evaluation_table,
                        &mut neg,
                        &mut expressions,
                        pos,
                    );
                    // There should be an expression
                    if operators.len() >= expressions.len() {
                        panic!(
                            "Could not parse: Unexpected {:?} operator at position {:?}",
                            c, pos
                        );
                    }
                    operators.push(Box::new(Operator::Or));
                }
                '(' => {
                    Self::parse_variable(
                        &mut variable,
                        evaluation_table,
                        &mut neg,
                        &mut expressions,
                        pos,
                    );
                    // There should be an operator or ')'
                    if operators.len() < expressions.len() {
                        panic!(
                            "Could not parse: Unexpected expression {:?} at position {:?}",
                            c, pos
                        );
                    }
                    let mut expr = Expression::parse(str, initial_length, evaluation_table);
                    // Negate if needed
                    if neg {
                        expr = Box::new(Expression::Neg(expr));
                        neg = false;
                    }
                    expressions.push(expr);
                }
                ')' => {
                    Self::parse_variable(
                        &mut variable,
                        evaluation_table,
                        &mut neg,
                        &mut expressions,
                        pos,
                    );
                    break;
                }
                c => {
                    // Ignore whitespace
                    if c.is_whitespace() {
                        Self::parse_variable(
                            &mut variable,
                            evaluation_table,
                            &mut neg,
                            &mut expressions,
                            pos,
                        );
                        continue;
                    }
                    if c.is_digit(10) {
                        // There should be an operator or ')'
                        if operators.len() < expressions.len() {
                            panic!(
                                "Could not parse: Unexpected expression {:?} at position {:?}",
                                c, pos
                            );
                        }
                        variable.push(c);
                    } else {
                        panic!(
                            "Could not parse: {:?} is not a digit at position {:?}",
                            c, pos
                        );
                    }
                }
            }
        }
        Self::parse_variable(
            &mut variable,
            evaluation_table,
            &mut neg,
            &mut expressions,
            pos,
        );
        let mut i = 0;
        // Formate AND operations
        while i < operators.len() {
            match operators[i].as_ref() {
                Operator::And => {
                    let a = expressions.remove(i);
                    let b = expressions.remove(i);
                    expressions.insert(i, Box::new(Expression::And(a, b)));
                    operators.remove(i);
                }
                Operator::Or => i = i + 1,
            }
        }
        i = 0;
        // Formate OR operations
        while i < operators.len() {
            match operators[i].as_ref() {
                Operator::And => i = i + 1,
                Operator::Or => {
                    let a = expressions.remove(i);
                    let b = expressions.remove(i);
                    expressions.insert(i, Box::new(Expression::Or(a, b)));
                    operators.remove(i);
                }
            }
        }
        if let Some(a) = expressions.pop() {
            return a;
        }
        panic!(
            "Could not parse: ')' not found at position {:?}",
            initial_length
        );
    }
}
