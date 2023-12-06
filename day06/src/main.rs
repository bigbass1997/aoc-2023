use std::fmt::Display;

const INPUT: &'static str = include_str!("input.txt");
const _SAMPLE: &'static str = include_str!("sample.txt");

fn part1() -> impl Display {
    let (times, distances) = INPUT
        .split_once('\n')
        .map(|(lhs, rhs)| (
            lhs
                .trim_start_matches("Time:")
                .trim()
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>(),
            rhs
                .trim_start_matches("Distance:")
                .trim()
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        ))
        .unwrap();
    
    let mut ways = vec![0; times.len()];
    for race in 0..times.len() {
        for charge_time in 1..times[race] {
            let remaining = times[race] - charge_time;
            let distance = remaining * charge_time;
            
            if distance > distances[race] {
                ways[race] += 1;
            }
        }
    }
    
    ways.into_iter().fold(1, |acc, element| acc * element)
}

fn part2() -> impl Display {
    let (time, dist) = INPUT
        .split_once('\n')
        .map(|(lhs, rhs)| (
            lhs
                .trim_start_matches("Time:")
                .trim()
                .split_whitespace()
                .fold(String::new(), |acc, val| acc + val)
                .parse::<usize>().unwrap(),
            rhs
                .trim_start_matches("Distance:")
                .trim()
                .split_whitespace()
                .fold(String::new(), |acc, val| acc + val)
                .parse::<usize>().unwrap(),
        ))
        .unwrap();
    
    let mut ways = 0usize;
    for charge_time in 0..time {
        let remaining = time - charge_time;
        let distance = remaining * charge_time;
        
        if distance > dist {
            ways += 1;
        }
    }
    
    ways
}



fn main() {
    println!("{}", part1());
    println!("{}", part2());
}