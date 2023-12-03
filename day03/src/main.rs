use std::collections::HashSet;
use std::fmt::Display;

const INPUT: &'static str = include_str!("input.txt");
const _SAMPLE: &'static str = include_str!("sample.txt");

#[derive(Debug)]
struct Span {
    bits: Vec<char>,
    value: usize,
    pos: Vec<(isize, isize)>,
    symbols: HashSet<(char, isize, isize)>,
}
impl PartialEq for Span {
    fn eq(&self, other: &Self) -> bool {
        self.pos.get(0).is_some_and(|lhs| other.pos.get(0).is_some_and(|rhs| lhs == rhs))
    }
}

#[derive(Debug, Default)]
struct Grid {
    bits: Vec<Vec<char>>,
    width: isize,
    height: isize,
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
        
        grid.height = grid.bits.len() as isize;
        grid.width = grid.bits[0].len() as isize;
        
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
    
    /// Assumes (x, y) position is a number. So check for that before calling this function.
    pub fn detect_symbol(&self, x: isize, y: isize) -> Span {
        let mut cx = x;
        let cy = y;
        
        let mut span = Span {
            bits: Vec::with_capacity(4),
            value: 0,
            pos: Vec::with_capacity(4),
            symbols: HashSet::with_capacity(4),
        };
        loop {
            for y in (cy - 1)..=(cy + 1) {
                for x in (cx - 1)..=(cx + 1) {
                    if let Some(bit) = self.bit(x, y) { // bound check
                        if x == cx && y == cy {
                            span.bits.push(bit);
                            span.pos.push((x, y));
                        } else if bit != '.' && !bit.is_ascii_digit() {
                            span.symbols.insert((bit, x, y));
                        }
                    }
                }
            }
            
            let next = self.bit(cx + 1, cy);
            if next.is_none() || !next.unwrap().is_ascii_digit() {
                break;
            }
            
            cx += 1;
        }
        
        span.value = String::from_iter(span.bits.iter()).parse::<usize>().unwrap();
        
        span
    }
    
    pub fn bit(&self, x: isize, y: isize) -> Option<char> {
        if x >= 0 && y >= 0 {
            if let Some(row) = self.bits.get(y as usize) {
                return row.get(x as usize).copied();
            }
        }
        
        None
    }
}

fn part1() -> impl Display {
    let grid = Grid::new();
    
    grid.spans.into_iter()
        .map(|span| span.value)
        .sum::<usize>()
}

fn part2() -> impl Display {
    let grid = Grid::new();
    
    let mut sum = 0;
    for lhs_span in &grid.spans {
        for gear in lhs_span.symbols.iter().filter(|sym| sym.0 == '*') {
            for rhs_span in &grid.spans {
                if lhs_span != rhs_span && rhs_span.symbols.iter().find(|sym| gear == *sym).is_some() {
                    sum += lhs_span.value * rhs_span.value;
                }
            }
        }
    }
    
    // All gear ratios were counted twice, so we only need half of the result.
    // 
    // Duplicates could be checked pre-emptively (check previous commit), but doing so can only be
    //   attempted within the deepest part of the algorithm. Thus for this circumstance, checking
    //   overcomplicates the code for no meaningful benefit.
    // 
    // (Profiling would be needed to know which is ultimately faster)
    sum / 2
}



fn main() {
    println!("{}", part1());
    println!("{}", part2());
}