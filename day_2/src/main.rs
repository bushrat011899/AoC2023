use clap::Parser;

/// Command arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file from AoC
    #[arg(short, long, default_value_t = String::from("inputs/day_2.txt"))]
    input: String,
}

fn main() {
    let args = Args::parse();

    let input = std::fs::read_to_string(args.input).expect("must be able to read input file");

    let result = solve_part_1(input.as_str()).expect("Must be able to parse input");

    println!("Part 1: {}", result);

    let result = solve_part_2(input.as_str()).expect("Must be able to parse input");

    println!("Part 2: {}", result);
}

struct Bag {
    red: u8,
    green: u8,
    blue: u8,
}

struct Round {
    red: u8,
    green: u8,
    blue: u8,
}

struct Game {
    id: u8,
    rounds: Vec<Round>,
}

impl<'a> TryFrom<&'a str> for Game {
    type Error = ();

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let value = value.strip_prefix("Game ").ok_or(())?;

        let mut split = value.split(": ");

        let id = split.next().ok_or(())?.parse::<u8>().map_err(|_| ())?;

        let summaries = split.next().ok_or(())?;

        let None = split.next() else { return Err(()) };

        let mut rounds = Vec::new();

        for summary in summaries.split("; ") {
            let mut round = Round {
                red: 0,
                green: 0,
                blue: 0,
            };

            for cubes in summary.split(", ") {
                let mut split = cubes.split(' ');

                let count = split.next().ok_or(())?.parse::<u8>().map_err(|_| ())?;

                let colour = split.next().ok_or(())?;

                let None = split.next() else { return Err(()) };

                match colour {
                    "red" => round.red = count,
                    "green" => round.green = count,
                    "blue" => round.blue = count,
                    _ => return Err(()),
                }
            }

            rounds.push(round);
        }

        Ok(Game { id, rounds })
    }
}

fn solve_part_1(input: &str) -> Option<u128> {
    let bag = Bag {
        red: 12,
        green: 13,
        blue: 14,
    };

    let games = input
        .lines()
        .map(|line| Game::try_from(line))
        .collect::<Result<Vec<_>, _>>()
        .ok()?;

    let result = games
        .into_iter()
        .filter(|game| {
            game.rounds.iter().all(|round| {
                round.red <= bag.red && round.green <= bag.green && round.blue <= bag.blue
            })
        })
        .fold(0, |x, game| x + game.id as u128);

    Some(result)
}

fn solve_part_2(input: &str) -> Option<u128> {
    let games = input
        .lines()
        .map(|line| Game::try_from(line))
        .collect::<Result<Vec<_>, _>>()
        .ok()?;

    let result = games
        .into_iter()
        .map(|game| {
            let mut min_bag = Bag {
                red: 0,
                green: 0,
                blue: 0,
            };

            min_bag.red = game.rounds.iter().map(|round| round.red).max().unwrap_or_default();
            min_bag.green = game.rounds.iter().map(|round| round.green).max().unwrap_or_default();
            min_bag.blue = game.rounds.iter().map(|round| round.blue).max().unwrap_or_default();

            min_bag
        })
        .map(|min_bag| min_bag.red as u32 * min_bag.green as u32 * min_bag.blue as u32)
        .fold(0, |x, power| x + power as u128);

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        const INPUT: &'static str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        const RESULT: Option<u128> = Some(8);

        assert_eq!(solve_part_1(INPUT), RESULT);
    }

    #[test]
    fn example_part_2() {
        const INPUT: &'static str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        const RESULT: Option<u128> = Some(2286);

        assert_eq!(solve_part_2(INPUT), RESULT);
    }
}
