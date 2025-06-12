use secrets_manager::SecretsKeeper;
use std::env;

mod password_generator;
mod secrets_manager;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("More options are required: --verb <OPTION>");
    }
    if &args[1] != "--list" && &args[1] != "-l" && args.len() < 3 {
        panic!("App name is required: --verb app_name");
    }

    let verb = &args[1];
    let app_name: Option<String> = {
        if verb != "-l" && verb != "--list" {
            Some(args[2].to_string())
        } else {
            None
        }
    };

    let path = env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
        .join("keeper.txt");

    let mut secrets_keeper = SecretsKeeper::new(path.to_str().unwrap());
    if let Err(e) = secrets_keeper.execute(verb, app_name) {
        match e {
            cocoon::Error::Cryptography => println!("Wrong password!"),
            _ => println!("Zerbero failed with error: {:?}", e),
        }
    }
}
