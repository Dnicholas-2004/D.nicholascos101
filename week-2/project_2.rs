fn main() {
   
    let toshiba_qty: f64 = 2.0;
    let mac_qty: f64 = 1.0;
    let hp_qty: f64 = 3.0;
    let dell_qty: f64 = 3.0;
    let acer_qty: f64 = 1.0;

  
    let toshiba_price: f64 = 450_000.0;
    let mac_price: f64 = 1_500_000.0;
    let hp_price: f64 = 750_000.0;
    let dell_price: f64 = 2_850_000.0;
    let acer_price: f64 = 250_000.0;

   
    let toshiba_amount = toshiba_qty * toshiba_price;
    let mac_amount = mac_qty * mac_price;
    let hp_amount = hp_qty * hp_price;
    let dell_amount = dell_qty * dell_price;
    let acer_amount = acer_qty * acer_price;

    println!("Toshiba amount = {}", toshiba_amount);
    println!("Mac amount = {}", mac_amount);
    println!("HP amount = {}", hp_amount);
    println!("Dell amount = {}", dell_amount);
    println!("Acer amount = {}", acer_amount);

    let sum = toshiba_amount + mac_amount + hp_amount + dell_amount + acer_amount;
    println!("Total sales = {}", sum);

    let average = sum / 5.0;
    println!("Average sales = {}", average);
}