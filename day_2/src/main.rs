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

    let result = solve_part_1(input.as_str());

    println!("Part 1: {:?}", result);

    let result = solve_part_2(input.as_str());

    println!("Part 2: {:?}", result);
}

#[derive(Default, Debug)]
struct Dice<'a> {
    count: std::collections::HashMap<&'a str, u8>,
}

impl<'a> Dice<'a> {
    fn power(&self) -> u32 {
        self.count.values().map(|&value| value as u32).product()
    }

    fn subset(&self, other: &Self) -> bool {
        self.count.keys().all(|key| {
            self.count.get(key).copied().unwrap_or_default()
                <= other.count.get(key).copied().unwrap_or_default()
        })
    }
}

impl<'a> TryFrom<&'a str> for Dice<'a> {
    type Error = ();

    fn try_from(summary: &'a str) -> Result<Self, Self::Error> {
        let mut round = Self::default();

        for cubes in summary.split(',') {
            let mut split = cubes.trim().split(' ');

            let count = split.next().ok_or(())?.parse::<u8>().map_err(|_| ())?;

            round.count.insert(split.next().ok_or(())?, count);

            if split.next().is_some() {
                return Err(());
            }
        }

        Ok(round)
    }
}

#[derive(Default, Debug)]
struct Game<'a> {
    id: u8,
    rounds: Vec<Dice<'a>>,
}

impl<'a> TryFrom<&'a str> for Game<'a> {
    type Error = ();

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut split = value.strip_prefix("Game").ok_or(())?.trim().split(':');

        let id = split
            .next()
            .ok_or(())?
            .trim()
            .parse::<u8>()
            .map_err(|_| ())?;

        let rounds = split
            .next()
            .ok_or(())?
            .split(';')
            .map(str::trim)
            .map(Dice::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        if split.next().is_some() {
            return Err(());
        }

        Ok(Self { id, rounds })
    }
}

impl<'a> Game<'a> {
    fn minimum_bag(&self) -> Dice {
        self.rounds.iter().fold(Dice::default(), |mut bag, round| {
            for (&colour, &count) in round.count.iter() {
                let old_count = bag.count.entry(colour).or_default();
                *old_count = (*old_count).max(count);
            }

            bag
        })
    }
}

fn solve_part_1(input: &str) -> Option<u128> {
    let bag = Dice {
        count: vec![("red", 12), ("green", 13), ("blue", 14)]
            .into_iter()
            .collect(),
    };

    input
        .lines()
        .map(|line| Game::try_from(line).ok())
        .filter(|g| !g.as_ref().is_some_and(|g| !g.minimum_bag().subset(&bag)))
        .try_fold(0, |x, game| Some(game?.id as u128 + x))
}

fn solve_part_2(input: &str) -> Option<u128> {
    input
        .lines()
        .map(|line| Some(Game::try_from(line).ok()?.minimum_bag().power()))
        .try_fold(0, |x, power| Some(power? as u128 + x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        const INPUT: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        const RESULT: Option<u128> = Some(8);

        assert_eq!(solve_part_1(INPUT), RESULT);
    }

    #[test]
    fn example_part_2() {
        const INPUT: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        const RESULT: Option<u128> = Some(2286);

        assert_eq!(solve_part_2(INPUT), RESULT);
    }
}
