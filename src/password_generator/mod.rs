use rand::random_range;

pub struct PasswordGenerator {
    valid_chars: String,
    length: u8,
}

impl PasswordGenerator {
    pub fn new(valid_chars: &str, length: u8) -> PasswordGenerator {
        PasswordGenerator {
            valid_chars: valid_chars.to_string(),
            length,
        }
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
