use rand::random_range;

fn generate_random_char() -> String {
    let safe_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#%^&*-_=+";
    let random_char = safe_chars.chars().nth(random_range(0..76)).unwrap();

    return String::from(random_char);
}

fn generate_password() -> String {
    let mut password = String::new();
    for _ in 0..32 {
        let char = generate_random_char();
        password += &char;
    }

    return password;
}

fn main() {
    println!("Password: {}", generate_password());
}
