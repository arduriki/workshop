use std::io;

pub fn prompt_ktype() -> String {
    let mut ktype = String::new();

    io::stdin()
        .read_line(&mut ktype)
        .expect("Failed to read line");

    ktype.trim().to_string()
}
