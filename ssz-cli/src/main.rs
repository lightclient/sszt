#[macro_use]
extern crate ssz_derive;

use clap::{App, Arg};
use hex;
use serde::Deserialize;
use serde_json::{Map, Number, Result, Value};
use ssz::Encode;
use std::collections::HashMap;
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

        // let ssz = binary_to_ssz(data);
        let ssz = Test {
            fixed: [0, 1, 2, 3, 4],
            variable: vec![5, 6],
            other: Other {
                a: 16,
                b: 32,
                c: 64,
                v: vec![7, 8, 9],
            },
        };

        let ssz = ssz.encode();

        if matches.is_present("bytes") {
            println!("{:?}", ssz);
        } else {
            println!("{}", hex::encode(ssz));
        }
    }
}

#[derive(Debug, Ssz)]
struct Other {
    a: u16,
    b: u32,
    c: u64,
    v: Vec<u8>,
}

#[derive(Debug, Ssz)]
struct Test {
    fixed: [u8; 5],
    variable: Vec<u8>,
    other: Other,
}

pub fn binary_to_ssz(data: Vec<u8>) -> Vec<u8> {
    data.encode()
}
