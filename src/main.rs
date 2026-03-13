use reqwest::StatusCode;
use std::process::exit;

use crate::wiring_diagrams::{WiringDiagramsResponse, get_wiring_diagram_systems_response};

mod client;
mod prompt;
mod wiring_diagrams;

fn main() {
    const LANGUAGE: &str = "es";

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
    println!("\n¡Bienvenid@ a la documentación de Hella! 🚗\n");
    println!("Introduce un kType valido:");

    // Prompt for the car's kType
    let ktype = prompt::prompt_ktype();

    // Get Wiring Diagram Systems Response from its endpoint
    let gwdsr = match get_wiring_diagram_systems_response(&token, &ktype, LANGUAGE) {
        Ok(r) => r,
        Err(e) => {
            eprint!("{}", e);
            exit(1);
        }
    };

    // TODO: Select WD options
    wiring_diagrams::select_wiring_diagram_system(gwdsr);
}
