use std::{collections::HashMap, str::FromStr};

use clap::Parser;

/// Command arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file from AoC
    #[arg(short, long, default_value_t = String::from("inputs/day_9.txt"))]
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

fn finite_diff(input: impl Iterator<Item = i64> + Clone) -> Option<impl Iterator<Item = i64> + Clone> {
    if input.clone().all(|item| item == 0) {
        None
    } else {
        Some(input.clone().zip(input.skip(1)).map(|(a, b)| b - a))
    }
}

fn solve_part_1(input: &str) -> Option<i64> {
    let mut sum = 0;

    for line in input.lines() {
        let sequence = line
            .split_ascii_whitespace()
            .map(|token| token.parse::<i64>().ok())
            .collect::<Option<Vec<_>>>()?;

        let mut sequences = vec![sequence];

        loop {
            let last_sequence = sequences.last().unwrap().iter().copied();

            let Some(difference) = finite_diff(last_sequence) else {
                break;
            };

            sequences.push(difference.collect());
        }

        sum += sequences.into_iter().rev().try_fold(0, |sum, sequence| Some(sequence.last()? + sum))?;
    }

    Some(sum)
}

fn solve_part_2(input: &str) -> Option<i64> {
    let mut sum = 0;

    for line in input.lines() {
        let sequence = line
            .split_ascii_whitespace()
            .map(|token| token.parse::<i64>().ok())
            .collect::<Option<Vec<_>>>()?;

        let mut sequences = vec![sequence];

        loop {
            let last_sequence = sequences.last().unwrap().iter().copied();

            let Some(difference) = finite_diff(last_sequence) else {
                break;
            };

            sequences.push(difference.collect());
        }

        sum += sequences.into_iter().rev().try_fold(0, |sum, sequence| Some(sequence.first()? - sum))?;
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        const INPUT: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;
        const RESULT: Option<i64> = Some(114);

        assert_eq!(solve_part_1(INPUT), RESULT);
    }

    #[test]
    fn example_part_2() {
        const INPUT: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;
        const RESULT: Option<i64> = Some(2);

        assert_eq!(solve_part_2(INPUT), RESULT);
    }
}
