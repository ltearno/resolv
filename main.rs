use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug)]
struct Rule<'a> {
    name: Option<&'a str>,
    dependencies: Option<Vec<&'a str>>,
    script: Option<Vec<&'a str>>,
}

#[derive(Debug)]
enum Dependency<'a> {
    Resource(&'a str),
    Rule(&'a str),
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

fn fetch_lines<'a>() -> Vec<String> {
    let f = File::open("Resolvfile").expect("did not find Resolvefile");
    let buf_reader = BufReader::new(&f);
    buf_reader
        .lines()
        .map(|line| line.expect("cannot read line"))
        .collect()
}

fn complete_rule<'a>(r: &mut Rule<'a>, line: &'a str) {
    if let None = r.name {
        r.name = Some(line);
    } else if let None = r.dependencies {
        r.dependencies = Some(line.split(" ").collect());
    } else if let None = r.script {
        r.script = Some(vec![line]);
    } else if let Some(lines) = &mut r.script {
        lines.push(line);
    }
}

fn main() {
    println!("Resolv v0.1, welcome\n");

    let lines = fetch_file();

    let rules = parse_rules(&lines);

    // let's try to execute the first rule
    let to_build: &str = match rules[0].name {
        None => "",
        Some(name) => name,
    };

    println!("building rule {}", to_build);

    let rule = find_rule(&rules, to_build).expect("not found building rule");
    println!("found {:?}", rule);

    let mut plan: Vec<&Rule> = Vec::new();
    build_plan(&rules, to_build, &mut plan);

    println!("executing {}", to_build);
    for rule in &plan {
        match rule.name {
            Some(name) => println!("((in rule {}))", name),
            None => println!("((in anonymous rule))"),
        }

        if let Some(script) = &rule.script {
            for command in script {
                println!("{}", command);
            }
        }
    }

    //println!("plan: {:?}", plan);

    println!("done");
}

fn build_plan<'a>(rules: &'a Vec<Rule<'a>>, first_rule: &str, plan: &mut Vec<&'a Rule<'a>>) {
    match find_rule(rules, first_rule) {
        None => {
            println!("[WARNING] skipping not found rule {}", first_rule);
        }

        Some(rule) => {
            if let Some(dependencies) = &rule.dependencies {
                for dependency in dependencies {
                    build_plan(rules, dependency, plan);
                }
            }

            plan.push(&rule);
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

fn fetch_file() -> Vec<String> {
    let mut lines = fetch_lines();
    lines.push(String::from(""));

    lines
}

fn push_and_prepare<'a>(rules: &mut Vec<Rule<'a>>, rule: Rule<'a>) -> Rule<'a> {
    rules.push(rule);
    Rule::new()
}

fn parse_rules<'a>(lines: &'a Vec<String>) -> Vec<Rule<'a>> {
    let mut rules: Vec<Rule> = Vec::new();

    let lines: Vec<&'a str> = lines
        .iter()
        .map(|line| line.as_str())
        .map(|line| line.trim())
        .collect();

    let mut current_rule = Rule::new();
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
                println!("processed rule {:?}", current_rule);
                current_rule = push_and_prepare(&mut rules, current_rule);

                state = State::Waiting;
            } else {
                complete_rule(&mut current_rule, line);
            }
        }
    }

    rules
}
