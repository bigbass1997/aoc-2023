use std::collections::HashMap;
use std::fmt::Display;

const INPUT: &'static str = include_str!("input.txt");
const _SAMPLE: &'static str = include_str!("sample.txt");

fn part1() -> impl Display {
    let mut lines = INPUT.lines();
    let dirs = lines.next().unwrap().chars();
    lines.next();
    
    let mut nodes = HashMap::with_capacity(800);
    for line in lines {
        let (name, (left, right)) = line.split_once(" = (").map(|(name, rhs)| (name, rhs.split_once(", ").map(|(left, right)| (left, right.trim_end_matches(')'))).unwrap())).unwrap();
        
        nodes.insert(name, (left, right));
    }
    
    let mut sequence = dirs.cycle();
    
    let mut steps = 0usize;
    let mut current = "AAA";
    while current != "ZZZ" {
        let (left, right) = nodes.get(current).unwrap();
        
        current = match sequence.next().unwrap() {
            'L' => *left,
            'R' => *right,
            _ => unreachable!()
        };
        steps += 1;
    }
    
    steps
}

fn part2() -> impl Display {
    let mut lines = INPUT.lines();
    let dirs = lines.next().unwrap().chars();
    lines.next();
    
    let mut nodes = HashMap::with_capacity(800);
    for line in lines {
        let (name, (left, right)) = line.split_once(" = (").map(|(name, rhs)| (name, rhs.split_once(", ").map(|(left, right)| (left, right.trim_end_matches(')'))).unwrap())).unwrap();
        
        nodes.insert(name, (left, right));
    }
    
    
    //let starts = nodes.keys().filter(|name| name.ends_with('A'));
    
    let mut currents: Vec<_> = nodes.keys().filter(|name| name.ends_with('A')).collect();
    let mut sequence = dirs.cycle();
    let mut steps = 0usize;
    println!("  starts: {currents:?}");
    while let Some(_) = currents.iter().filter(|cur| !cur.ends_with('Z')).next() {
        
        let next_dir = sequence.next().unwrap();
        for current in &mut currents {
            let (left, right) = nodes.get(*current).unwrap();
            
            *current = match next_dir {
                'L' => left,
                'R' => right,
                _ => unreachable!()
            };
            //print!(" {current}");
        }
        steps += 1;
        if steps % 10000000 == 0 {
            println!("currents: {currents:?} ({steps})");
        }
    }
    
    steps
}



fn main() {
    println!("{}", part1());
    println!("{}", part2());
}