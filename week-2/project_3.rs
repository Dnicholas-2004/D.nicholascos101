fn main(){
    let p: f64 = 210_000.0;
    let r: f64 = 0.5;
    let n: f64 = 3.0;

    let a = p * (1.0 - (r/100.0)).powf(n);
    println!("Price depriciation after 3 years {}", a);
}