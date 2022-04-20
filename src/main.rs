use std::{env::args, process::exit};

use mewl::mewlrun;

fn main() {
    if args().len() < 2 {
        eprintln!("Please provide a source file to execute");
        exit(1);
    } else {
        let file_name = args().nth(1);

        if let Some(..) = file_name {
            mewlrun(file_name.unwrap())
        } else {
            println!("[Err!] source file name is empty/invalid!");
            exit(1);
        }
    }
}
