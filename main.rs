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
use std::result::Result;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
struct Rule<'a> {
    name: Option<&'a str>,
    dependencies: Option<Vec<&'a str>>,
    resources: Option<Vec<&'a str>>,
    script: Option<Vec<&'a str>>,
}

impl<'a> Rule<'a> {
    fn new() -> Rule<'a> {
        Rule {
            name: None,
            dependencies: None,
            resources: None,
            script: None,
        }
    }

    fn latest_run(&self, rules_state_store: &HashMap<&str, u64>) -> Option<u64> {
        match self.name {
            None => None,
            Some(name) => match rules_state_store.get(name) {
                None => None,
                Some(timestamp) => Some(*timestamp),
            },
        }
    }
}

fn last_write_time(path: &str) -> u64 {
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

enum State {
    Waiting,
    Completing,
}

fn fetch_lines(path: &str) -> Result<Vec<String>, String> {
    match File::open(path) {
        Ok(f) => Ok(BufReader::new(&f)
            .lines()
            .map(|line| line.expect("cannot read line"))
            .collect()),

        Err(e) => Result::Err(format!("did not find {}", path)),
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

fn run_for_rule<'a>(
    rules: &'a Vec<Rule<'a>>,
    to_build: &str,
    rules_state_store: &mut HashMap<&'a str, u64>,
) {
    let mut plan: Vec<&Rule> = Vec::new();

    println!("building execution plan");
    build_plan(rules, to_build, &mut plan, rules_state_store);

    println!("running execution plan");
    for rule in &plan {
        match rule.name {
            Some(name) => println!("((in rule {}))", name),
            None => println!("((in anonymous rule))"),
        }

        if let Some(resources) = &rule.resources {
            for path in resources {
                println!("((use resource {}))", path);
            }
        }

        if let Some(script) = &rule.script {
            for command in script {
                println!("{}", command);

                let mut cmd = Command::new("sh");
                cmd.arg("-c").arg(command);
                println!(
                    "-> {}",
                    String::from_utf8(cmd.output().expect("error running command").stdout)
                        .expect("invalid utf8")
                );
            }
        }

        if let Some(name) = rule.name {
            rules_state_store.insert(
                name,
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("error in time")
                    .as_secs(),
            );
        }
    }
}

fn build_plan<'a>(
    rules: &'a Vec<Rule<'a>>,
    first_rule: &str,
    plan: &mut Vec<&'a Rule<'a>>,
    rules_state_store: &HashMap<&'a str, u64>,
) -> bool {
    match find_rule(rules, first_rule) {
        None => {
            println!("[WARNING] skipping not found rule {}", first_rule);

            false
        }

        Some(rule) => {
            let own_last_execution = rule.latest_run(rules_state_store);
            let mut execute_script: bool = own_last_execution.is_none();

            if let Some(dependencies) = &rule.dependencies {
                for dependency in dependencies {
                    let rebuilded = build_plan(rules, dependency, plan, rules_state_store);

                    execute_script = execute_script || rebuilded;
                }
            }

            if let Some(own_last_execution) = own_last_execution {
                if let Some(resources) = &rule.resources {
                    for resource in resources {
                        if last_write_time(resource) > own_last_execution {
                            execute_script = true;
                        }
                    }
                }
            }

            if execute_script {
                plan.push(&rule);
            } else {
                println!("((skipping task {}, already complete))", first_rule)
            }

            execute_script
        }
    }
}

fn find_rule<'a, 'b>(rules: &'a Vec<Rule<'a>>, search: &'b str) -> Option<&'a Rule<'a>> {
    for rule in rules {
        if let Some(name) = rule.name {
            if name == search {
                return Some(rule);
            }
        }
    }

    None
}

fn fetch_file(path: &str) -> Result<Vec<String>, String> {
    let lines = fetch_lines(path);
    if lines.is_err() {
        return lines;
    }

    let mut lines = lines.unwrap();
    lines.push(String::from(""));
    Ok(lines)
}

fn push_and_prepare<'a>(rules: &mut Vec<Rule<'a>>, rule: Option<Rule<'a>>) -> Rule<'a> {
    if let Some(rule) = rule {
        rules.push(rule);
    }

    Rule::new()
}

fn parse_rules<'a>(lines: &'a Vec<String>) -> Vec<Rule<'a>> {
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
