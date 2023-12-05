use std::cmp::min;
use std::fmt::Display;

const INPUT: &'static str = include_str!("input.txt");
const _SAMPLE: &'static str = include_str!("sample.txt");

#[derive(Debug, Default, Clone, PartialEq)]
struct Map {
    converts: Vec<Convertion>,
}
impl Map {
    pub fn convert(&self, input: usize) -> usize {
        for convert in &self.converts {
            if convert.is_included(input) {
                return convert.dst + (input - convert.src);
            }
        }
        
        input
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Convertion {
    dst: usize,
    src: usize,
    range: usize,
}
impl Convertion {
    pub fn is_included(&self, input: usize) -> bool {
        (self.src..(self.src + self.range)).contains(&input)
    }
}

fn parse_seeds() -> Vec<usize> {
    INPUT.lines().next().unwrap()
        .trim_start_matches("seeds: ")
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect()
}

fn parse_maps() -> Vec<Map> {
    let mut maps = vec![];
    
    let mut map = Map::default();
    for line in INPUT.lines().skip(3) {
        if line.ends_with(':') {
            maps.push(map);
            map = Map::default();
            continue;
        } else if line.is_empty() {
            continue;
        }
        
        let mut nums = line.split_whitespace().map(|s| s.parse().unwrap());
        
        map.converts.push(Convertion {
            dst: nums.next().unwrap(),
            src: nums.next().unwrap(),
            range: nums.next().unwrap(),
        })
    }
    maps.push(map);
    
    maps
}

fn part1() -> impl Display {
    let maps = parse_maps();
    let seeds = parse_seeds();
    
    let mut minimum = usize::MAX;
    for seed in seeds {
        let mut output = seed;
        for map in &maps {
            output = map.convert(output);
        }
        minimum = min(output, minimum);
    }
    
    minimum
}

fn part2() -> impl Display {
    let maps = parse_maps();
    let mut minimum = usize::MAX;
    
    parse_seeds()
        .chunks_exact(2)
        .map(|chunk| chunk[0]..(chunk[0] + chunk[1]))
        .for_each(|range| range.for_each(|seed| {
            let mut output = seed;
            for map in &maps {
                output = map.convert(output);
            }
            minimum = min(output, minimum);
        }));
    
    minimum
}



fn main() {
    println!("{}", part1());
    println!("{}", part2());
}