// Define the Laptop structure
struct Laptop {
    brand: String,
    price: u32,
}

// Implement methods for the Laptop structure
impl Laptop {
    // Method to calculate the cost of purchasing laptops
    fn cost(&self, quantity: u32) -> u32 {
        self.price * quantity
    }
}

fn main() {
    // Create instances of the Laptop structure
    let hp = Laptop {
        brand: String::from("HP"),
        price: 650_000,
    };

    let ibm = Laptop {
        brand: String::from("IBM"),
        price: 755_000,
    };

    let toshiba = Laptop {
        brand: String::from("Toshiba"),
        price: 550_000,
    };

    let dell = Laptop {
        brand: String::from("Dell"),
        price: 850_000,
    };

    // Calculate the cost of purchasing 3 laptops from each brand
    let hp_cost = hp.cost(3);
    let ibm_cost = ibm.cost(3);
    let toshiba_cost = toshiba.cost(3);
    let dell_cost = dell.cost(3);

    // Calculate the total cost
    let total_cost = hp_cost + ibm_cost + toshiba_cost + dell_cost;

    // Display the cost for each brand
    println!("Brand: {}, Cost: ₦{}", hp.brand, hp_cost);
    println!("Brand: {}, Cost: ₦{}", ibm.brand, ibm_cost);
    println!("Brand: {}, Cost: ₦{}", toshiba.brand, toshiba_cost);
    println!("Brand: {}, Cost: ₦{}", dell.brand, dell_cost);

    // Display the total cost
    println!("\nTotal cost: ₦{}", total_cost);
