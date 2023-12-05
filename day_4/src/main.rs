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
}

impl FromStr for ScratchCard {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_ascii_whitespace();

        let Some("Card") = tokens.next() else {
            return Err("Missing 'Card' token");
        };

        Ok(ScratchCard {
            id: tokens
                .next()
                .ok_or("Missing ID")?
                .strip_suffix(":")
                .ok_or("Missing ':' token")?
                .parse()
                .map_err(|_| "Could not parse ID")?,
            winners: tokens
                .by_ref()
                .take_while(|&token| token != "|")
                .map(|token| token.parse())
                .collect::<Result<_, _>>()
                .map_err(|_| "Could not parse winners")?,
            scratched: tokens
                .map(|token| token.parse())
                .collect::<Result<_, _>>()
                .map_err(|_| "Could not parse scratched")?,
        })
    }
}

fn solve_part_1(input: &str) -> Option<u128> {
    input
        .lines()
        .filter_map(|line| line.parse::<ScratchCard>().ok())
        .map(|card| card.matches())
        .filter(|&matches| matches > 0)
        .map(|matches| 1 << (matches - 1))
        .sum::<u128>()
        .into()
}

fn solve_part_2(input: &str) -> Option<u128> {
    let (total, pending) = input
        .lines()
        .filter_map(|line| line.parse::<ScratchCard>().ok())
        .enumerate()
        .filter(|(index, card)| index + 1 == card.id)
        .map(|(_, card)| card.matches())
        .fold((0, VecDeque::new()), |(total, mut pending), matches| {
            let count = 1 + pending.pop_front().unwrap_or(0);

            let new = pending.len()..matches;

            pending.iter_mut().take(matches).for_each(|x| *x += count);

            pending.extend(new.map(|_| count));

            (total + count, pending)
        });

    pending.is_empty().then_some(total)
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
}
