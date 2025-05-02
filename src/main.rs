use password_generator::PasswordGenerator;
use secrets_manager::SecretsKeeper;
use std::env;

mod password_generator;
mod secrets_manager;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        panic!("More options are required: --verb app_name");
    }

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
        "--update" | "-u" => {
            let password_generator = PasswordGenerator::build();
            let password = password_generator.generate_password();
            secrets_keeper.update(app_name.to_string(), &password);
            println!("Password updated: {}", password);
        }
        "--get" | "-g" => {
            let password = secrets_keeper.get(app_name.to_string());
            println!("Password: {}", password);
        }
        _ => eprintln!("invalid verb!"),
    };
}
