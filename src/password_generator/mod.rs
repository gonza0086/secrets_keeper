use rand::random_range;
use std::io::{self, Write};

const LETTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const NUMBERS: &str = "0123456789";
const LETTERS_NUMBERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
const DEFAULT_CHARS: &str =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#%^&*-_=+!@%&*=+";

pub struct PasswordGenerator {
    valid_chars: String,
    length: u8,
}

impl PasswordGenerator {
    fn new(valid_chars: &str, length: u8) -> PasswordGenerator {
        PasswordGenerator {
            valid_chars: valid_chars.to_string(),
            length,
        }
    }

    pub fn build() -> PasswordGenerator {
        PasswordGenerator::new(
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#%^&*-_=+!@%&*=+",
            32,
        )
    }

    pub fn config() -> PasswordGenerator {
        print!("Enter password length: ");
        io::stdout().flush().unwrap();

        let mut password_length_input = String::new();
        io::stdin().read_line(&mut password_length_input).unwrap();
        let password_length = password_length_input
            .trim()
            .parse()
            .expect("Please enter a valid number");

        println!("Enter valid characters:");
        println!("1. Letters + Numbers + Special characters");
        println!("2. Letters + Numbers");
        println!("3. Only numbers");
        println!("4. Only letters");
        print!("Select the valid character: ");
        io::stdout().flush().unwrap();

        let mut valid_characters_input = String::new();
        io::stdin().read_line(&mut valid_characters_input).unwrap();
        let valid_characters: i32 = valid_characters_input
            .trim()
            .parse()
            .expect("Please enter a valid number");

        let characters = match valid_characters {
            1 => DEFAULT_CHARS,
            2 => LETTERS_NUMBERS,
            3 => NUMBERS,
            4 => LETTERS,
            _ => DEFAULT_CHARS,
        };

        PasswordGenerator::new(characters, password_length)
    }

    fn generate_random_char(&self) -> String {
        let random_char = self
            .valid_chars
            .chars()
            .nth(random_range(0..self.valid_chars.len() - 1))
            .unwrap();

        return String::from(random_char);
    }

    pub fn generate_password(&self) -> String {
        let mut password = String::new();
        for _ in 0..self.length {
            let char = self.generate_random_char();
            password += &char;
        }

        return password;
    }
}
