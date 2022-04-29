use std::path::Path;

use clap::{crate_authors, crate_version, Arg, Command};
use console::style;

use mewl::mewlrun;

fn main() {
    let mewl_args = Command::new("Mewl")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Mewl Programmming Language")
        .arg(
            Arg::new("script")
                .takes_value(false)
                .required(true)
                .help("Mewl script to run"),
        )
        .get_matches();

    if let Some(a) = mewl_args.value_of("script") {
        if Path::new(a).exists() {
            mewlrun(a.to_string())
        } else {
            eprintln!(
                "Err , script file `{}` is invalid or can not be found!",
                style(a).magenta().bold().for_stderr()
            );
        }
    }
}
