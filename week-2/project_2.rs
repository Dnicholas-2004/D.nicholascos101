fn main(){
    let sales: [f64;5] = [450_000.0, 1_500_000.0, 750_000.0, 2_850_000.0, 250_000.0];
    let sum: f64 = sales.iter().sum();
    println!("total sales = {}", sum);
    let average: f64 = sum/5.0;
    println!("Average sales = {}", average)

}
