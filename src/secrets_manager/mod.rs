use crate::password_generator::PasswordGenerator;
use cocoon::{Cocoon, Error};
use rpassword::read_password;
use std::fs::File;
use std::io::Write;
use std::{io, str};

pub struct SecretsKeeper {
    path: String,
    master_key: String,
}

impl SecretsKeeper {
    pub fn new(path: &str) -> SecretsKeeper {
        SecretsKeeper {
            path: path.to_string(),
            master_key: String::new(),
        }
    }

    fn add(&self, app_name: String, password: &str, prev_content: Vec<String>) {
        let mut lines = prev_content.into_iter();
        let mut new_password_added = false;
        let mut new_content = lines.next().unwrap_or_else(|| String::new());

        if lines.len() > 0 && !new_content.contains(&app_name) {
            new_content += &format!(",{}\n", app_name);
        } else if !new_content.contains(&app_name) {
            new_content += &format!("{}\n", app_name);
        } else {
            new_content += "\n";
        }

        while let Some(line) = lines.next() {
            new_content += &format!("{}\n", line);

            if line.to_lowercase() == app_name.to_lowercase() {
                new_content += &format!("{}\n", password);
                new_password_added = true;
            }
        }

        if !new_password_added {
            new_content += &format!("\n{}\n{}\n", app_name, password);
        }

        self.write_file(new_content);
    }

    fn update(&self, app_name: String, password: &str, prev_content: Vec<String>) {
        let mut lines = prev_content.into_iter();
        let mut password_updated = false;
        let mut new_content = String::new();

        while let Some(line) = lines.next() {
            new_content += &format!("{}\n", line);

            if line.to_lowercase() == app_name.to_lowercase() {
                new_content += &format!("{}\n", password);
                password_updated = true;
                lines.next();
            }
        }

        if !password_updated {
            new_content += &format!("{}\n{}\n", app_name, password);
        }

        self.write_file(new_content);
    }

    fn get(&self, app_name: String, prev_content: Vec<String>) -> String {
        let mut password: Option<String> = None;
        let mut lines = prev_content.into_iter();
        while let Some(line) = lines.next() {
            if line.to_lowercase() == app_name.to_lowercase() {
                password = Some(lines.next().unwrap());
            }
        }

        match password {
            Some(pass) => pass,
            None => String::from("No password!"),
        }
    }

    fn delete(&self, app_name: String, prev_content: Vec<String>) {
        let mut lines = prev_content.into_iter();
        let saved_apps = lines.next().expect("File must have passwords");
        let mut splitted_apps: Vec<&str> = saved_apps.split(",").collect();
        splitted_apps.retain(|app| *app != app_name);

        let saved_apps_string = &splitted_apps.join(",");
        let mut new_content = String::from(saved_apps_string) + "\n";
        let mut reading_selected_app = false;

        while let Some(line) = lines.next() {
            if line.to_lowercase() == app_name.to_lowercase() {
                reading_selected_app = true;
                new_content.pop();
            } else if reading_selected_app
                && line != "\n"
                && saved_apps_string.contains(&line.to_lowercase().as_str())
            {
                reading_selected_app = false;
            }

            if !reading_selected_app && line != "\n" {
                new_content += &format!("{}\n", line);
            } else if !reading_selected_app {
                new_content += &format!("{}", line);
            }
        }

        self.write_file(new_content);
    }

    fn list(&self, prev_content: Vec<String>) -> String {
        prev_content.join("\n")
    }

    fn decrypt_data(&mut self, mut file: File) -> Result<Vec<u8>, Error> {
        print!("Enter master key: ");
        io::stdout().flush().unwrap();

        let key = read_password().expect("Failed to read master key!");
        println!("\n------------------------");

        self.master_key = key.clone();
        let cocoon = Cocoon::new(key.as_bytes()).with_weak_kdf();
        cocoon.parse(&mut file)
    }

    fn read_file(&mut self) -> Result<Vec<String>, Error> {
        let lines = match File::open(&self.path) {
            Ok(file) => {
                let decrypted_file = self.decrypt_data(file)?;
                str::from_utf8(&decrypted_file)
                    .expect("Error converting data")
                    .lines()
                    .map(str::to_string)
                    .collect::<Vec<String>>()
            }
            Err(_) => {
                print!("Enter new master key: ");
                io::stdout().flush().unwrap();
                let first_key = read_password().expect("Failed to read master key!");

                print!("\nRepeat master key: ");
                io::stdout().flush().unwrap();
                let second_key = read_password().expect("Failed to read master key!");

                println!("\n------------------------");
                if first_key != second_key {
                    panic!("Passwords do not match!");
                }

                self.master_key = second_key;
                let mut cocoon = Cocoon::new(first_key.trim().as_bytes()).with_weak_kdf();
                let mut file = File::create(&self.path).expect("Error writting the file!");
                let _ = cocoon.dump("".as_bytes().to_vec(), &mut file);

                Vec::new()
            }
        };

        Ok(lines)
    }

    fn write_file(&self, new_content: String) {
        let mut cocoon = Cocoon::new(self.master_key.as_bytes()).with_weak_kdf();
        let mut file = File::create(&self.path).expect("Error writting the file!");
        let _ = cocoon.dump(new_content.as_bytes().to_vec(), &mut file);
    }

    pub fn execute(&mut self, verb: &str, app_name: Option<String>) -> Result<(), Error> {
        let prev_content = self.read_file()?;

        match verb {
            "--add" | "-a" => {
                let password_generator = PasswordGenerator::build();
                let password = password_generator.generate_password();
                self.add(app_name.unwrap(), &password, prev_content);
                println!("Password generated: {}", password);
            }
            "--update" | "-u" => {
                let password_generator = PasswordGenerator::build();
                let password = password_generator.generate_password();
                self.update(app_name.unwrap(), &password, prev_content);
                println!("Password updated: {}", password);
            }
            "--delete" | "-d" => {
                self.delete(app_name.unwrap(), prev_content);
                println!("Passwords deleted");
            }
            "--get" | "-g" => {
                let password = {
                    if prev_content.len() > 0 {
                        self.get(app_name.unwrap(), prev_content)
                    } else {
                        String::from("No password!")
                    }
                };
                println!("Password: {}", password);
            }
            "--list" | "-l" => {
                let passwords = {
                    if prev_content.len() > 0 {
                        self.list(prev_content)
                    } else {
                        String::new()
                    }
                };
                println!("{}", passwords);
            }
            _ => eprintln!("invalid verb!"),
        };

        Ok(())
    }
}
