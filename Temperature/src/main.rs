use std::io;

fn main() {
    println!("temperature converter");
    println!("1. celsius to fahrenheit");
    println!("2. fahrenheit to celsius");
    print!("please select an option (1 or 2) : ");

    println!("");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("failed to read input");

    let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("invalid chouce. plase enter 1 or 2.");
            return;
        }
    };

    if choice == 1 {
        celsius_to_fahrenheit();
    } else if choice == 2 {
        fahrenheit_to_celsius();
    } else {
        println!("invalid choice. please select 1 or 2.");
    }

}

fn celsius_to_fahrenheit() {
    println!("enter temperature in celsius : ");

    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("failed to read input");

    let temp: f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("invaid input. please enter a valid number.");
            return;
        }
    };

    let fahrenheit = (temp * 9.0 / 5.0) + 32.0;
    println!("{:.2}C is {:.2}F", temp, fahrenheit);
}

fn fahrenheit_to_celsius() {
    println!("enter temperature in fahrenheit: ");

    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("failed to read input");

    let temp: f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("invalid input. please enter a valid number.");
            return;
        }
    };

    let celsius = (temp - 32.0) * 5.0 / 9.0;
    println!("{:.2}F is {:.2}C", temp, celsius);
}