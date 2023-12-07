use std::{
    cmp::Ordering,
    str::FromStr, collections::HashMap,
};

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl TryFrom<char> for Card {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'J' => Ok(Self::Joker),
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            'T' => Ok(Self::Ten),
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
            Self::Joker,
            Self::Two,
            Self::Three,
            Self::Four,
            Self::Five,
            Self::Six,
            Self::Seven,
            Self::Eight,
            Self::Nine,
            Self::Ten,
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
        let all_counts = Card::all()
            .map(|target| (target, self.cards.iter().filter(|&&card| card == target).count()))
            .collect::<HashMap<_, _>>();

        let mut standard_counts = all_counts.iter().filter(|&(&card, _)| card != Card::Joker).map(|(_, count)| *count).collect::<Vec<_>>();

        standard_counts.sort();

        let jokers = *all_counts.get(&Card::Joker).unwrap_or(&0);

        match (&standard_counts[..], jokers) {
            ([.., 5], 0) => HandType::FiveOfAKind,
            ([.., 4], 1) => HandType::FiveOfAKind,
            ([.., 3], 2) => HandType::FiveOfAKind,
            ([.., 2], 3) => HandType::FiveOfAKind,
            ([.., 1], 4) => HandType::FiveOfAKind,
            ([.., 0], 5) => HandType::FiveOfAKind,

            ([.., 4], 0) => HandType::FourOfAKind,
            ([.., 3], 1) => HandType::FourOfAKind,
            ([.., 2], 2) => HandType::FourOfAKind,
            ([.., 1], 3) => HandType::FourOfAKind,

            ([.., 2, 3], 0) => HandType::FullHouse,
            ([.., 2, 2], 1) => HandType::FullHouse,

            ([.., 3], 0) => HandType::ThreeOfAKind,
            ([.., 2], 1) => HandType::ThreeOfAKind,
            ([.., 1], 2) => HandType::ThreeOfAKind,

            ([.., 2, 2], 0) => HandType::TwoPair,

            ([.., 2], 0) => HandType::OnePair,
            ([.., 1], 1) => HandType::OnePair,

            ([.., 1], 0) => HandType::HighCard,

            _ => unreachable!(),
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

pub fn solve_part_2(input: &str) -> Option<u128> {
    let game: Game = input.parse().ok()?;

    Some(game.score())
}