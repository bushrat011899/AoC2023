use std::{collections::VecDeque, str::FromStr};

use clap::Parser;

/// Command arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file from AoC
    #[arg(short, long, default_value_t = String::from("inputs/day_4.txt"))]
    input: String,
}

fn main() {
    let args = Args::parse();

    let input = std::fs::read_to_string(args.input).expect("must be able to read input file");

    let result = solve_part_1(input.as_str());

    println!("Part 1: {:?}", result);

    let result = solve_part_2(input.as_str());

    println!("Part 2: {:?}", result);
}

struct ScratchCard {
    id: usize,
    winners: Vec<usize>,
    scratched: Vec<usize>,
}

impl ScratchCard {
    fn matches(&self) -> usize {
        self.scratched
            .iter()
            .filter(|number| self.winners.contains(number))
            .count()
    }

    fn score(&self) -> usize {
        let matches = self.matches();

        if matches > 0 {
            1 << (matches - 1)
        } else {
            0
        }
    }

    fn bonus_cards(&self) -> impl Iterator<Item = usize> {
        (self.id + 1)..(self.id + 1 + self.matches())
    }
}

impl FromStr for ScratchCard {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.strip_prefix("Card").ok_or(())?.trim().split(':');

        let id = split
            .next()
            .ok_or(())?
            .trim()
            .parse::<usize>()
            .map_err(|_| ())?;

        let rest = split.next().ok_or(())?.trim();

        if split.next().is_some() {
            return Err(());
        }

        let mut split = rest.split('|');

        let winners = split
            .next()
            .ok_or(())?
            .split_ascii_whitespace()
            .map(|num| num.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| ())?;

        let scratched = split
            .next()
            .ok_or(())?
            .split_ascii_whitespace()
            .map(|num| num.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| ())?;

        if split.next().is_some() {
            return Err(());
        }

        Ok(ScratchCard {
            id,
            winners,
            scratched,
        })
    }
}

fn solve_part_1(input: &str) -> Option<u128> {
    let sum = input
        .lines()
        .filter_map(|line| line.parse::<ScratchCard>().ok())
        .map(|card| card.score() as u128)
        .sum::<u128>();

    Some(sum)
}

fn solve_part_2(input: &str) -> Option<u128> {
    let cards = input
        .lines()
        .map(|line| line.parse::<ScratchCard>())
        .collect::<Result<Vec<_>, _>>()
        .ok()?;

    let mut stack = cards.iter().map(|card| card.id).collect::<VecDeque<_>>();
    let mut sum = 0;

    while !stack.is_empty() {
        let card_id = stack.pop_front().unwrap();
        sum += 1;

        let card = cards.iter().find(|card| card.id == card_id).unwrap();

        for card_id in card.bonus_cards() {
            stack.push_back(card_id);
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        const INPUT: &'static str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
        const RESULT: Option<u128> = Some(13);

        assert_eq!(solve_part_1(INPUT), RESULT);
    }

    #[test]
    fn example_part_2() {
        const INPUT: &'static str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
        const RESULT: Option<u128> = Some(30);

        assert_eq!(solve_part_2(INPUT), RESULT);
    }

    #[test]
    fn example_part_2_1() {
        const INPUT: &'static str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"#;

        let card = INPUT.parse::<ScratchCard>().unwrap();

        assert_eq!(card.id, 1);
        assert_eq!(card.matches(), 4);
        assert_eq!(card.bonus_cards().collect::<Vec<_>>(), vec![2, 3, 4, 5]);
    }

    #[test]
    fn example_part_2_2() {
        const INPUT: &'static str = r#"Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"#;

        let card = INPUT.parse::<ScratchCard>().unwrap();

        assert_eq!(card.id, 2);
        assert_eq!(card.matches(), 2);
        assert_eq!(card.bonus_cards().collect::<Vec<_>>(), vec![3, 4]);
    }
}
