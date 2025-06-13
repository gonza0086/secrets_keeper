use std::{env, fs, process::exit};
use zerbero::Zerbero;

mod password_generator;
mod zerbero;

fn main() {
    let args: Vec<String> = env::args().collect();

    if &args[1] == "--help" || &args[1] == "-h" {
        let path_buffer = env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf()
            .join("help.md");

        println!("{:?}", &path_buffer);
        let help_doc = fs::read_to_string(path_buffer).expect("Error reading help doc!");
        println!("{}", help_doc);
        exit(1);
    }
    if args.len() < 2 {
        eprintln!("More options are required: --verb <APP_NAME> [-o]");
        exit(1);
    }
    if &args[1] != "--list" && &args[1] != "-l" && args.len() < 3 {
        eprintln!("App name is required: --verb <APP_NAME> [-o]");
        exit(1);
    }

    let verb = &args[1];
    let app_name: Option<String> = {
        if verb != "-l" && verb != "--list" {
            Some(args[2].to_string())
        } else {
            None
        }
    };

    let config_password = {
        if args.len() == 4 && &args[3] == "-o" {
            true
        } else {
            false
        }
    };

    match Zerbero::new() {
        Ok(zerbero) => {
            if let Err(e) = zerbero.execute(verb, app_name, config_password) {
                match e {
                    cocoon::Error::Cryptography => println!("Wrong password!"),
                    _ => println!("Zerbero failed with error: {:?}", e),
                }
            }
        }
        Err(e) => println!("{}", e),
    }
}
