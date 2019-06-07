use clap::{App, Arg};
use hex;
use ssz_json::json_to_ssz;
use std::fs;
use std::process;

fn main() {
    let matches = App::new("ssz")
        .arg(
            Arg::with_name("filename")
                .help("the name of the input file")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("bytes")
                .short("b")
                .help("display result in a byte array"),
        )
        .get_matches();

    if let Some(filename) = matches.value_of("filename") {
        let data = fs::read(filename).unwrap_or_else(|err| {
            eprintln!("Unable to read file: {}", err);
            process::exit(1);
        });

        let ssz = json_to_ssz(data);

        if matches.is_present("bytes") {
            println!("{:?}", ssz);
        } else {
            println!("{}", hex::encode(ssz));
        }
    }
}
