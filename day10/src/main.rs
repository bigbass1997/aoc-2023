use std::fmt::Display;
use std::time::Duration;
use minifb::{Key, MouseButton, MouseMode, Scale, ScaleMode, Window, WindowOptions};

const INPUT: &'static str = include_str!("input.txt");
const _SAMPLE1: &'static str = include_str!("sample1.txt");

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Tile {
    Vert,
    Hori,
    NE,
    NW,
    SE,
    SW,
    Ground,
    Start,
}
impl From<char> for Tile {
    fn from(value: char) -> Self {
        use Tile::*;
        match value {
            '|' => Vert,
            '-' => Hori,
            'L' => NE,
            'J' => NW,
            'F' => SE,
            '7' => SW,
            '.' => Ground,
            'S' => Start,
            
            _ => unreachable!()
        }
    }
}
impl From<Tile> for char {
    fn from(value: Tile) -> Self {
        use Tile::*;
        match value {
            Vert => '║',
            Hori => '═',
            NE => '╚',
            NW => '╝',
            SE => '╔',
            SW => '╗',
            Ground => '░',
            Start => 'S',
        }
    }
}

#[derive(Debug, Clone)]
pub struct Grid {
    pub data: Vec<Vec<(Tile, Option<usize>)>>,
}
impl Grid {
    pub fn new() -> Self {
        let data: Vec<Vec<(Tile, Option<usize>)>> = INPUT.lines()
            .map(|line| line.chars().map(|c| (c.into(), None)).collect())
            .collect();
        
        Self {
            data,
        }
    }
    
    pub fn get(&self, x: usize, y: usize) -> Option<(Tile, Option<usize>)> {
        self.data.get(y).map(|row| row.get(x)).flatten().copied()
    }
    
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut (Tile, Option<usize>)> {
        self.data.get_mut(y).map(|row| row.get_mut(x)).flatten()
    }
    
    pub fn start(&self) -> (usize, usize) {
        for y in 0..self.data.len() {
            let row = &self.data[y];
            for x in 0..row.len() {
                if row[x].0 == Tile::Start {
                    return (x, y);
                }
            }
        }
        
        unreachable!()
    }
    
    fn possibilities(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        use Tile::*;
        match self.get(x, y).unwrap().0 {
            Vert => vec![(0, -1), (0, 1)],
            Hori => vec![(-1, 0), (1, 0)],
            NE => vec![(0, -1), (1, 0)],
            NW => vec![(0, -1), (-1, 0)],
            SW => vec![(0, 1), (-1, 0)],
            SE => vec![(0, 1), (1, 0)],
            Start => vec![(0, -1), (0, 1), (1, 0), (-1, 0)],
            Ground => vec![],
        }.into_iter()
            .filter_map(|(ox, oy)| {
                let x = (x as isize + ox) as usize;
                let y = (y as isize + oy) as usize;

                if self.get(x, y).is_some() {
                    Some((x, y))
                } else {
                    None
                }
            })
            .collect()
    }
    
    pub fn neighbors(&self, x: usize, y: usize, from: Option<(usize, usize)>) -> Vec<(usize, usize)> {
        /*match self.get(x, y).unwrap() {
            Vert => vec![(x, y - 1), (x, y + 1)],
            Hori => vec![(x - 1, y), (x + 1, y)],
            NE => vec![(x, y - 1), (x + 1, y)],
            NW => vec![(x, y - 1), (x - 1, y)],
            SW => vec![(x, y + 1), (x - 1, y)],
            SE => vec![(x, y + 1), (x + 1, y)],
            Start => vec![(x, y - 1), (x, y + 1), (x + 1, y), (x - 1, y)],
            Ground => vec![],
        }.into_iter()
            .filter(|(ox, oy)| (*ox, *oy) != (from_x, from_y) && self.get(*ox, *oy).is_some())
            .collect()
        */
        
        let self_possibles = self.possibilities(x, y);
        
        let mut neighbors = Vec::with_capacity(3);
        for possible in self_possibles {
            //println!("possible: {possible:?}");
            
            let other_possibles = self.possibilities(possible.0, possible.1);
            //println!("       found: {other_possibles:?}");
            if other_possibles.contains(&(x, y)) {
                //println!("       taken: {possible:?}");
                neighbors.push(possible);
            }
        }
        
        if let Some((from_x, from_y)) = from {
            for i in (0..neighbors.len()).rev() {
                if neighbors[i].0 == from_x && neighbors[i].1 == from_y {
                    neighbors.remove(i);
                }
            }
        }
        
        neighbors
        
        
        
        /*
        const OFFSETS: [(isize, isize, [Tile; 4]); 4] = [(0, -1, [Start, Vert, SW, SE]), (0, 1, [Start, Vert, NW, NE]), (1, 0, [Start, Hori, NW, SW]), (-1, 0, [Start, Hori, NE, SE])];
        
        let mut neighbors = Vec::with_capacity(3);
        for offset in OFFSETS {
            let x = (x as isize + offset.0) as usize;
            let y = (y as isize + offset.1) as usize;
            let valids = offset.2;
            
            if x != from_x && y != from_y {
                if let Some(tile) = self.get(x, y) {
                    println!("{x}, {y} = {tile:?}");
                    if valids.contains(&tile) {
                        neighbors.push((x, y));
                    }
                }
            }
        }
        
        neighbors*/
    }
}


