use std::fs::OpenOptions;
use std::io::Write;

use rand::random_range;

fn generate_random_char() -> String {
    let safe_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#%^&*-_=+";
    let random_char = safe_chars.chars().nth(random_range(0..72)).unwrap();

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

fn write_password_into_file(password: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("keeper.txt")?;

    writeln!(file, "{}", password)?;

    Ok(())
}

fn main() {
    let password = generate_password();
    if let Err(e) = write_password_into_file(&password) {
        eprintln!("Could not write file: {}", e);
    }
    println!("Password: {}", password);
}
