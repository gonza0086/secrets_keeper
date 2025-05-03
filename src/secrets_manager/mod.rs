use std::fs::File;
use std::str;

use cocoon::Cocoon;

pub struct SecretsKeeper {
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

            if line.to_lowercase() == app_name.to_lowercase() {
                new_content += &format!("{}\n", password);
                new_password_added = true;
            }
        }

        if !new_password_added {
            new_content += &format!("{}\n{}\n", app_name, password);
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

    fn read_file(&self) -> Vec<String> {
        let cocoon = Cocoon::new(b"master_key").with_weak_kdf();
        let mut file = File::open(&self.path).expect("Error reading file!");
        let decrypted_file = cocoon.parse(&mut file).expect("Error decrypting file!");

        let lines = str::from_utf8(&decrypted_file)
            .expect("Error converting data")
            .lines()
            .map(str::to_string)
            .collect::<Vec<String>>();

        return lines;
    }

    fn write_file(&self, new_content: String) {
        let mut cocoon = Cocoon::new(b"master_key").with_weak_kdf();
        let mut file = File::create(&self.path).expect("Error writting the file!");
        let _ = cocoon.dump(new_content.as_bytes().to_vec(), &mut file);
    }
}
