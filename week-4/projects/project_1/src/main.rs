use std::io;

fn main() {
    let mut a_input = String::new();
    let mut b_input = String::new();
    let mut c_input = String::new();

    println!("Enter a");
    io::stdin().read_line(&mut a_input).expect("Expected a string!");
    let a:f64 = a_input.trim().parse().expect("Expected a number");

    println!("Enter b");
    io::stdin().read_line(&mut b_input).expect("Expected a string!");
    let b:f64 = b_input.trim().parse().expect("Expected a number");

    println!("Enter c");
    io::stdin().read_line(&mut c_input).expect("Expected a string!");
    let c:f64 = c_input.trim().parse().expect("Expected a number");

    let d:f64 = b.powf(2.0) - 4.0 * a * c;

    let pos_quadratic = (-b + d.sqrt()) / (2.0 * a);
    let neg_quadratic = (-b - d.sqrt()) / (2.0 * a);

    if d < 0.0 {
        println!("No real roots");
    } else if pos_quadratic == neg_quadratic {
        println!("The root is {}", pos_quadratic );
    } else {
        println!("The root are {} and {}", pos_quadratic, neg_quadratic);
    }
}
