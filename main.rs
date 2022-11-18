use std::io;
use rand::Rng;
fn main() {
    println!("Guess the number");

    let mut input:String = String::new();
    io::stdin().read_line(&mut input).expect("failed to readline");

    let casted_input:i32 = input.trim().parse::<i32>().unwrap();

    let machine_number:i32 = rand::thread_rng().gen_range(0..11);

    if casted_input == machine_number {
        println!("You won");
    }
    else {
        println!("You lost");
    }
}
