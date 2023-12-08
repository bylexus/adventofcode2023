use regex::{Captures, Regex};
use std::{
    fs,
    io::{BufRead, BufReader},
};

/// Re-export types
pub mod types;

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

/// Euclid's Greatest Common Divisor algorithm
pub fn euklid_gcd<T: num::traits::PrimInt>(a: T, b: T) -> T {
    
    if b.is_zero() {
        return a;
    } else {
        return euklid_gcd(b, a % b);
    }
}

/// Smallest Common Multiple
pub fn scm<T: num::traits::PrimInt>(a: T, b: T) -> T {
    a * b / euklid_gcd(a, b)
}