```rust
use std::io;

fn get_input() -> f64 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().parse::<f64>().expect("Enter a valid number")
}

// 1. Trapezium area
fn trapezium_area(height: f64, base1: f64, base2: f64) -> f64 {
    height / 2.0 * (base1 + base2)
}

// 2. Rhombus area
fn rhombus_area(diagonal1: f64, diagonal2: f64) -> f64 {
    0.5 * diagonal1 * diagonal2
}

// 3. Parallelogram area
fn parallelogram_area(base: f64, altitude: f64) -> f64 {
    base * altitude
}

// 4. Cube surface area
fn cube_surface_area(side: f64) -> f64 {
    6.0 * side * side
}

// 5. Cylinder volume
fn cylinder_volume(radius: f64, height: f64) -> f64 {
    std::f64::consts::PI * radius * radius * height
}

fn main() {
    loop {
        println!("\n===== THE SHAPE CALCULATOR =====");
        println!("1. Trapezium Area");
        println!("2. Rhombus Area");
        println!("3. Parallelogram Area");
        println!("4. Cube Surface Area");
        println!("5. Cylinder Volume");
        println!("0. Exit");
        println!("Enter your choice:");

        let choice = get_input() as u32;

        match choice {
            1 => {
                println!("Enter height:");
                let height = get_input();
                println!("Enter first base:");
                let base1 = get_input();
                println!("Enter second base:");
                let base2 = get_input();

                println!(
                    "Trapezium area = {:.2}",
                    trapezium_area(height, base1, base2)
                );
            }

            2 => {
                println!("Enter first diagonal:");
                let d1 = get_input();
                println!("Enter second diagonal:");
                let d2 = get_input();

                println!("Rhombus area = {:.2}", rhombus_area(d1, d2));
            }

            3 => {
                println!("Enter base:");
                let base = get_input();
                println!("Enter altitude:");
                let altitude = get_input();

                println!(
                    "Parallelogram area = {:.2}",
                    parallelogram_area(base, altitude)
                );
            }

            4 => {
                println!("Enter cube side length:");
                let side = get_input();

                println!(
                    "Cube surface area = {:.2}",
                    cube_surface_area(side)
                );
            }

            5 => {
                println!("Enter cylinder radius:");
                let radius = get_input();
                println!("Enter cylinder height:");
                let height = get_input();

                println!(
                    "Cylinder volume = {:.2}",
                    cylinder_volume(radius, height)
                );
            }

            0 => {
                println!("Thank you for using the Shape Calculator!");
                break;
            }

            _ => println!("Invalid choice. Please select 0 to 5."),
        }
    }
}
```