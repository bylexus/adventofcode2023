use super::Day;
use alex_lib::read_lines;

#[derive(Debug)]
struct Entry {
    springs: String,
    groups: Vec<u64>,
}
#[derive(Debug)]
pub struct Day12 {
    input: Vec<String>,
    data: Vec<Entry>,
}

impl Day12 {
    pub fn new() -> Day12 {
        Day12 {
            input: Vec::new(),
            data: Vec::new(),
        }
    }

    fn parse_input(&mut self) {
        for line in &self.input {
            let parts: Vec<&str> = line.split(' ').collect();
            let springs = parts[0];
            let groups = parts[1].split(',').map(|s| s.parse::<u64>().unwrap());
            self.data.push(Entry {
                springs: springs.to_string(),
                groups: groups.collect(),
            });
        }
    }
    fn permute_line(&self, line: &str) -> Vec<String> {
        let mut permutations: Vec<String> = Vec::new();
        let first_unknown = line.chars().position(|c| c == '?');
        match first_unknown {
            None => {
                permutations.push(line.to_string());
            }
            Some(idx) => {
                let prefix = line[..idx].to_string();
                let rest = line[idx + 1..].to_string();
                let sub_permutations = self.permute_line(&rest);
                for sub in sub_permutations {
                    let mut res_str1 = prefix.clone();
                    res_str1.push('.');
                    res_str1.push_str(&sub);
                    permutations.push(res_str1);

                    let mut res_str2 = prefix.clone();
                    res_str2.push('#');
                    res_str2.push_str(&sub);
                    permutations.push(res_str2);
                }
            }
        }

        permutations
    }

    fn count_group_match(&self, springs: &Vec<String>, groups: &Vec<u64>) -> u64 {
        let mut count: u64 = 0;
        for spring in springs.iter() {
            let parts: Vec<&str> = spring.split('.').filter(|s| !s.is_empty()).collect();
            if parts.len() != groups.len() {
                continue;
            }
            let mut parts_match = true;
            for (i, group) in groups.iter().enumerate() {
                if parts[i].len() as u64 != *group {
                    parts_match = false;
                    break;
                }
            }
            if parts_match {
                count += 1;
            }
        }

        count
    }
}

impl Day for Day12 {
    fn day_nr(&self) -> String {
        String::from("12")
    }
    fn title(&self) -> String {
        String::from("Hot Springs")
    }

    fn prepare(&mut self) {
        // let input = read_lines("data/day12.txt");
        let input = read_lines("data/day12-test.txt");
        self.input = input
            .iter()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        self.parse_input();
    }

    fn solve1(&mut self) -> String {
        let mut solution: u64 = 0;
        let mut solution2: u64 = 0;
        // println!("{:?}", self.data);

        for entry in &self.data {
            println!("springs: {:?}", entry.springs);
            let permutations = self.permute_line(&entry.springs);
            // println!("perms  : {:?}", permutations);
            let match_count = self.count_group_match(&permutations, &entry.groups);
            println!(
                "match  : {:?}, groups: {:?}",
                match_count,
                entry.groups.len()
            );
            println!(
                "match 5 times  : {:?}",
                match_count.pow(match_count.try_into().unwrap())
            );
            solution += match_count;
            solution2 += (match_count.pow(5));
        }
        println!("Solution 2: {:?}", solution2);
        String::from(format!("{0}", solution))
    }

    fn solve2(&mut self) -> String {
        let mut solution: u64 = 0;
        String::from(format!("{0}", solution))
    }
}
