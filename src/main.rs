use std::io::{self, Write};

fn main() {

    print!("Enter value of Z score: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let z_score: f64 = match input.trim().parse() {
    Ok(v) => v,
    Err(_) => {
        eprintln!("Please enter a valid number (e.g 2.5, 1.5 etc)");
        return;
    }
    };
       
    if z_score > 2.0 {
        println!("Spread is high, z = {},: Sell X and buy Y.", z_score);
    } else if z_score < -2.0 {
        println!("Spread is low, z = {},: Sell Y and buy X.", z_score)
    } else {
        println!("Spread z = {} is within ±2; Hold.", z_score)
    }
}

