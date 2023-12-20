use std::collections::HashMap;

use super::Day;
use alex_lib::read_lines;
use regex::Regex;

#[derive(Debug)]
enum Operator {
    LT,
    GT,
    NONE,
}

#[derive(Debug)]
struct Rule {
    prop: String,
    op: Operator,
    value: i64,
    target: String,
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn process_part(&self, part: &Part) -> String {
        for rule in self.rules.iter() {
            let target = rule.target.as_str();
            if let Operator::NONE = rule.op {
                return target.to_string();
            }
            let part_val = match rule.prop.as_str() {
                "x" => part.x,
                "m" => part.m,
                "a" => part.a,
                "s" => part.s,
                _ => panic!("Unknown part property"),
            };
            match rule.op {
                Operator::LT => {
                    if part_val < rule.value {
                        return target.to_string();
                    }
                }
                Operator::GT => {
                    if part_val > rule.value {
                        return target.to_string();
                    }
                }
                _ => panic!("Unknown operator"),
            }
        }
        panic!("Oops! This should not happen!")
    }

    /// calc how many accepted / rejected  possibilities are there for this
    /// workflow (and all that follow)
    /// This is relatively simple:
    /// go through each rule, and calc the outcome of each rule, and sum them up
    /// per accepted/rejected.
    /// returns nr_of_accepted
    fn calc_max_acc(&self, workflows: &HashMap<String, Workflow>) -> i128 {
        let possibilities: i128 = 4000;
        let mut acc: i128 = 0;
        let mut last_rej: i128 = 1;
        for rule in self.rules.iter() {
            let rule_value = rule.value as i128;
            match rule.op {
                Operator::NONE => match rule.target.as_str() {
                    "A" => acc += last_rej,
                    "R" => {},
                    target => {
                        let target_wf = workflows.get(target).unwrap();
                        let a = target_wf.calc_max_acc(workflows);
                        acc += a * last_rej;
                    }
                },
                Operator::GT => {
                    let local_acc = possibilities - rule_value;
                    match rule.target.as_str() {
                        "A" => acc += local_acc * last_rej,
                        "R" => {},
                        target => {
                            let target_wf = workflows.get(target).unwrap();
                            let a = target_wf.calc_max_acc(workflows);
                            acc += a * local_acc * last_rej;
                        }
                    }
                    last_rej = possibilities - local_acc;
                }
                Operator::LT => {
                    let local_acc = rule_value - 1;
                    match rule.target.as_str() {
                        "A" => acc += local_acc * last_rej,
                        "R" => {},
                        target => {
                            let target_wf = workflows.get(target).unwrap();
                            let a = target_wf.calc_max_acc(workflows);
                            acc += a * local_acc * last_rej;
                        }
                    }
                    last_rej = possibilities - local_acc;
                }
            }
        }

        acc 
    }
}

#[derive(Debug)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

#[derive(Debug)]
pub struct Day19 {
    input: Vec<String>,
    workflows: HashMap<String, Workflow>,
    parts: Vec<Part>,
}

impl Day19 {
    pub fn new() -> Day19 {
        Day19 {
            input: Vec::new(),
            workflows: HashMap::new(),
            parts: Vec::new(),
        }
    }

    fn parse_input(&mut self) {
        let mut iter = self.input.iter();
        // Part 1: read workflows:
        // matches: px{a<2006:qkq,m>2090:A,rfg}
        let wf_matcher = Regex::new(r"(\w+)\{(.*)\}").unwrap();
        let r_matcher = Regex::new(r"([xmas])([<>])(\d+):(\w+)").unwrap();
        while let Some(line) = iter.next() {
            if line.is_empty() {
                break;
            }
            let captures = wf_matcher.captures(line).unwrap();
            let name = captures.get(1).unwrap().as_str();
            let rules = captures.get(2).unwrap().as_str().split(',');
            let mut prop: String;
            let mut op: Operator;
            let mut value: i64;
            let mut target: String;
            let mut rules_vec: Vec<Rule> = Vec::new();
            for r in rules {
                if let Some(r_captures) = r_matcher.captures(r) {
                    prop = r_captures.get(1).unwrap().as_str().to_string();
                    op = match r_captures.get(2).unwrap().as_str() {
                        "<" => Operator::LT,
                        ">" => Operator::GT,
                        _ => panic!("Unexpected operator"),
                    };
                    value = r_captures.get(3).unwrap().as_str().parse::<i64>().unwrap();
                    target = r_captures.get(4).unwrap().as_str().to_string();
                } else {
                    prop = "-".to_string();
                    op = Operator::NONE;
                    value = 0;
                    target = r.to_string();
                }
                rules_vec.push(Rule {
                    prop,
                    op,
                    value,
                    target,
                });
            }
            self.workflows.insert(
                name.to_string(),
                Workflow {
                    name: name.to_string(),
                    rules: rules_vec,
                },
            );
        }

        // Part 2: Read parts
        let parts_matcher = Regex::new(r"x=(\d+),m=(\d+),a=(\d+),s=(\d+)").unwrap();
        while let Some(line) = iter.next() {
            if let Some(groups) = parts_matcher.captures(line) {
                let part = Part {
                    x: groups.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                    m: groups.get(2).unwrap().as_str().parse::<i64>().unwrap(),
                    a: groups.get(3).unwrap().as_str().parse::<i64>().unwrap(),
                    s: groups.get(4).unwrap().as_str().parse::<i64>().unwrap(),
                };
                self.parts.push(part);
            }
        }

        // println!("Workflows: {:?}", self.workflows);
        // println!("Parts: {:?}", self.parts);
    }
}

impl Day for Day19 {
    fn day_nr(&self) -> String {
        String::from("19")
    }
    fn title(&self) -> String {
        String::from("Aplenty")
    }

    fn prepare(&mut self) {
        // let input = read_lines("data/day19.txt");
        let input = read_lines("data/day19-test.txt");
        self.input = input;
        self.parse_input();
    }

    fn solve1(&mut self) -> String {
        let mut solution: i64 = 0;
        let mut accepted_parts: Vec<&Part> = Vec::new();
        let mut rejected_parts: Vec<&Part> = Vec::new();
        let in_wf = self.workflows.get("in").unwrap();

        for part in self.parts.iter() {
            let mut act_wf = in_wf;
            loop {
                let next_wf_name = act_wf.process_part(part);
                if next_wf_name == "A" {
                    accepted_parts.push(part);
                    break;
                }
                if next_wf_name == "R" {
                    rejected_parts.push(part);
                    break;
                }
                act_wf = self.workflows.get(next_wf_name.as_str()).unwrap();
            }
        }

        // println!("Accepted parts: {:?}", accepted_parts);
        solution = accepted_parts.iter().map(|p| p.x + p.m + p.a + p.s).sum();
        // println!("Rejected parts: {:?}", rejected_parts);
        // println!("Solution: {}", solution);

        String::from(format!("{0}", solution))
    }

    fn solve2(&mut self) -> String {
        let mut solution: i128 = 0;

        // let start_wf = self.workflows.get("lnx").unwrap();
        let start_wf = self.workflows.get("in").unwrap();
        let acc = start_wf.calc_max_acc(&self.workflows);
        solution = acc;

        String::from(format!("{0}", solution))
    }
}
