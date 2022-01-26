use std::io;
use std::{i32};

fn main() {
    println!("Insert a number : ");
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1)
    .expect("Data should be a number");

    println!("The number is {}", num1);
    let a: i32 = num1.trim().parse().ok().expect("Data should be a number");
    println!("The number is {}", a);
}