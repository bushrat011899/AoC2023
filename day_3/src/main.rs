use std::{collections::HashMap, convert::Infallible, ops::RangeInclusive, str::FromStr};

use clap::Parser;

/// Command arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file from AoC
    #[arg(short, long, default_value_t = String::from("inputs/day_3.txt"))]
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

#[derive(Default, Debug, Clone, Copy)]
enum Entry {
    Symbol(char),
    Value(u8),
    #[default]
    Blank,
}

impl From<char> for Entry {
    fn from(value: char) -> Self {
        if let Some(digit) = value.to_digit(10) {
            Self::Value(digit as u8)
        } else if value == '.' {
            Self::Blank
        } else {
            Self::Symbol(value)
        }
    }
}

#[derive(Default, Debug)]
struct Schematic {
    parts: HashMap<(RangeInclusive<usize>, usize), usize>,
    symbols: HashMap<(usize, usize), char>,
}

impl FromStr for Schematic {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut schematic = Schematic::default();
        let mut active_part = None;

        for ((x, y), char) in s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| line.char_indices().map(move |(x, char)| ((x, y), char)))
        {
            active_part = match (Entry::from(char), active_part.take()) {
                (Entry::Value(digit), None) => Some(((x..=x, y), digit as usize)),
                (Entry::Value(digit), Some(((range, y_old), value))) if y_old == y => {
                    Some(((*range.start()..=x, y), 10 * value + digit as usize))
                }
                (Entry::Value(digit), Some(((range, y_old), value))) => {
                    schematic.parts.insert((range, y_old), value);
                    Some(((x..=x, y), digit as usize))
                }
                (Entry::Blank, None) => None,
                (Entry::Blank, Some((key, value))) => {
                    schematic.parts.insert(key, value);
                    None
                }
                (Entry::Symbol(symbol), None) => {
                    schematic.symbols.insert((x, y), symbol);
                    None
                }
                (Entry::Symbol(symbol), Some((key, value))) => {
                    schematic.symbols.insert((x, y), symbol);
                    schematic.parts.insert(key, value);
                    None
                }
            }
        }

        Ok(schematic)
    }
}

fn solve_part_1(input: &str) -> Option<u128> {
    let schematic: Schematic = input.parse().ok()?;

    let sum = schematic
        .parts
        .iter()
        .filter_map(|((x_range, y), &part)| {
            (x_range.start().saturating_sub(1)..=x_range.end().saturating_add(1))
                .flat_map(|x| (y.saturating_sub(1)..=y.saturating_add(1)).map(move |y| (x, y)))
                .find_map(|key| schematic.symbols.get(&key))
                .map(move |_| part as u128)
        })
        .sum();

    Some(sum)
}

fn solve_part_2(input: &str) -> Option<u128> {
    let schematic: Schematic = input.parse().ok()?;

    let sum = schematic
        .symbols
        .iter()
        .filter(|(_, &symbol)| symbol == '*')
        .filter_map(|(&(x, y), _)| {
            let ys = y.saturating_sub(1)..=y.saturating_add(1);

            let adjacent = schematic
                .parts
                .iter()
                .filter(|((_, y), _)| ys.contains(y))
                .filter(|((xs, _), _)| {
                    (xs.start().saturating_sub(1)..=xs.end().saturating_add(1)).contains(&x)
                })
                .map(|(_, &value)| value as u128)
                .take(3)
                .collect::<Vec<_>>();

            if adjacent.len() == 2 {
                Some(adjacent.into_iter().product::<u128>())
            } else {
                None
            }
        })
        .sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        const INPUT: &'static str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
        const RESULT: Option<u128> = Some(4361);

        assert_eq!(solve_part_1(INPUT), RESULT);
    }

    #[test]
    fn example_part_2() {
        const INPUT: &'static str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
        const RESULT: Option<u128> = Some(467835);

        assert_eq!(solve_part_2(INPUT), RESULT);
    }
}
