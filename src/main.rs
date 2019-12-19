//! This code is a part of py2rust project.
//! Copyright (C) 2019. py2rust developers. Licensed under the MIT Licence.

extern crate clap;
extern crate colored;

use clap::{Arg, ArgMatches, App, SubCommand};

use colored::*;

use std::fs;
use std::path::Path;
use std::process::exit;

pub mod python;

fn main() {
    let app = App::new("py2rust");
    let matched = app
        .version("0.1")
        .arg(Arg::with_name("input")
            .short("v")
            .long("input")
            .index(1)
            .required(true)
            .takes_value(true)
            .help("Input file path"))
        .get_matches();

    let filename = matched.value_of("input").unwrap();

    if !Path::new(filename).exists() {
        println!("{}: '{}' file not found!", "error".red().bold(), filename);
        exit(1);
    }
    
    let raw = fs::read_to_string(filename).unwrap();

    print!("{}", raw);
}
