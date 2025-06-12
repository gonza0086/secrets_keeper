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

    match SecretsKeeper::new() {
        Ok(zerbero) => {
            if let Err(e) = zerbero.execute(verb, app_name) {
                match e {
                    cocoon::Error::Cryptography => println!("Wrong password!"),
                    _ => println!("Zerbero failed with error: {:?}", e),
                }
            }
        }
        Err(e) => println!("{}", e),
    }
}
