use std::str::FromStr;

use clap::Parser;
use rayon::prelude::*;
use indicatif::ParallelProgressIterator;

/// Command arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file from AoC
    #[arg(short, long, default_value_t = String::from("inputs/day_12.txt"))]
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum State {
    Operational,
    Damaged,
    Unknown,
}

impl TryFrom<char> for State {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Operational),
            '#' => Ok(Self::Damaged),
            '?' => Ok(Self::Unknown),
            _ => Err("Unrecognized symbol"),
        }
    }
}

#[derive(Debug, Clone)]
struct Row {
    states: Vec<State>,
    groups: Vec<usize>,
    active: Option<usize>,
}

impl FromStr for Row {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_ascii_whitespace();

        let states = split
            .next()
            .ok_or("Unexpected End of Stream")?
            .chars()
            .map(|char| char.try_into())
            .collect::<Result<_, _>>()?;

        let groups = split
            .next()
            .ok_or("Unexpected End of Stream")?
            .split(',')
            .map(|token| token.trim().parse())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| "Could not parse groups")?;

        let active = None;

        Ok(Self { states, groups, active })
    }
}

impl Row {
    fn arrangements(&self) -> usize {
        let mut groups = self.groups.clone();
        let mut active = self.active;

        for (index, state) in self.states.iter().enumerate() {
            match state {
                State::Damaged => {
                    if active.is_none() {
                        if groups.is_empty() {
                            return 0;
                        }

                        active = Some(groups.remove(0))
                    }
                    
                    let Some(count) = active else {
                        return 0;
                    };
                    
                    let Some(count) = count.checked_sub(1) else {
                        return 0
                    };

                    active = Some(count);
                },
                State::Operational => {
                    if let Some(count) = active {
                        if count == 0 {
                            active = None;
                        } else {
                            return 0
                        }
                    }
                },
                State::Unknown => {
                    if active.is_some_and(|count| count > 0) {
                        active = Some(active.unwrap() - 1);
                    } else if active.is_some_and(|count| count == 0) {
                        active = None;
                    } else {
                        let mut result = 0;

                        let mut clone = Self {
                            states: self.states[index..].to_vec(),
                            groups: groups.clone(),
                            active,
                        };

                        clone.states[0] = State::Operational;
                        result += clone.arrangements();

                        clone.states[0] = State::Damaged;
                        result += clone.arrangements();

                        return result;
                    }
                },
            }
        }

        while let Some(0) = groups.get(0) {
            groups.remove(0);
        }

        if groups.is_empty() {
            1
        } else {
            0
        }
    }

    fn unfold(self) -> Self {
        let n = self.states.len();
        let m = self.groups.len();

        let states = self.states.into_iter().chain(std::iter::once(State::Unknown)).cycle().take((n + 1) * 5 - 1).collect();
        let groups = self.groups.into_iter().cycle().take(m * 5).collect();

        Self { states, groups, active: None }
    }
}

fn solve_part_1(input: &str) -> Result<usize, &'static str> {
    let rows = input.lines().map(|line| line.parse()).collect::<Result<Vec<Row>, _>>()?;

    let count = rows.len();

    Ok(rows.into_par_iter().progress_count(count as u64).map(|row| row.arrangements()).sum())
}

fn solve_part_2(input: &str) -> Result<usize, &'static str> {
    let rows = input.lines().map(|line| line.parse()).collect::<Result<Vec<Row>, _>>()?;

    let count = rows.len();

    Ok(rows.into_par_iter().progress_count(count as u64).map(|row| row.unfold().arrangements()).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_part_1() {
        const INPUT: &str = r#"???.### 1,1,3"#;
        const RESULT: usize = 1;

        let row = INPUT.parse::<Row>().expect("Must be able to parse input");

        assert_eq!(row.arrangements(), RESULT);
    }

    #[test]
    fn example_2_part_1() {
        const INPUT: &str = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;
        const RESULT: Result<usize, &'static str> = Ok(21);

        assert_eq!(solve_part_1(INPUT), RESULT);
    }

    #[test]
    fn example_1_part_2() {
        const INPUT: &str = r#"???.### 1,1,3"#;
        const RESULT: usize = 1;

        let row = INPUT.parse::<Row>().expect("Must be able to parse input");

        assert_eq!(row.unfold().arrangements(), RESULT);
    }

    #[test]
    fn example_2_part_2() {
        const INPUT: &str = r#".??..??...?##. 1,1,3"#;
        const RESULT: usize = 16384;

        let row = INPUT.parse::<Row>().expect("Must be able to parse input");

        assert_eq!(row.unfold().arrangements(), RESULT);
    }

    #[test]
    fn example_3_part_2() {
        const INPUT: &str = r#"?#?#?#?#?#?#?#? 1,3,1,6"#;
        const RESULT: usize = 1;

        let row = INPUT.parse::<Row>().expect("Must be able to parse input");

        assert_eq!(row.unfold().arrangements(), RESULT);
    }

    #[test]
    fn example_4_part_2() {
        const INPUT: &str = r#"????.#...#... 4,1,1"#;
        const RESULT: usize = 16;

        let row = INPUT.parse::<Row>().expect("Must be able to parse input");

        assert_eq!(row.unfold().arrangements(), RESULT);
    }

    #[test]
    fn example_5_part_2() {
        const INPUT: &str = r#"????.######..#####. 1,6,5"#;
        const RESULT: usize = 2500;

        let row = INPUT.parse::<Row>().expect("Must be able to parse input");

        assert_eq!(row.unfold().arrangements(), RESULT);
    }

    #[test]
    fn example_6_part_2() {
        const INPUT: &str = r#"?###???????? 3,2,1"#;
        const RESULT: usize = 506250;

        let row = INPUT.parse::<Row>().expect("Must be able to parse input");

        assert_eq!(row.unfold().arrangements(), RESULT);
    }

    #[test]
    fn example_7_part_2() {
        const INPUT: &str = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;
        const RESULT: Result<usize, &'static str> = Ok(525152);

        assert_eq!(solve_part_2(INPUT), RESULT);
    }
}
