use clap::{App, Arg};
use ssz::Encode;
use std::fs;
use std::process;
use std::string::String;

fn main() {
    let matches = App::new("ssz-cli")
        .arg(
            Arg::with_name("filename")
                .help("the name of the input file")
                .index(1)
                .required(true),
        )
        .get_matches();

    if let Some(filename) = matches.value_of("filename") {
        let ssz = file_to_ssz(filename);
        println!("{:?}", ssz);
    }
}

pub fn file_to_ssz(filename: &str) -> Vec<u8> {
    let data = fs::read(filename).unwrap_or_else(|err| {
        eprintln!("Unable to read file: {}", err);
        process::exit(1);
    });

    data.encode()
}
