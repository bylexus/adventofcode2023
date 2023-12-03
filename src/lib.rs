use std::{
    fs,
    io::{BufRead, BufReader},
};

use regex::{Captures, Regex};

pub fn read_lines(file: &str) -> Vec<String> {
    let fh = fs::File::open(file).unwrap();
    let buffered_reader = BufReader::new(fh);
    buffered_reader.lines().map(|res| res.unwrap()).collect()
}

pub fn split_lines(lines: &Vec<String>, separator: &str) -> Vec<Vec<String>> {
    let lines: Vec<Vec<String>> = lines
        .iter()
        .filter(|l| l.trim().len() > 0)
        .map(|s| s.split(separator).map(|el| String::from(el)).collect())
        .collect();
    lines
}

pub fn split_groups<'a>(lines: &'a Vec<String>, regex: &'a Regex) -> Vec<Captures<'a>> {
    let mut res = Vec::new();
    for line in lines {
        res.push(match regex.captures(line) {
            Some(c) => c,
            None => continue,
        });
    }

    return res;
}

pub fn lines_to_numbers(lines: &Vec<String>) -> Vec<i64> {
    let lines: Vec<i64> = lines
        .iter()
        .map(|l| l.trim())
        .filter(|l| l.len() > 0)
        .map(|s| str::parse::<i64>(&s).unwrap())
        .collect();
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lines_to_numbers() {
        let input: Vec<String> = Vec::from([
            "2".to_string(),
            "12345678".to_string(),
            "    12345678".to_string(),
            "    87654321    ".to_string(),
            "42    ".to_string(),
        ]);
        assert_eq!(
            lines_to_numbers(&input),
            vec![2, 12345678, 12345678, 87654321, 42]
        );
    }
}
