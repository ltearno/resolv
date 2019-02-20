extern crate getopts;
use getopts::Options;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

mod parser;
mod runner;
mod rule;
mod tools;

use parser::*;
use runner::*;

fn read_state_store() -> HashMap<String, u64> {
    let mut res: HashMap<String, u64> = HashMap::new();

    if let Ok(file) = File::open(".resolv") {
        BufReader::new(&file).lines().for_each(|line| {
            if let Ok(line) = line {
                let parts: Vec<&str> = line.split(":").collect();
                res.insert(
                    String::from(parts[0]),
                    parts[1].parse::<u64>().expect("bad"),
                );
            }
        });
    }

    res
}

fn store_state_store(rules_state_store: &HashMap<&str, u64>) {
    let file = File::create(".resolv").expect("error opening cache file");
    let mut writer = BufWriter::new(&file);
    for (name, value) in rules_state_store {
        match writer.write_all(format!("{}:{}\n", name, value).as_bytes()) {
            Ok(_) => (),
            Err(_) => (),
        }
    }
}

fn main() {
    println!("Resolv v0.1, welcome\n");

    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("f", "file", "set input file name", "NAME");
    opts.optflag("c", "clean", "clean state");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("c") {
        std::fs::remove_file(".resolv").expect(
            "cannot clean, there is already no state. You can ignore that, everything is fine.",
        );
    }

    let resolve_file_path = match matches.opt_str("f") {
        Some(path) => String::from(path),
        None => String::from("Resolvfile"),
    };

    let mut rules_state_store: HashMap<&str, u64> = HashMap::new();

    let state_store_from_settings = read_state_store();
    for (name, value) in &state_store_from_settings {
        rules_state_store.insert(name, *value);
    }

    let lines = fetch_file(&resolve_file_path);
    match lines {
        Ok(lines) => {
            let rules = parse_rules(&lines);

            let to_build: &str = match rules[0].name {
                None => "",
                Some(name) => name,
            };

            println!("run for target '{}'", to_build);
            run_for_rule(&rules, to_build, &mut rules_state_store);

            store_state_store(&rules_state_store);
        }
        Err(e) => println!("[ERROR] {}", e),
    }

    println!("done");
}