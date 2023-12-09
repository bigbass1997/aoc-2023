use std::fmt::Display;

const INPUT: &'static str = include_str!("input.txt");
const _SAMPLE: &'static str = include_str!("sample.txt");

fn parse_samples() -> Vec<Vec<isize>> {
    INPUT.lines()
        .map(|line| line.split(' ').map(|s| s.parse().unwrap()).collect())
        .collect()
}

#[inline(always)]
fn all_zeros(seq: &[isize]) -> bool {
    seq.iter().all(|num| *num == 0)
}

fn part1() -> impl Display {
    let sample_list = parse_samples();
    
    let mut total = 0isize;
    for init_seq in sample_list {
        let mut sequences = Vec::with_capacity(10);
        sequences.push(init_seq);
        
        while !all_zeros(sequences.last().unwrap()) {
            let mut seq = Vec::with_capacity(sequences.last().unwrap().len() - 1);
            
            for window in sequences.last().unwrap().windows(2) {
                seq.push(window[1] - window[0]);
            }
            
            sequences.push(seq);
        }
        
        sequences.last_mut().unwrap().push(0);
        
        for i in (0..(sequences.len() - 1)).rev() {
            let upper = *sequences[i].last().unwrap();
            let lower = *sequences[i + 1].last().unwrap();
            
            sequences[i].push(upper + lower);
        }
        
        total += *sequences[0].last().unwrap();
    }
    
    total
}

fn part2() -> impl Display {
    let sample_list = parse_samples();
    
    let mut total = 0isize;
    for init_seq in sample_list {
        let mut sequences = Vec::with_capacity(10);
        sequences.push(init_seq);
        
        while !all_zeros(sequences.last().unwrap()) {
            let mut seq = Vec::with_capacity(sequences.last().unwrap().len() - 1);
            
            for window in sequences.last().unwrap().windows(2) {
                seq.push(window[1] - window[0]);
            }
            
            sequences.push(seq);
        }
        
        sequences.last_mut().unwrap().insert(0, 0);
        
        for i in (0..(sequences.len() - 1)).rev() {
            let upper = *sequences[i].first().unwrap();
            let lower = *sequences[i + 1].first().unwrap();
            
            sequences[i].insert(0, upper - lower);
        }
        
        total += *sequences[0].first().unwrap();
    }
    
    total
}



fn main() {
    println!("{}", part1());
    println!("{}", part2());
}