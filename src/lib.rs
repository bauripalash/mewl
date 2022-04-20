pub mod mewl;

use std::{fs::File, io::Read};

pub fn mewlrun(filename: String) {
    let mut source_file = match File::open(filename.as_str()) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("[Err!] The source file '{}' can not open/found!", filename);
            std::process::exit(1);
        }
    };

    let mut source_code = String::new();

    match source_file.read_to_string(&mut source_code) {
        Ok(_) => {}
        Err(_) => {
            eprintln!("[Err!] The source file cannot be read!");
            std::process::exit(1);
        }
    };

    let _ = mewl::parser::MewlParser::new(source_code).parse();
}
