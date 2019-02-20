use crate::rule::*;
use crate::tools;
use std::collections::HashMap;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn run_for_rule<'a>(
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
    match tools::find_rule(rules, first_rule) {
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
                        if tools::last_write_time(resource) > own_last_execution {
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
