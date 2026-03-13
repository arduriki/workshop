use std::io;

pub fn prompt_ktype() -> String {
    let mut ktype = String::new();

    io::stdin()
        .read_line(&mut ktype)
        .expect("Failed to read line");

    ktype.trim().to_string()
    
    // TODO: Make a for loop to make sure that it's a correct Ktype, otherwise prompt again.
}
