use rand::{thread_rng, Rng};
use std::io::stdin;

// lit un nombre depuis l'entrée standard
fn read_number() -> Result<u8, String> {
    println!("entrez un nombre entre 1 et 100");

    let mut num = String::new();

    match stdin().read_line(&mut num).map_err(|e| e.to_string())? {
        0 => Err("empty input".to_string()),
        _ => match num.trim().parse::<u8>() {
            Ok(x) => Ok(x),
            Err(e) => Err(e.to_string()),
        },
    }
}

// appelle read_number() en boucle tant que la valeur retournée n'est pas comprise entre 1 et 100
fn validate_input() -> u8 {
    loop {
        match read_number() {
            Ok(n) if n <= 100 => return n,
            _ => continue,
        }
    }
}

// generate a function that generate random number between 1 and 100
fn generate_random_number() -> u8 {
    thread_rng().gen_range(1..=100)
}

fn user_guess(number_to_guess: u8) {
    loop {
        let user_input = validate_input();

        if user_input == number_to_guess {
            break;
        } else if user_input < number_to_guess {
            println!("trop petit");
        } else {
            println!("trop grand");
        }
    }
}

fn main() {
    let user_number = generate_random_number();
    user_guess(user_number);

    println!("you found it!");
}
