use crate::password_generator::PasswordGenerator;
use cocoon::{Cocoon, Error};
use rpassword::read_password;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{env, io, str};

pub struct Zerbero {
    path: String,
    master_key: String,
}

impl Zerbero {
    pub fn new() -> Result<Zerbero, String> {
        let path_buffer = env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf()
            .join("zerbero.txt");

        let path = path_buffer.to_str().unwrap();
        let dir = Path::new(&path);

        print!("Enter master key: ");
        io::stdout().flush().unwrap();

        let key = read_password().expect("Failed to read master key!");

        if !dir.exists() {
            print!("\nRepeat master key: ");
            io::stdout().flush().unwrap();
            let second_key = read_password().expect("Failed to read master key!");

            if key != second_key {
                println!("\n------------------------");
                return Err("Passwords do not match!".to_string());
            }

            let mut cocoon = Cocoon::new(key.trim().as_bytes());
            let mut file = File::create(path).expect("Error writting the file!");
            let _ = cocoon.dump("".as_bytes().to_vec(), &mut file);
        }
        println!("\n------------------------");

        Ok(Zerbero {
            path: path.to_string(),
            master_key: key.trim().to_string(),
        })
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
                && splitted_apps.contains(&line.to_lowercase().as_str())
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

    fn decrypt_data(&self, mut file: File) -> Result<Vec<u8>, Error> {
        let cocoon = Cocoon::new(self.master_key.as_bytes());
        cocoon.parse(&mut file)
    }

    fn read_file(&self) -> Result<Vec<String>, Error> {
        let file = File::open(&self.path).expect("Error reading file!");
        let decrypted_file = self.decrypt_data(file)?;
        let lines = str::from_utf8(&decrypted_file)
            .expect("Error converting data")
            .lines()
            .map(str::to_string)
            .collect::<Vec<String>>();

        Ok(lines)
    }

    fn write_file(&self, new_content: String) {
        let mut cocoon = Cocoon::new(self.master_key.as_bytes());
        let mut file = File::create(&self.path).expect("Error writting the file!");
        let _ = cocoon.dump(new_content.as_bytes().to_vec(), &mut file);
    }

    pub fn execute(
        &self,
        verb: &str,
        app_name: Option<String>,
        config_password: bool,
    ) -> Result<(), Error> {
        let prev_content = self.read_file()?;

        match verb {
            "--add" | "-a" => {
                let password_generator = {
                    if config_password {
                        PasswordGenerator::config()
                    } else {
                        PasswordGenerator::build()
                    }
                };
                let password = password_generator.generate_password();
                self.add(app_name.unwrap(), &password, prev_content);
                println!("Password generated: {}", password);
            }
            "--update" | "-u" => {
                let password_generator = {
                    if config_password {
                        PasswordGenerator::config()
                    } else {
                        PasswordGenerator::build()
                    }
                };
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
