use std::cmp::Ordering;
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

fn part2() -> impl Display {
    INPUT.lines()
        .map(|line| {
            let char_first = line.find(|c: char| c.is_numeric()).unwrap_or(usize::MAX);
            let (text_first, text_val) = find_num_text(line).unwrap_or((usize::MAX, 0));
            let first = match char_first.cmp(&text_first) {
                Ordering::Equal => panic!(),
                Ordering::Less => line.chars().nth(char_first).unwrap(),
                Ordering::Greater => text_val.to_string().chars().next().unwrap(),
            };
            
            let char_last = line.rfind(|c: char| c.is_numeric()).map(|p| p as isize).unwrap_or(-1);
            let (text_last, text_val) = rfind_num_text(line).map(|(l, r)| (l as isize, r)).unwrap_or((-1, 0));
            let last = match char_last.cmp(&text_last) {
                Ordering::Equal => panic!(),
                Ordering::Greater => line.chars().nth(char_last as usize).unwrap(),
                Ordering::Less => text_val.to_string().chars().next().unwrap(),
            };
            
            format!("{first}{last}").parse::<usize>().unwrap()
        })
        .sum::<usize>()
}


fn find_num_text(text: &str) -> Option<(usize, usize)> {
    let mut posses = [None; 10];
    for (i, needle) in ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"].iter().enumerate() {
        if let Some(pos) = text.find(needle) {
            posses[i] = Some((pos, parse_num(needle)));
        }
    }
    
    posses.iter().filter(|p| p.is_some()).map(|p| p.unwrap()).min_by_key(|(lhs, _)| *lhs)
}

fn rfind_num_text(text: &str) -> Option<(usize, usize)> {
    let mut posses = [None; 10];
    for (i, needle) in ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"].iter().enumerate() {
        if let Some(pos) = text.rfind(needle) {
            posses[i] = Some((pos, parse_num(needle)));
        }
    }
    
    posses.iter().filter(|p| p.is_some()).map(|p| p.unwrap()).max_by_key(|(lhs, _)| *lhs)
}

fn parse_num(text: &str) -> usize {
    match text {
        "zero" => 0,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => unreachable!()
    }
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}