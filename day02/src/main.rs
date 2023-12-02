use std::cmp::max;
use std::fmt::Display;

const INPUT: &'static str = include_str!("input.txt");

type Set<'a> = Vec<(usize, &'a str)>;
type Game<'a> = Vec<Set<'a>>;

const R: usize = 0;
const G: usize = 1;
const B: usize = 2;

fn part1() -> impl Display {
    parse_maximums()
        .filter(|(_, maximums)| !(maximums[R] > 12 || maximums[G] > 13 || maximums[B] > 14))
        .map(|(id, _)| id)
        .sum::<usize>()
}

fn part2() -> impl Display {
    parse_maximums()
        .map(|(_, maximums)| maximums[R] * maximums[G] * maximums[B])
        .sum::<usize>()
}

fn parse_maximums() -> impl Iterator<Item = (usize, [usize; 3])> {
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
            let mut maximums = [0; 3];
            
            for set in game {
                for (num, color) in set {
                    let i = match color {
                        "red" => R,
                        "green" => G,
                        "blue" => B,
                        _ => unreachable!()
                    };
                    maximums[i] = max(maximums[i], num);
                }
            }

            (id, maximums)
        })
}


fn main() {
    println!("{}", part1());
    println!("{}", part2());
}