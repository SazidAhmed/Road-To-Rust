use std::io;
use std::{i32};

fn addition(num1:i32, num2:i32) -> i32 {
    return num1 + num2;
}

fn substitution(num1:i32, num2:i32) -> i32 {
    return num1 - num2;
}

fn multiplication(num1:i32, num2:i32) -> i32 {
    return num1 * num2;
}

fn division(num1:i32, num2:i32) -> i32 {
    return num1 / num2;
}
fn main() {
    println!("Insert first number : ");
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1)
    .expect("Data should be a number");

    println!("Insert second number : ");
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2)
    .expect("Data should be a number");

    let a: i32 = num1.trim().parse().ok().expect("Data should be a number");
    let b: i32 = num2.trim().parse().ok().expect("Data should be a number");

    let add: i32 = addition(a, b);
    let subs: i32 = substitution(a, b);
    let multi: i32 = multiplication(a, b);
    let divi: i32 = division(a, b);

    println!("Addition is {}", add);
    println!("Substitution is {}", subs);
    println!("Multiplication is {}", multi);
    println!("Division is {}", divi);
}