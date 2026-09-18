use std::io;

fn main() {
    let mut experience_input = String::new();
    let mut age_input = String::new();
    let is_experienced:bool;

    println!("Do you have experience? ");
    io::stdin().read_line(&mut experience_input).expect("Expected a string");
    let experience:String = experience_input.trim().to_string();

    println!("What is your age? ");
    io::stdin().read_line(&mut age_input).expect("Expected a string");
    let age:u8 = age_input.trim().parse().expect("Expected a positive number");

    if experience.to_lowercase() == "yes" {
        is_experienced = true;
    } else {
        is_experienced = false;
    }

    if age >= 40 &&  is_experienced{
        println!("The annual incentive is N1560000");
    } else if age <= 39 && age >= 29 && is_experienced {
        println!("The annual incentive is N14800000");
    } else if is_experienced && age > 28 {
        println!("The annual incentive is N1300000");
    } else {
        println!("The annual incentive is N100000");
    }
}