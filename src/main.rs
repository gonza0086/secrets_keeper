use std::env;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};

mod password_generator;

use password_generator::PasswordGenerator;

struct SecretsKeeper {
    path: String,
}

impl SecretsKeeper {
    pub fn new(path: &str) -> SecretsKeeper {
        SecretsKeeper {
            path: path.to_string(),
        }
    }

    pub fn add(&self, app_name: String, password: &str) {
        let lines = self.read_file();

        let mut new_password_added = false;
        let mut new_content = String::new();

        for line in lines {
            new_content += &format!("{}\n", line);

            if line == app_name {
                new_content += &format!("{}\n", password);
                new_password_added = true;
            }
        }

        if !new_password_added {
            new_content += &format!("{}\n{}\n", app_name, password);
        }

        self.write_file(new_content);
    }

    fn read_file(&self) -> Vec<String> {
        let file = File::open(&self.path).expect("Error reading file!");
        let file_reader = BufReader::new(file);
        let lines = file_reader
            .lines()
            .collect::<std::io::Result<Vec<String>>>()
            .expect("Error parsing lines!");

        return lines;
    }

    fn write_file(&self, new_content: String) {
        fs::write(&self.path, new_content.as_bytes()).expect("Error writting file!");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let verb = &args[1];
    let app_name = &args[2];
    let secrets_keeper = SecretsKeeper::new("keeper.txt");

    match verb.to_string().as_str() {
        "--add" | "-a" => {
            let password_generator = PasswordGenerator::build();
            let password = password_generator.generate_password();
            secrets_keeper.add(app_name.to_string(), &password);
            println!("Password generated: {}", password);
        }
        _ => eprintln!("invalid verb!"),
    };
}