fn part1() -> impl Display {
    let grid = Grid::new();
    let start = grid.start();
    
    println!("start: {start:?}");
    //println!("{:?}", grid.neighbors(start.0, start.1, None));
    
    let mut current = grid.neighbors(start.0, start.1, None).first().copied().unwrap();
    let mut last = start;
    
    let mut path = Vec::with_capacity(128);
    path.push(current);
    while !((current.0 == start.0) && (current.1 == start.1)) {
        let next = grid.neighbors(current.0, current.1, Some(last)).first().copied().unwrap();
        //println!("next: {next:?}");
        path.push(next);
        last = current;
        current = next;
    }
    
    println!("{path:?}");
    
    path.len() / 2
}

fn part2() -> impl Display {
    let mut grid = Grid::new();
    let start = grid.start();
    
    println!("start: {start:?}");
    //println!("{:?}", grid.neighbors(start.0, start.1, None));
    
    let mut current = grid.neighbors(start.0, start.1, None).first().copied().unwrap();
    let mut last = start;
    
    let mut path = Vec::with_capacity(128);
    path.push(current);
    while !((current.0 == start.0) && (current.1 == start.1)) {
        let next = grid.neighbors(current.0, current.1, Some(last)).first().copied().unwrap();
        //println!("next: {next:?}");
        path.push(next);
        last = current;
        current = next;
    }
    
    println!("path len: {}", path.len());
    
    for (x, y) in &path {
        grid.get_mut(*x, *y).unwrap().1 = Some(usize::MAX);
    }
    
    let mut id = 0;
    for y in 0..grid.data.len() {
        for x in 0..grid.data[0].len() {
            let cell = grid.get(x, y).unwrap();
            if cell.1.is_none() { // cell has not been checked
                flood_fill(&mut grid, &path, x, y, id);
                id += 1;
            }
        }
    }
    
    println!("ids: {id}");
    
    //println!("{:?}\n{:?}\n{:?}\n{:?}", grid.get(0, 0), grid.get(1, 1), grid.get(3, 3), grid.get(2, 6));
    
    let mut total = 0usize;
    for y in 0..grid.data.len() {
        for x in 0..grid.data[0].len() {
            let cell = grid.get(x, y).unwrap();
            let id = cell.1.unwrap();
            if id > 1 && id < (u32::MAX as usize - 1) {
                total += 1;
            }
        }
    }

    {
        let mut s = String::with_capacity(grid.data.len() * grid.data[0].len());
        let grid = grid.clone();
        for y in 0..grid.data.len() {
            for x in 0..grid.data[0].len() {
                let cell = grid.get(x, y).unwrap();
                if path.contains(&(x, y)) {
                    s.push(cell.0.into());
                } else {
                    if cell.1.unwrap() <= 1 {
                        s.push(' ');
                    } else {
                        s.push('░');
                    }
                }
            }
            s.push('\n');
        }
        std::fs::write("output.txt", s).unwrap();
    }
    
    
    let h = grid.data.len();
    let w = grid.data[0].len();
    let mut buffer = vec![0u32; w * h];
    let mut window = Window::new("", w, h, WindowOptions {
        borderless: false,
        title: true,
        resize: false,
        scale: Scale::X4,
        scale_mode: ScaleMode::Stretch,
        topmost: false,
        transparency: false,
        none: false,
    }).unwrap();
    window.limit_update_rate(Some(Duration::from_micros(33333)));
    
    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in 0..buffer.len() {
            let x = i % w;
            let y = i / h;
            
            let tile = grid.get(x, y).unwrap();
            let id = tile.1.unwrap();
            if id > u32::MAX as usize {
                buffer[i] = 0x00FFFFFF;
            } else {
                buffer[i] = (id as u32 * 1) + 0x0000AA00;
            }
            
            if tile.0 == Tile::Start {
                buffer[i] = 0x00FF0000;
            }
        }
        
        if window.get_mouse_down(MouseButton::Left) {
            if let Some((x, y)) = window.get_mouse_pos(MouseMode::Discard) {
                buffer[(y as usize * w) + x as usize] = 0x00FF0000;
                println!("({x}, {y}) {:?}", grid.get(x as usize, y as usize).unwrap());
            }
        }
        
        window.update_with_buffer(&buffer, w, h).unwrap();
    }
    
    
    total
}

fn flood_fill(grid: &mut Grid, path: &[(usize, usize)], x: usize, y: usize, id: usize) {
    if let Some(cell) = grid.get(x, y) {
        if cell.1.is_none() && !path.contains(&(x, y)) {
            let cell = grid.get_mut(x, y).unwrap();
            cell.1 = Some(id);
            
            flood_fill(grid, path, x, y - 1, id);
            flood_fill(grid, path, x, y + 1, id);
            flood_fill(grid, path, x + 1, y, id);
            flood_fill(grid, path, x - 1, y, id);
        }
    }
}



fn main() {
    println!("{}", part1());
    println!("{}", part2());
}