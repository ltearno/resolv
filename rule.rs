use std::collections::HashMap;

#[derive(Debug)]
pub struct Rule<'a> {
    pub name: Option<&'a str>,
    pub dependencies: Option<Vec<&'a str>>,
    pub resources: Option<Vec<&'a str>>,
    pub script: Option<Vec<&'a str>>,
}

impl<'a> Rule<'a> {
    pub fn new() -> Rule<'a> {
        Rule {
            name: None,
            dependencies: None,
            resources: None,
            script: None,
        }
    }

    pub fn latest_run(&self, rules_state_store: &HashMap<&str, u64>) -> Option<u64> {
        match self.name {
            None => None,
            Some(name) => match rules_state_store.get(name) {
                None => None,
                Some(timestamp) => Some(*timestamp),
            },
        }
    }
}
