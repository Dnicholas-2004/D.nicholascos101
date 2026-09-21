// Rust program to find the roots of a quadratic equation given a, b and c

use std::io;

fn main() {
    let mut input_a = String::new();
    let mut input_b = String::new();
    let mut input_c = String::new();

    println!("Enter a: ");
    io::stdin().read_line(&mut input_a).expect("Failed to read input");
    let a:f64 = input_a.trim().parse().expect("Not a valid number");

    println!("Enter b: ");
    io::stdin().read_line(&mut input_b).expect("Failed to read input");
    let b:f64 = input_b.trim().parse().expect("Not a valid number");

    println!("Enter c: ");
    io::stdin().read_line(&mut input_c).expect("Failed to read input");
    let c:f64 = input_c.trim().parse().expect("Not a valid number");

    let d:f64 = b*b - 4.0*a*c;

    if d > 0.0 {
        let root1:f64 = (-b + d.sqrt()) / (2.0*a);
        let root2:f64 = (-b - d.sqrt()) / (2.0*a);
        println!("Two distinct roots: {} and {}", root1, root2);
    } else if d == 0.0 {
        let root:f64 = -b / (2.0*a);
        println!("One real root: {}", root);
    } else {
        println!("No real roots");
    }
}