use std::{
    collections::HashMap,
    convert::Infallible,
    ops::RangeInclusive,
    str::FromStr,
};

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

        for (y, line) in s.lines().enumerate() {
            let mut active_part: Option<((RangeInclusive<usize>, usize), usize)> = None;

            for (x, char) in line.char_indices() {
                match (Entry::from(char), &active_part) {
                    (Entry::Value(digit), None) => {
                        active_part = Some(((x..=x, y), digit as usize));
                    }
                    (Entry::Value(digit), Some(((old_range, _), old_value))) => {
                        active_part =
                            Some(((*old_range.start()..=x, y), 10 * old_value + digit as usize));
                    }
                    (Entry::Blank, None) => {}
                    (Entry::Blank, Some((key, value))) => {
                        schematic.parts.insert(key.clone(), *value);
                        active_part = None;
                    }
                    (Entry::Symbol(symbol), None) => {
                        schematic.symbols.insert((x, y), symbol);
                    }
                    (Entry::Symbol(symbol), Some((key, value))) => {
                        schematic.symbols.insert((x, y), symbol);
                        schematic.parts.insert(key.clone(), *value);
                        active_part = None;
                    }
                }
            }

            if let Some((key, value)) = active_part {
                schematic.parts.insert(key, value);
            }
        }

        Ok(schematic)
    }
}

fn solve_part_1(input: &str) -> Option<u128> {
    let schematic: Schematic = input.parse().unwrap();
    let mut sum = 0;

    for ((x_range, y), part) in schematic.parts.iter() {
        let y_range = y.saturating_sub(1)..=y.saturating_add(1);
        let x_range = x_range.start().saturating_sub(1)..=x_range.end().saturating_add(1);

        let has_symbol = x_range
            .flat_map(|x| y_range.clone().map(move |y| (x, y)))
            .any(|key| schematic.symbols.get(&key).is_some());

        if has_symbol {
            sum += *part as u128;
        }
    }

    Some(sum)
}

fn solve_part_2(input: &str) -> Option<u128> {
    let schematic: Schematic = input.parse().unwrap();
    let mut sum = 0;

    for (&(x, y), _) in schematic
        .symbols
        .iter()
        .filter(|(_, &symbol)| symbol == '*')
    {
        let y_range = y.saturating_sub(1)..=y.saturating_add(1);

        let mut adjacent = schematic
            .parts
            .iter()
            .filter(|((x_range, y), _)| {
                let x_range = x_range.start().saturating_sub(1)..=x_range.end().saturating_add(1);
                x_range.contains(&x) && y_range.contains(y)
            })
            .map(|(_, &value)| value as u128);

        match (adjacent.next(), adjacent.next(), adjacent.next()) {
            (Some(a), Some(b), None) => sum += a * b,
            _ => {}
        }
    }

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
