use std::fmt::Display;

const INPUT: &'static str = include_str!("input.txt");

fn part1() -> impl Display {
    INPUT.lines()
        .map(|line| {
            let first = line.chars().find(|c| c.is_numeric()).unwrap();
            let last = line.chars().rfind(|c| c.is_numeric()).unwrap();
            
            format!("{first}{last}").parse::<usize>().unwrap()
        })
        .sum::<usize>()
}

const PAT_REPLACE: [(&str, &str); 10] = [
    ("zero", "zero0zero"),
    ("one", "one1one"),
    ("two", "two2two"),
    ("three", "three3three"),
    ("four", "four4four"),
    ("five", "five5five"),
    ("six", "six6six"),
    ("seven", "seven7seven"),
    ("eight", "eight8eight"),
    ("nine", "nine9nine"),
];

fn part2() -> impl Display {
    INPUT.lines()
        .map(|line| {
            let mut line = line.to_string();
            
            for (pattern, replace) in PAT_REPLACE {
                line = line.replace(pattern, replace);
            }
            
            let first = line.chars().find(|c| c.is_numeric()).unwrap();
            let last = line.chars().rfind(|c| c.is_numeric()).unwrap();
            
            println!("{line} | {first}{last}");
            
            format!("{first}{last}").parse::<usize>().unwrap()
        })
        .sum::<usize>()
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}