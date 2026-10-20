use std::io;

// Helper: shows a message and reads a line of text
fn read_text(message: &str) -> String {
    println!("{}", message);
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("Failed to read input");
    text.trim().to_string()
}

fn main() {
    // The levels, from lowest to highest
    let levels = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];

    // One vector per job area, in the same order as the levels
    let office_admin = vec!["Intern", "Administrator", "Senior Administrator", "Office Manager", "Director", "CEO"];
    let academic = vec!["-", "Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean"];
    let lawyer = vec!["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"];
    let teacher = vec!["Placement", "Classroom Teacher", "Senior Teacher", "Leading Teacher", "Deputy Principal", "Principal"];

    // Show the menu
    println!("===== APS LEVEL CHECKER =====");
    println!("1 - Office Admin");
    println!("2 - Academic");
    println!("3 - Lawyer");
    println!("4 - Teacher");
    println!("=============================");

    let choice = read_text("Enter your job area (1-4):");

    // Pick the vector that matches the choice
    let titles = match choice.as_str() {
        "1" => &office_admin,
        "2" => &academic,
        "3" => &lawyer,
        "4" => &teacher,
        _ => {
            println!("Invalid choice!");
            return;
        }
    };

    let job = read_text("Enter your job title:");

    // Look for the title; its position gives the level
    let mut found = false;
    for i in 0..titles.len() {
        if titles[i].to_lowercase() == job.to_lowercase() {
            println!("{} holds position {}", titles[i], levels[i]);
            found = true;
        }
    }

    if !found {
        println!("Job title not found. Please check the spelling.");
    }
}