use std::io;

fn main() {
    println!("simple calculator.");
    println!("available operations :  +, -, *, /");
    println!("Enter your expression (e.g., 5 + 3)");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");

    let tokens: Vec<&str> = input.trim().split_whitespace().collect();

    if tokens.len() != 3 {
        println!("invalid input. please follow the format : numberoperator number");
        return;
    }

    let num1: f64 = match tokens[0].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("invalid first number");
            return;
        }
    };

    let operator = tokens[1];

    let num2: f64 = match tokens[2].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("invalid second number.");
            return;
        }
    };

    let result = match operator {
        "+" => add(num1, num2),
        "-" => subtract(num1, num2),
        "*" => multiply(num1, num2),
        "/" => divide(num1, num2),
        _ => {
            println!("invalid operator. Use + - * or /");
            return;
        } 
    };

    println!("result : {:.2}", result);
}

fn add(a: f64, b: f64) -> f64 {
    return a + b;
}

fn subtract(a: f64, b: f64) -> f64 {
    return a - b;
}

fn multiply(a: f64, b: f64) -> f64 {
    return a * b;
}

fn divide(a: f64, b: f64) -> f64 {
    // return a / b;
    if b == 0.0 {
        println!("division by zeor is not allowed");
        std::process::exit(1);
    }
    a / b
}