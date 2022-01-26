use std::io;
use std::{i32};

fn check_prime(num1:i32) -> bool {
    if num1 == 0 || num1 == 1 {
        return false;
    }else{
        let mut i:i32 = 1;
        while i < num1{
            if num1 % i == 0{
                return true;
            }else {
                return false;
            }
            i+=1;
        }
    }
}

fn main() {
    println!("Insert a positive integer : ");
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1)
    .expect("Data should be a number");

    let a: i32 = num1.trim().parse().ok().expect("Data should be a number");

    let is_prime: bool = check_prime(a);

    println!("Prime {}", is_prime);
}