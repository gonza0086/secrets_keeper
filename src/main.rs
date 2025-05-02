use std::env::args;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};

mod password_generator;

use password_generator::PasswordGenerator;

fn write_password_into_file(app_name: String, password: &str) -> std::io::Result<()> {
    let file = File::open("keeper.txt")?;

    let file_reader = BufReader::new(&file);
    let mut new_password_added = false;
    let mut new_content = String::new();

    for line in file_reader.lines() {
        let line = line?;
        new_content += &format!("{}\n", line);

        if line == app_name {
            new_content += &format!("{}\n", password);
            new_password_added = true;
        }
    }

    if !new_password_added {
        new_content += &format!("{}\n{}\n", app_name, password);
    }
    fs::write("keeper.txt", new_content.as_bytes())?;

    Ok(())
}

fn main() {
    let app_name = args()
        .into_iter()
        .nth(1)
        .expect("App name must be provided!");

    let password_generator = PasswordGenerator::new(
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#%^&*-_=+!@%&*=+",
        32,
    );

    let password = password_generator.generate_password();
    if let Err(e) = write_password_into_file(app_name, &password) {
        eprintln!("Could not write file: {}", e);
    }
}
