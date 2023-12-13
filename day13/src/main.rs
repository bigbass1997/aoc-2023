use std::fmt::Display;

const INPUT: &'static str = include_str!("input.txt");
const _SAMPLE: &'static str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

#[derive(Debug, Clone)]
struct Grid {
    cells: Vec<Vec<bool>>,
    w: usize,
    h: usize,
}
impl Grid {
    pub fn new(chunk: &str) -> Self {
        let raw: Vec<Vec<bool>> = chunk.lines()
            .map(|row| row
                .chars()
                .map(|c| c == '#')
                .collect())
            .collect();
        
        let h = raw.len();
        let w = raw[0].len();
        
        Self {
            cells: raw,
            w,
            h,
        }
    }
    
    pub fn check_vert_sym(&self) -> Option<usize> {
        let mut found_lines: Vec<usize> = (0..(self.w - 1)).collect();
        
        for y in 0..self.h {
            for i in (0..found_lines.len()).rev() {
                if !Self::sym(&self.cells[y], found_lines[i]) {
                    found_lines.remove(i);
                }
            }
        }
        
        if found_lines.len() > 1 {
            panic!("multiple lines found");
        } else {
            found_lines.get(0).map(|num| *num + 1)
        }
    }
    
    pub fn check_hori_sym(&self) -> Option<usize> {
        let mut found_lines: Vec<usize> = (0..(self.h - 1)).collect();
        
        for x in 0..self.w {
            let mut col = Vec::with_capacity(self.h);
            for y in 0..self.h {
                col.push(self.cells[y][x]);
            }
            
            for i in (0..found_lines.len()).rev() {
                if !Self::sym(&col, found_lines[i]) {
                    found_lines.remove(i);
                }
            }
        }
        
        if found_lines.len() > 1 {
            panic!("multiple lines found");
        } else {
            found_lines.get(0).map(|num| *num + 1)
        }
    }
    
    
    pub fn check_vert_sym_smudge(&self) -> Option<usize> {
        for y in 0..self.h {
            for x in 0..self.w {
                let mut grid = self.cells.clone();
                grid[y][x] = !grid[y][x];
                
                
                let mut found_lines_orig: Vec<usize> = (0..(self.w - 1)).collect();
                for y in 0..self.h {
                    for i in (0..found_lines_orig.len()).rev() {
                        if !Self::sym(&self.cells[y], found_lines_orig[i]) {
                            found_lines_orig.remove(i);
                        }
                    }
                }
                
                let mut found_lines_alt: Vec<usize> = (0..(self.w - 1)).collect();
                for y in 0..self.h {
                    for i in (0..found_lines_alt.len()).rev() {
                        if !Self::sym(&grid[y], found_lines_alt[i]) {
                            found_lines_alt.remove(i);
                        }
                    }
                }
                
                if found_lines_alt.is_empty() {
                    continue;
                }
                
                for alt in found_lines_alt {
                    if found_lines_orig.contains(&alt) {
                        continue;
                    } else {
                        return Some(alt + 1);
                    }
                }
            }
        }
        
        None
    }
    
    pub fn check_hori_sym_smudge(&self) -> Option<usize> {
        for y in 0..self.h {
            for x in 0..self.w {
                let mut grid = self.cells.clone();
                grid[y][x] = !grid[y][x];
                
                
                let mut found_lines_orig: Vec<usize> = (0..(self.h - 1)).collect();
                for x in 0..self.w {
                    let mut col = Vec::with_capacity(self.h);
                    for y in 0..self.h {
                        col.push(self.cells[y][x]);
                    }
                    
                    for i in (0..found_lines_orig.len()).rev() {
                        if !Self::sym(&col, found_lines_orig[i]) {
                            found_lines_orig.remove(i);
                        }
                    }
                }
                
                
                let mut found_lines_alt: Vec<usize> = (0..(self.h - 1)).collect();
                for x in 0..self.w {
                    let mut col = Vec::with_capacity(self.h);
                    for y in 0..self.h {
                        col.push(grid[y][x]);
                    }
                    
                    for i in (0..found_lines_alt.len()).rev() {
                        if !Self::sym(&col, found_lines_alt[i]) {
                            found_lines_alt.remove(i);
                        }
                    }
                }
                
                if found_lines_alt.is_empty() {
                    continue;
                }
                
                for alt in found_lines_alt {
                    if found_lines_orig.contains(&alt) {
                        continue;
                    } else {
                        return Some(alt + 1);
                    }
                }
            }
        }
        
        None
    }
    
    
    fn sym(data: &[bool], mut lhs_i: usize) -> bool {
        let mut rhs_i = lhs_i + 1;
        if rhs_i >= data.len() {
            unreachable!();
        }
        
        loop {
            let lhs = data.get(lhs_i);
            let rhs = data.get(rhs_i);
            
            match (lhs, rhs) {
                (Some(lhs), Some(rhs)) => if lhs != rhs { return false },
                (None,      Some(_)  ) => return true,
                (Some(_),   None     ) => return true,
                (None,      None     ) => return false,
            }
            
            lhs_i -= 1;
            rhs_i += 1;
        }
    }
}



fn parse_patterns() -> Vec<Grid> {
    INPUT.split("\n\n")
        .map(|chunk| Grid::new(chunk))
        .collect()
}

fn part1() -> impl Display {
    let pats = parse_patterns();
    
    let mut total = 0usize;
    for (i, pat) in pats.into_iter().enumerate() {
        if let Some(num) = pat.check_vert_sym() {
            total += num;
        } else if let Some(num) = pat.check_hori_sym() {
            total += 100 * num;
        } else {
            println!("warning: no sym found in pat {i}!");
        }
    }
    
    total
}

fn part2() -> impl Display {
    let pats = parse_patterns();
    
    let mut total = 0usize;
    for (i, pat) in pats.into_iter().enumerate() {
        if let Some(num) = pat.check_vert_sym_smudge() {
            total += num;
        } else if let Some(num) = pat.check_hori_sym_smudge() {
            total += 100 * num;
        } else {
            println!("warning: no sym found in pat {i}!");
        }
    }
    
    total
}



fn main() {
    println!("{}", part1());
    println!("{}", part2());
}