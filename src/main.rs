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

    let app_name: Option<String> = {
        if verb != "-l" && verb != "--list" {
            Some(args[4].to_string())
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

    let secrets_keeper = SecretsKeeper::new(path.to_str().unwrap(), master_key);

    match secrets_keeper.execute(verb, app_name) {
        Ok(_) => println!("Zerbero executed succesfully!"),
        Err(e) => match e {
            cocoon::Error::Cryptography => println!("Wrong password!"),
            _ => println!("Zerbero failed with error: {:?}", e),
        },
    }
}
