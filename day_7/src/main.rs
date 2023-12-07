use std::{
    cmp::Ordering,
    str::FromStr,
};

use clap::Parser;

mod main_part_2;

use main_part_2::solve_part_2;

/// Command arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file from AoC
    #[arg(short, long, default_value_t = String::from("inputs/day_7.txt"))]
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

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl TryFrom<char> for Card {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            'T' => Ok(Self::Ten),
            'J' => Ok(Self::Jack),
            'Q' => Ok(Self::Queen),
            'K' => Ok(Self::King),
            'A' => Ok(Self::Ace),
            _ => Err("Unknown card"),
        }
    }
}

impl Card {
    fn all() -> impl Iterator<Item = Self> {
        [
            Self::Two,
            Self::Three,
            Self::Four,
            Self::Five,
            Self::Six,
            Self::Seven,
            Self::Eight,
            Self::Nine,
            Self::Ten,
            Self::Jack,
            Self::Queen,
            Self::King,
            Self::Ace,
        ]
        .into_iter()
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Hand {
    cards: [Card; 5],
}

impl FromStr for Hand {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.trim().chars().map(Card::try_from);

        let hand = Self {
            cards: [
                chars.next().ok_or("Not Enough Cards")??,
                chars.next().ok_or("Not Enough Cards")??,
                chars.next().ok_or("Not Enough Cards")??,
                chars.next().ok_or("Not Enough Cards")??,
                chars.next().ok_or("Not Enough Cards")??,
            ],
        };

        let None = chars.next() else {
            return Err("Too Many Cards");
        };

        Ok(hand)
    }
}

impl Hand {
    fn classify(&self) -> HandType {
        let mut counts = Card::all()
            .map(|target| self.cards.iter().filter(|&&card| card == target).count())
            .collect::<Vec<_>>();

        counts.sort();

        match counts[..] {
            [.., 5] => HandType::FiveOfAKind,
            [.., 4] => HandType::FourOfAKind,
            [.., 2, 3] => HandType::FullHouse,
            [.., 3] => HandType::ThreeOfAKind,
            [.., 2, 2] => HandType::TwoPair,
            [.., 2] => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let by_classification = self.classify().cmp(&other.classify());

        let Ordering::Equal = by_classification else {
            return by_classification;
        };

        let by_higher_card = self
            .cards
            .iter()
            .zip(other.cards.iter())
            .map(|(a, b)| a.cmp(b))
            .find(|&ordering| ordering != Ordering::Equal);

        if let Some(ordering) = by_higher_card {
            ordering
        } else {
            Ordering::Equal
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

struct Game {
    hands: Vec<(Hand, u32)>,
}

impl FromStr for Game {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hands = s
            .trim()
            .lines()
            .map(|line| {
                let mut tokens = line.split_ascii_whitespace();

                let hand = tokens.next().ok_or("Missing Hand")?.parse()?;

                let bid = tokens
                    .next()
                    .ok_or("Missing Bid")?
                    .parse()
                    .map_err(|_| "Could not parse bid")?;

                Ok((hand, bid))
            })
            .collect::<Result<_, _>>()?;

        Ok(Game { hands })
    }
}

impl Game {
    fn score(&self) -> u128 {
        let mut hands = self.hands.clone();

        hands.sort_by_key(|(hand, _)| *hand);

        hands
            .into_iter()
            .enumerate()
            .map(|(index, (_, bid))| (index as u128 + 1) * (bid as u128))
            .sum()
    }
}

fn solve_part_1(input: &str) -> Option<u128> {
    let game: Game = input.parse().ok()?;

    Some(game.score())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        const INPUT: &'static str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
        const RESULT: Option<u128> = Some(6440);

        assert_eq!(solve_part_1(INPUT), RESULT);
    }

    #[test]
    fn example_part_2() {
        const INPUT: &'static str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
        const RESULT: Option<u128> = Some(5905);

        assert_eq!(solve_part_2(INPUT), RESULT);
    }
}
