use std::io::Write;

fn main() {

    let lager = "Lager\n33 Export\nDesperados\nGoldberg\nGulder\nHeineken\nStar\n\n";
    let stout = "Stout\nLegend\nTurbo King\nWilliams\n\n";
    let non_alcoholic = "Non-Alcoholic\nMaltina\nAmstel Malta\nMalta Gold\nFayrouz\n";

    let mut file = std::fs::File::create("nigerian_breweries.txt").expect("create failed");
    file.write_all("Nigerian Breweries Plc - Categories of Drinks\n\n".as_bytes()).expect("write failed");
    file.write_all(lager.as_bytes()).expect("write failed");
    file.write_all(stout.as_bytes()).expect("write failed");
    file.write_all(non_alcoholic.as_bytes()).expect("write failed");

    println!("\nData written to file.");
}