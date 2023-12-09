use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

const INPUT: &'static str = include_str!("input.txt");
const _SAMPLE: &'static str = include_str!("sample.txt");

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
enum Kind {
    FiveOf,
    FourOf,
    FullHouse,
    ThreeOf,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}
impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            'J' => Self::J,
            'T' => Self::T,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    bid: usize,
}
impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s
            .split_once(' ')
            .map(|(l, r)| (
                l.chars()
                    .map(|c| c.into())
                    .enumerate()
                    .fold([Card::Two; 5], |mut arr, (i, card)| { arr[i] = card; arr }),
                r.parse::<usize>().unwrap()
            )).unwrap();
        
        Ok(Self {
            cards,
            bid,
        })
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(match self.kind().cmp(&rhs.kind()) {
            Ordering::Equal => {
                for (l, r) in self.cards.iter().zip(rhs.cards.iter()) {
                    let ord = l.cmp(r);
                    if ord != Ordering::Equal {
                        return Some(ord);
                    }
                }
                
                Ordering::Equal
            },
            x => x
        })
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl Hand {
    pub fn kind(&self) -> Kind {
        let mut counts = HashMap::with_capacity(5);
        
        for card in self.cards {
            if counts.contains_key(&card) {
                *counts.get_mut(&card).unwrap() += 1;
            } else {
                counts.insert(card, 1usize);
            }
        }
        let mut counts: Vec<usize> = counts.values().copied().collect();
        counts.sort();
        counts.reverse();
        counts.resize(5, 0);
        
        match counts[..] {
            [5, _, _, _, _] => Kind::FiveOf,
            [4, _, _, _, _] => Kind::FourOf,
            [3, 2, _, _, _] => Kind::FullHouse,
            [3, 1, 1, _, _] => Kind::ThreeOf,
            [2, 2, 1, _, _] => Kind::TwoPair,
            [2, 1, 1, 1, _] => Kind::OnePair,
            _ => Kind::HighCard,
        }
    }
}


fn part1() -> impl Display {
    let mut hands: Vec<Hand> = INPUT
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    
    hands.sort();
    
    hands.into_iter().rev().enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum::<usize>()
}

fn part2() -> impl Display {
    
    0
}



fn main() {
    println!("{}", part1());
    println!("{}", part2());
}