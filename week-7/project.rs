use std::io;

// Helper: shows a message, reads a number and gives it back
fn read_number(message: &str) -> f64 {
    println!("{}", message);
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("Failed to read input");
    text.trim().parse().expect("Please enter a number")
}

// 1. Trapezium, area
fn trapezium() -> f64 {
    let height = read_number("Enter the height:");
    let base1 = read_number("Enter base1:");
    let base2 = read_number("Enter base2:");
    height / 2.0 * (base1 + base2)
}

// 2. Rhombus, area
fn rhombus() -> f64 {
    let diagonal1 = read_number("Enter diagonal1:");
    let diagonal2 = read_number("Enter diagonal2:");
    0.5 * diagonal1 * diagonal2
}

// 3. Parallelogram, area
fn parallelogram() -> f64 {
    let base = read_number("Enter the base:");
    let altitude = read_number("Enter the altitude:");
    base * altitude
}

// 4. Cube, surface area
fn cube() -> f64 {
    let side = read_number("Enter the side:");
    6.0 * side * side
}

// 5. Cylinder, volume
fn cylinder() -> f64 {
    let pi = 3.142;
    let radius = read_number("Enter the radius:");
    let height = read_number("Enter the height:");
    pi * radius * radius * height
}

fn main() {
    // Show the menu
    println!("===== SHAPE MENU =====");
    println!("1 - Trapezium (area)");
    println!("2 - Rhombus (area)");
    println!("3 - Parallelogram (area)");
    println!("4 - Cube (surface area)");
    println!("5 - Cylinder (volume)");
    println!("======================");

    // Read the choice
    println!("Enter your choice (1-5):");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let choice = choice.trim();

    // Call the function that matches the choice
    match choice {
        "1" => println!("Area of trapezium = {}", trapezium()),
        "2" => println!("Area of rhombus = {}", rhombus()),
        "3" => println!("Area of parallelogram = {}", parallelogram()),
        "4" => println!("Surface area of cube = {}", cube()),
        "5" => println!("Volume of cylinder = {}", cylinder()),
        _ => println!("Invalid choice!"),
    }
}