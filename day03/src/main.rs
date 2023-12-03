use std::cmp::{max, min};
use std::collections::HashSet;
use std::fmt::Display;

const INPUT: &'static str = include_str!("input.txt");
const _SAMPLE: &'static str = include_str!("sample.txt");

#[derive(Debug)]
struct Span {
    bits: Vec<char>,
    value: usize,
    pos: Vec<(usize, usize)>,
    symbols: HashSet<(char, usize, usize)>,
}
impl PartialEq for Span {
    fn eq(&self, other: &Self) -> bool {
        self.pos.get(0).is_some_and(|lhs| other.pos.get(0).is_some_and(|rhs| lhs == rhs))
    }
}

#[derive(Debug, Default)]
struct Grid {
    bits: Vec<Vec<char>>,
    width: usize,
    height: usize,
    /// spans touching at least one symbol
    spans: Vec<Span>,
}
impl Grid {
    pub fn new() -> Self {
        let mut grid = Grid::default();
        for line in INPUT.lines() {
            let mut bits = Vec::with_capacity(128);
            for c in line.chars() {
                bits.push(c);
            }
            grid.bits.push(bits);
        }
        
        grid.height = grid.bits.len();
        grid.width = grid.bits[0].len();
        
        for y in 0..grid.height {
            let mut in_span = false;
            for x in 0..grid.width {
                if grid.bit(x, y).unwrap().is_ascii_digit() {
                    if !in_span {
                        let span = grid.detect_symbol(x, y);
                        if !span.symbols.is_empty() {
                            grid.spans.push(span);
                        }
                        
                        in_span = true;
                    }
                } else {
                    in_span = false;
                }
            }
        }
        
        grid
    }
    
    pub fn detect_symbol(&self, x: usize, y: usize) -> Span {
        let mut cx = x as isize;
        let cy = y as isize;
        let w = self.width as isize;
        let h = self.height as isize;
        
        let mut span = Span {
            bits: Vec::with_capacity(4),
            value: 0,
            pos: Vec::with_capacity(4),
            symbols: HashSet::with_capacity(4),
        };
        loop {
            for y in (cy - 1)..=(cy + 1) {
                for x in (cx - 1)..=(cx + 1) {
                    if x >= 0 && x < w && y >= 0 && y < h {
                        let bit = self.bit(x as usize, y as usize).unwrap();
                        if x == cx && y == cy {
                            span.bits.push(bit);
                            span.pos.push((x as usize, y as usize));
                        } else if bit != '.' && !bit.is_ascii_digit() {
                            span.symbols.insert((bit, x as usize, y as usize));
                        }
                    }
                }
            }
            
            let next = self.bit(cx as usize + 1, cy as usize);
            if next.is_none() || !next.unwrap().is_ascii_digit() {
                break;
            }
            
            cx += 1;
        }
        
        span.value = String::from_iter(span.bits.iter()).parse::<usize>().unwrap();
        
        span
    }
    
    pub fn bit(&self, x: usize, y: usize) -> Option<char> {
        if let Some(row) = self.bits.get(y) {
            return row.get(x).copied();
        }
        
        None
    }
}

fn part1() -> impl Display {
    let grid = Grid::new();
    
    let mut sum = 0;
    
    for span in grid.spans {
        sum += span.value;
    }
    
    
    sum
}

fn part2() -> impl Display {
    let grid = Grid::new();
    
    let mut already_counted = HashSet::with_capacity(grid.spans.len());
    
    let mut sum = 0;
    for lhs_span in &grid.spans {
        for gear in lhs_span.symbols.iter().filter(|sym| sym.0 == '*') {
            for rhs_span in &grid.spans {
                if lhs_span != rhs_span && rhs_span.symbols.iter().find(|sym| gear == *sym).is_some() {
                    let lower = min(lhs_span.value, rhs_span.value);
                    let higher = max(lhs_span.value, rhs_span.value);
                    if !already_counted.contains(&(lower, higher)) {
                        sum += lower * higher;
                        already_counted.insert((lower, higher));
                    }
                }
            }
        }
    }
    
    sum
}



fn main() {
    println!("{}", part1());
    println!("{}", part2());
}