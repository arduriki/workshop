use std::io;

pub fn welcome() -> () {
    println!("¡Bienvenid@ a la documentación de Hella! 🚗");
    println!("Introduce un kType valido:");
}

pub fn prompt_ktype() -> String {
    let mut ktype = String::new();

    io::stdin()
        .read_line(&mut ktype)
        .expect("Failed to read line");

    return ktype.trim().to_string();
}
