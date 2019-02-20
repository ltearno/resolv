use crate::rule::*;
use std::fs::File;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn find_rule<'a, 'b>(rules: &'a Vec<Rule<'a>>, search: &'b str) -> Option<&'a Rule<'a>> {
    for rule in rules {
        if let Some(name) = rule.name {
            if name == search {
                return Some(rule);
            }
        }
    }

    None
}

pub fn last_write_time(path: &str) -> u64 {
    match File::open(path) {
        Err(_) => 0,

        Ok(f) => {
            let res = f
                .metadata()
                .expect("cannot read metadata")
                .modified()
                .expect("cannot read modification time of a file");

            match res.duration_since(UNIX_EPOCH) {
                Ok(n) => n.as_secs(),
                Err(_) => panic!("SystemTime before UNIX EPOCH!"),
            }
        }
    }
}
