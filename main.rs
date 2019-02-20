use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug)]
struct Rule<'a> {
    name: Option<&'a str>,
    dependencies: Option<&'a str>,
    script: Option<&'a str>,
}

impl<'a> Rule<'a> {
    fn new() -> Rule<'a> {
        Rule {
            name: None,
            dependencies: None,
            script: None,
        }
    }
}

enum State {
    Waiting,
    Completing,
}

fn fetch_lines() -> Vec<String> {
    let f = File::open("Resolvfile").expect("did not find Resolvefile");
    let buf_reader = BufReader::new(&f);
    let lines = buf_reader.lines();

    let mut result = Vec::new();
    for line in lines {
        result.push(line.unwrap());
    }

    result.push(String::from(""));

    result
}

fn main() {
    println!("Resolv v0.1, welcome\n");

    let mut rules: Vec<Rule> = Vec::new();

    let lines = fetch_lines();

    let mut current_rule = Rule::new();
    let mut state: State = State::Waiting;

    fn push_and_prepare<'a>(rules: &mut Vec<Rule<'a>>, rule: Rule<'a>) -> Rule<'a> {
        rules.push(rule);
        Rule::new()
    }

    for line in &lines {
        if let State::Waiting = state {
            if !line.is_empty() {
                state = State::Completing;
            }
        }

        if let State::Completing = state {
            if line.is_empty() {
                println!("processed rule {:?}", current_rule);
                current_rule = push_and_prepare(&mut rules, current_rule);

                state = State::Waiting;
            } else {
                complete_rule(&mut current_rule, line);
            }
        }
    }

    println!("rules: {:?}", rules);
}

fn complete_rule<'a>(r: &mut Rule<'a>, line: &'a str) {
    if let None = r.name {
        r.name = Some(line);
    } else if let None = r.dependencies {
        r.dependencies = Some(line);
    } else if let None = r.script {
        r.script = Some(line);
    }
}
