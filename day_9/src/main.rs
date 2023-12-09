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

struct Predictor<T> {
    wave: Vec<T>,
}

impl Iterator for Predictor<i64> {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        self.wave.iter_mut().rev().fold(None, |sum, diff| {
            *diff += sum.unwrap_or(0);
            Some(*diff)
        })
    }
}

impl FromIterator<i64> for Predictor<i64> {
    fn from_iter<T: IntoIterator<Item = i64>>(iter: T) -> Self {
        let mut wave = iter
            .into_iter()
            .fold(Vec::<i64>::new(), |mut wave, mut item| {
                for old in wave.iter_mut() {
                    std::mem::swap(old, &mut item);
                    item = *old - item;
                }

                wave.push(item);

                wave
            });

        while let Some(0) = wave.last() {
            wave.pop();
        }

        wave.push(0);

        Self { wave }
    }
}

fn solve_part_1(input: &str) -> Option<i64> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|token| token.parse::<i64>().ok())
                .collect::<Option<Predictor<_>>>()?
                .next()
        })
        .try_fold(0, |sum, item| Some(sum + item?))
}

fn solve_part_2(input: &str) -> Option<i64> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|token| token.parse::<i64>().ok())
                .rev()
                .collect::<Option<Predictor<_>>>()?
                .next()
        })
        .try_fold(0, |sum, item| Some(sum + item?))
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
