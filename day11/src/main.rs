use std::collections::HashMap;
use std::fmt::Display;

const INPUT: &'static str = include_str!("input.txt");
const _SAMPLE: &'static str = include_str!("sample.txt");

#[derive(Debug, Copy, Clone, PartialEq)]
struct Cell {
    galaxy_id: Option<usize>,
}
impl Cell {
    pub fn new(galaxy_id: Option<usize>) -> Self {
        Self {
            galaxy_id,
        }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    cells: Vec<Cell>,
    /// `Vec<(x, y, id)>`
    galaxies: Vec<(usize, usize, usize)>,
    w: usize,
    h: usize,
}
impl Grid {
    pub fn new() -> Self {
        let mut counter = 1usize;
        let mut raw: Vec<Vec<Cell>> = INPUT.lines()
            .map(|row| row
                .chars()
                .enumerate()
                .map(|(col, c)| if c == '.' { Cell::new(None) } else { let cell = Cell::new(Some(counter)); counter += 1; cell })
                .collect())
            .collect();
        
        for row in (0..raw.len()).rev() {
            let mut has_galaxy = false;
            for col in 0..raw[row].len() {
                if raw[row][col].galaxy_id.is_some() {
                    has_galaxy = true;
                    break;
                }
            }
            
            if !has_galaxy {
                raw.insert(row, raw[row].clone());
            }
        }
        
        for col in (0..raw[0].len()).rev() {
            let mut has_galaxy = false;
            for row in 0..raw.len() {
                if raw[row][col].galaxy_id.is_some() {
                    has_galaxy = true;
                    break;
                }
            }
            
            if !has_galaxy {
                for row in 0..raw.len() {
                    raw[row].insert(col, Cell::new(None));
                }
            }
        }
        
        let mut galaxies = Vec::with_capacity(128);
        for row in 0..raw.len() {
            for col in 0..raw[0].len() {
                if let Some(id) = raw[row][col].galaxy_id {
                    galaxies.push((col, row, id));
                }
            }
        }
        
        let h = raw.len();
        let w = raw[0].len();
        
        Self {
            cells: raw.into_iter().flatten().collect(),
            galaxies,
            w,
            h,
        }
    }
    
    pub fn get(&self, x: usize, y: usize) -> Option<Cell> {
        self.cells.get((y * self.w) + x).copied()
    }
}


fn part1() -> impl Display {
    let grid = Grid::new();
    
    let mut pairs = HashMap::with_capacity(256);
    for lhs in &grid.galaxies {
        for rhs in &grid.galaxies {
            if lhs.2 != rhs.2 && !pairs.contains_key(&(rhs.2, lhs.2)) && !pairs.contains_key(&(lhs.2, rhs.2)) {
                let dist = rhs.0.abs_diff(lhs.0) + rhs.1.abs_diff(lhs.1);
                pairs.insert((lhs.2, rhs.2), dist);
            }
        }
    }
    
    pairs.values().sum::<usize>()
}

fn part2() -> impl Display {
    
    0
}



fn main() {
    println!("{}", part1());
    println!("{}", part2());
}