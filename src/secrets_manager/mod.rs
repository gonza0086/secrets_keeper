use std::fs::File;
use std::str;

use cocoon::Cocoon;

pub struct SecretsKeeper {
    path: String,
    master_key: String,
}

impl SecretsKeeper {
    pub fn new(path: &str, master_key: &str) -> SecretsKeeper {
        SecretsKeeper {
            path: path.to_string(),
            master_key: master_key.to_string(),
        }
    }

    pub fn add(&self, app_name: String, password: &str) {
        let mut lines = self.read_file().into_iter();
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

    pub fn update(&self, app_name: String, password: &str) {
        let mut lines = self.read_file().into_iter();
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

    pub fn get(&self, app_name: String) -> String {
        let mut password: Option<String> = None;
        let mut lines = self.read_file().into_iter();
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

    pub fn delete(&self, app_name: String) {
        let mut lines = self.read_file().into_iter();
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

    pub fn list(&self) -> String {
        self.read_file().join("\n")
    }

    fn read_file(&self) -> Vec<String> {
        let cocoon = Cocoon::new(self.master_key.as_bytes());
        let lines = match File::open(&self.path) {
            Ok(mut file) => {
                let decrypted_file = cocoon.parse(&mut file).expect("Error decrypting file!");

                str::from_utf8(&decrypted_file)
                    .expect("Error converting data")
                    .lines()
                    .map(str::to_string)
                    .collect::<Vec<String>>()
            }
            Err(_) => {
                File::create(&self.path).expect("Error writting the file!");
                Vec::new()
            }
        };

        return lines;
    }

    fn write_file(&self, new_content: String) {
        let mut cocoon = Cocoon::new(self.master_key.as_bytes());
        let mut file = File::create(&self.path).expect("Error writting the file!");
        let _ = cocoon.dump(new_content.as_bytes().to_vec(), &mut file);
    }
}
