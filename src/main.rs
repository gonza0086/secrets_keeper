use password_generator::PasswordGenerator;
use secrets_manager::SecretsKeeper;
use std::env;

mod password_generator;
mod secrets_manager;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        panic!("More options are required: -k key --verb <OPTION>");
    }
    if &args[1] != "--key" && &args[1] != "-k" {
        panic!("Master key must be provided!");
    }
    if &args[3] != "--list" && &args[3] != "-l" && args.len() < 5 {
        panic!("App name is required: -k key --verb app_name");
    }

    let master_key = &args[2];
    let verb = &args[3];
    let secrets_keeper = SecretsKeeper::new("keeper.txt", master_key);

    match verb.to_string().as_str() {
        "--add" | "-a" => {
            let password_generator = PasswordGenerator::build();
            let password = password_generator.generate_password();
            let app_name = &args[4];
            secrets_keeper.add(app_name.to_string(), &password);
            println!("Password generated: {}", password);
        }
        "--update" | "-u" => {
            let password_generator = PasswordGenerator::build();
            let password = password_generator.generate_password();
            let app_name = &args[4];
            secrets_keeper.update(app_name.to_string(), &password);
            println!("Password updated: {}", password);
        }
        "--get" | "-g" => {
            let app_name = &args[4];
            let password = secrets_keeper.get(app_name.to_string());
            println!("Password: {}", password);
        }
        "--delete" | "-d" => {
            let app_name = &args[4];
            secrets_keeper.delete(app_name.to_string());
            println!("Passwords deleted");
        }
        "--list" | "-l" => {
            let passwords = secrets_keeper.list();
            println!("{}", passwords);
        }
        _ => eprintln!("invalid verb!"),
    };
}
