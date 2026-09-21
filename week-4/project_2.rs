// Rust program to calculate an employee's annual incentive

use std::io;

fn main() {
    let mut input_exp = String::new();
    let mut input_age = String::new();

    println!("Do you have experience? (yes/no): ");
    io::stdin().read_line(&mut input_exp).expect("Failed to read input");
    let experience = input_exp.trim();

    println!("Enter your age: ");
    io::stdin().read_line(&mut input_age).expect("Failed to read input");
    let age:i32 = input_age.trim().parse().expect("Not a valid number");

    let incentive: &str;

    if experience == "yes" {
        if age >= 40 {
            incentive = "1,560,000";
        } else if age >= 30 && age <= 39 {
            incentive = "1,480,000";
        } else if age < 28 {
            incentive = "1,300,000";
        } else {
            incentive = "Not specified (age 28-29 not covered by criteria)";
        }
    } else {
        incentive = "100,000";
    }

    println!("Annual incentive: N{}", incentive);
}