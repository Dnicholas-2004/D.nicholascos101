struct Laptop {
    brand: String,
    price: u64,
}

impl Laptop {
    fn cost(&self, quantity: u64) -> u64 {
        self.price * quantity
    }
}

fn main() {
    let hp = Laptop { brand: String::from("HP"), price: 650000 };
    let ibm = Laptop { brand: String::from("IBM"), price: 755000 };
    let toshiba = Laptop { brand: String::from("Toshiba"), price: 550000 };
    let dell = Laptop { brand: String::from("Dell"), price: 850000 };

    let quantity = 3;

    println!("{} x{} = N{}", hp.brand, quantity, hp.cost(quantity));
    println!("{} x{} = N{}", ibm.brand, quantity, ibm.cost(quantity));
    println!("{} x{} = N{}", toshiba.brand, quantity, toshiba.cost(quantity));
    println!("{} x{} = N{}", dell.brand, quantity, dell.cost(quantity));


    let total = hp.cost(quantity) + ibm.cost(quantity) + toshiba.cost(quantity) + dell.cost(quantity);

    println!("Total cost = N{}", total);
}