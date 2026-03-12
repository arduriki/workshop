use reqwest::StatusCode;
use std::process::exit;

mod client;
mod prompt;

fn main() {
    dotenv::dotenv().ok();

    let auth_url = "https://auth.macsds.hgs.cloud/api/v1/auth/token";

    let username = match dotenv::var("HELLA_USER_ID") {
        Ok(u) => u,
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    };

    let password = match dotenv::var("HELLA_PASSWORD") {
        Ok(p) => p,
        Err(e) => {
            eprint!("{}", e);
            exit(1);
        }
    };

    // Get the user's token
    let token = match client::get_client_token(auth_url, username, password) {
        Ok(t) => t,
        Err(e) => {
            match e.status() {
                Some(status) => match status {
                    StatusCode::UNAUTHORIZED => eprintln!("Error: sin autorización"),
                    StatusCode::TOO_MANY_REQUESTS => {
                        eprintln!("Error: demasiadas peticiones al servidor")
                    }
                    _ => eprintln!("Error."),
                },
                None => eprintln!("Error de conexión"),
            }
            exit(1);
        }
    };

    // Welcome to the user.
    println!("¡Bienvenid@ a la documentación de Hella! 🚗");
    println!("Introduce un kType valido:");
    
    // Prompt for the car's kType
    let ktype = prompt::prompt_ktype();

    todo!("Implement wiring diagram's endpoint.");
}
