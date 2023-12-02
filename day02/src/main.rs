use std::cmp::max;
use std::collections::HashMap;
use std::fmt::Display;

const INPUT: &'static str = include_str!("input.txt");

type Set<'a> = Vec<(usize, &'a str)>;
type Game<'a> = Vec<Set<'a>>;

fn part1() -> impl Display {
    parse_maximums()
        .filter(|(_, maximums)| !(*maximums.get("red").unwrap() > 12 || *maximums.get("green").unwrap() > 13 || *maximums.get("blue").unwrap() > 14))
        .map(|(id, _)| id)
        .sum::<usize>()
}

fn part2() -> impl Display {
    parse_maximums()
        .map(|(_, maximums)| *maximums.get("red").unwrap() * *maximums.get("green").unwrap() * *maximums.get("blue").unwrap())
        .sum::<usize>()
}

fn parse_maximums<'a>() -> impl Iterator<Item = (usize, HashMap<&'a str, usize>)> {
    INPUT.lines()
        .map(|line| {
            let (gamenum, record) = line.split_once(':').unwrap();
            let game_id = gamenum.split_once(' ').unwrap().1.parse::<usize>().unwrap();
            
            let game = record.split(';')
                .map(|set| set
                    .split(',')
                    .map(|numcolor| numcolor.trim().split_once(' ').unwrap())
                    .map(|(num, color)| (num.parse::<usize>().unwrap(), color))
                    .collect::<Set>())
                .collect::<Game>();

            (game_id, game)
        })
        .map(|(id, game)| {
            let mut maximums = HashMap::new();
            
            for set in game {
                for (num, color) in set {
                    if !maximums.contains_key(color) {
                        maximums.insert(color, num);
                    } else {
                        let prev = maximums.get_mut(color).unwrap();
                        *prev = max(*prev, num);
                    }
                }
            }

            (id, maximums)
        })
}


fn main() {
    println!("{}", part1());
    println!("{}", part2());
}