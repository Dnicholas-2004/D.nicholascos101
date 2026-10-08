use std::io;

fn main() {
    // 1. Show the menu
    println!("===== MENU =====");
    println!("P - Poundo Yam / Edinkaiko Soup - N3,200");
    println!("F - Fried Rice & Chicken - N3,000");
    println!("A - Amala & Ewedu Soup - N2,500");
    println!("E - Eba & Egusi Soup - N2,000");
    println!("W - White Rice & Stew - N2,500");
    println!("================");

    // 2. Ask for the food letter
    println!("Enter the food letter (P, F, A, E or W):");
    let mut letter = String::new();
    io::stdin().read_line(&mut letter).unwrap();
    let letter = letter.trim().to_uppercase();

    // 3. Ask for the quantity
    println!("Enter the quantity:");
    let mut qty_text = String::new();
    io::stdin().read_line(&mut qty_text).unwrap();
    let quantity: f64 = qty_text.trim().parse().unwrap();

    // 4. Decide the price based on the letter
    let price = match letter.as_str() {
        "P" => 3200.0,
        "F" => 3000.0,
        "A" => 2500.0,
        "E" => 2000.0,
        "W" => 2500.0,
        _ => 0.0, // any other letter
    };

    if price == 0.0 {
        println!("Invalid food letter!");
        return;
    }

    // 5. Calculate the total
    let mut total = price * quantity;
    println!("Total before discount: N{}", total);

    // 6. Give 5% discount if total is more than N10,000
    if total > 10000.0 {
        let discount = total * 0.05;
        total = total - discount;
        println!("You got a 5% discount of N{}", discount);
    }

    println!("Total to pay: N{}", total);
}