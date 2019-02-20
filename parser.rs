use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use crate::rule::*;

enum State {
    Waiting,
    Completing,
}

pub fn fetch_file(path: &str) -> Result<Vec<String>, String> {
    let lines = fetch_lines(path);
    if lines.is_err() {
        return lines;
    }

    let mut lines = lines.unwrap();
    lines.push(String::from(""));
    Ok(lines)
}

pub fn parse_rules<'a>(lines: &'a Vec<String>) -> Vec<Rule<'a>> {
    let mut rules: Vec<Rule> = Vec::new();

    let lines: Vec<&'a str> = lines
        .iter()
        .map(|line| line.as_str())
        .map(|line| line.trim())
        .collect();

    let mut current_rule: Option<Rule> = None;
    let mut state: State = State::Waiting;

    for line in &lines {
        if let State::Waiting = state {
            if !line.is_empty() {
                state = State::Completing;
            }
        }

        if let State::Completing = state {
            if line.starts_with("#") {
            } else if line.is_empty() {
                //println!("processed rule {:?}", current_rule);
                current_rule = Some(push_and_prepare(&mut rules, current_rule));

                state = State::Waiting;
            } else {
                if let None = current_rule {
                    current_rule = Some(Rule::new());
                }

                if let Some(rule) = &mut current_rule {
                    complete_rule(rule, line);
                }
            }
        }
    }

    rules
}

fn fetch_lines(path: &str) -> Result<Vec<String>, String> {
    match File::open(path) {
        Ok(f) => Ok(BufReader::new(&f)
            .lines()
            .map(|line| line.expect("cannot read line"))
            .collect()),

        Err(_) => Result::Err(format!("cannot open file '{}'", path)),
    }
}

fn complete_rule<'a>(r: &mut Rule<'a>, line: &'a str) {
    if let None = r.name {
        let parts: Vec<&str> = line.split(":").collect();
        match parts.len() {
            0 => {
                r.name = Some(line.trim());
            }

            1 => {
                r.name = Some(parts[0].trim());
            }

            2 => {
                r.name = Some(parts[0].trim());
                r.dependencies = Some(
                    parts[1]
                        .trim()
                        .split(" ")
                        .filter(|name| !name.contains("."))
                        .collect(),
                );
                r.resources = Some(
                    parts[1]
                        .trim()
                        .split(" ")
                        .filter(|name| name.contains("."))
                        .collect(),
                );
            }

            _ => {}
        }
    } else if let None = r.script {
        r.script = Some(vec![line]);
    } else if let Some(lines) = &mut r.script {
        lines.push(line);
    }
}

fn push_and_prepare<'a>(rules: &mut Vec<Rule<'a>>, rule: Option<Rule<'a>>) -> Rule<'a> {
    if let Some(rule) = rule {
        rules.push(rule);
    }

    Rule::new()
}
