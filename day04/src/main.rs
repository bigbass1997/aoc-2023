use std::fmt::Display;

const INPUT: &'static str = include_str!("input.txt");
const _SAMPLE: &'static str = include_str!("sample.txt");

fn part1() -> impl Display {
    parse_cards()
        .map(|matches| if matches > 0 {
            2usize.pow(matches as u32 - 1)
        } else {
            0
        })
        .sum::<usize>()
}

fn part2() -> impl Display {
    let matches = parse_cards().collect::<Vec<usize>>();
    
    let mut totals = vec![1; matches.len()];
    
    for i in 0..totals.len() {
        let to_add = totals[i];
        for j in (i + 1)..(i + matches[i] + 1) {
            totals[j] += to_add;
        }
    }
    
    totals.into_iter().sum::<usize>()
}

fn parse_cards() -> impl Iterator<Item = usize> {
    INPUT.lines()
        .map(|line| {
            let mut winning = vec![];
            let mut matched = 0usize;
            
            let mut seen_pipe = false;
            for part in line.split_whitespace().skip(2) {
                if part == "|" {
                    seen_pipe = true;
                    continue;
                }
                
                let num = part.parse::<usize>().unwrap();
                
                if !seen_pipe {
                    winning.push(num);
                } else if winning.contains(&num) {
                    matched += 1;
                }
            }
            
            matched
        })
}



fn main() {
    println!("{}", part1());
    println!("{}", part2());
}