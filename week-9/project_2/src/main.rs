use std::io;

fn read_text(message: &str) -> String {
    println!("{}", message);
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("Failed to read input");
    text.trim().to_string()
}

fn main() {
    let mut people: Vec<(String, u32)> = Vec::new();

    let count: u32 = read_text("How many developers are being interviewed?")
        .parse()
        .expect("Please enter a number");

    for i in 0..count {
        println!("--- Developer {} ---", i + 1);
        let name = read_text("Enter name:");
        let years: u32 = read_text("Enter years of programming experience:")
            .parse()
            .expect("Please enter a number");
        people.push((name, years));
    }

    if people.len() == 0 {
        println!("No developers entered.");
        return;
    }

    let mut best = 0;
    for i in 1..people.len() {
        if people[i].1 > people[best].1 {
            best = i;
        }
    }

    println!("The most experienced developer is {} with {} years.", people[best].0, people[best].1);
}