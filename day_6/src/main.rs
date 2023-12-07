use std::{
    ops::RangeInclusive,
    str::FromStr,
};

use clap::Parser;

/// Command arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file from AoC
    #[arg(short, long, default_value_t = String::from("inputs/day_6.txt"))]
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

struct Race {
    time: u128,
    distance: u128,
}

impl Race {
    fn test(&self, hold: u128) -> u128 {
        const INITIAL_SPEED: u128 = 0; // 0 mm/ms
        const ACCELERATION: u128 = 1; // 1 mm/(ms^2)

        let start_time = hold.min(self.time);
        let start_speed = INITIAL_SPEED + ACCELERATION * start_time;
        

        (self.time - start_time) * start_speed
    }

    fn record_breakers(&self) -> RangeInclusive<u128> {
        // Need to find the minimum and maximum hold time
        let (min, max) = (0..=self.time)
            .map(|hold| (hold, self.test(hold)))
            .filter(|&(_, distance)| distance > self.distance)
            .fold((u128::MAX, 0), |(min, max), (hold, _)| {
                (min.min(hold), max.max(hold))
            });
        min..=max
    }
}

struct Competition {
    races: Vec<Race>,
}

impl FromStr for Competition {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let times = lines
            .next()
            .ok_or("Unexpected EOF")?
            .strip_prefix("Time:")
            .ok_or("Missing 'Time:' prefix")?
            .trim()
            .split_ascii_whitespace()
            .map(|token| token.parse())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| "Could not parse times")?;

        let distances = lines
            .next()
            .ok_or("Unexpected EOF")?
            .strip_prefix("Distance:")
            .ok_or("Missing 'Distance:' prefix")?
            .trim()
            .split_ascii_whitespace()
            .map(|token| token.parse())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| "Could not parse distances")?;

        Ok(Competition {
            races: times
                .into_iter()
                .zip(distances)
                .map(|(time, distance)| Race { time, distance })
                .collect(),
        })
    }
}

struct TheBigCompetition {
    race: Race,
}

impl FromStr for TheBigCompetition {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let time = lines
            .next()
            .ok_or("Unexpected EOF")?
            .strip_prefix("Time:")
            .ok_or("Missing 'Time:' prefix")?
            .split_ascii_whitespace()
            .collect::<String>()
            .parse()
            .map_err(|_| "Could not parse time")?;

        let distance = lines
            .next()
            .ok_or("Unexpected EOF")?
            .strip_prefix("Distance:")
            .ok_or("Missing 'Distance:' prefix")?
            .split_ascii_whitespace()
            .collect::<String>()
            .parse()
            .map_err(|_| "Could not parse distance")?;

        Ok(TheBigCompetition {
            race: Race { time, distance },
        })
    }
}

fn solve_part_1(input: &str) -> Option<u128> {
    let comp = Competition::from_str(input).ok()?;

    let result = comp
        .races
        .iter()
        .map(|race| race.record_breakers())
        .map(|range| range.count() as u128)
        .product();

    Some(result)
}

fn solve_part_2(input: &str) -> Option<u128> {
    let comp = TheBigCompetition::from_str(input).expect("");

    let result = comp.race.record_breakers().count() as u128;

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        const INPUT: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;
        const RESULT: Option<u128> = Some(288);

        assert_eq!(solve_part_1(INPUT), RESULT);
    }

    #[test]
    fn example_part_2() {
        const INPUT: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;
        const RESULT: Option<u128> = Some(71503);

        assert_eq!(solve_part_2(INPUT), RESULT);
    }
}
