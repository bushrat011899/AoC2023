use std::{ops::Range, str::FromStr};

use clap::Parser;

/// Command arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file from AoC
    #[arg(short, long, default_value_t = String::from("inputs/day_5.txt"))]
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

#[derive(Debug, Clone)]
struct Inventory {
    item_type: String,
    values: Vec<usize>,
}

impl FromStr for Inventory {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.trim().split_ascii_whitespace();

        let item_type = tokens
            .next()
            .ok_or("Missing inventory")?
            .strip_suffix(':')
            .ok_or("Expected ':'")?
            .to_string();

        let item_type = if let Some(stripped) = item_type.strip_suffix('s') {
            stripped.to_string()
        } else {
            item_type
        };

        let values = tokens
            .map(|token| token.parse())
            .collect::<Result<_, _>>()
            .map_err(|_| "Could not parse inventory values")?;

        Ok(Inventory { item_type, values })
    }
}

#[derive(Debug, Clone)]
struct Rule {
    source: Range<usize>,
    destination: Range<usize>,
}

impl FromStr for Rule {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_ascii_whitespace();

        let destination_start = tokens
            .next()
            .ok_or("Missing 'destination range start' field in mapping")?
            .parse()
            .map_err(|_| "Could not parse")?;

        let source_start = tokens
            .next()
            .ok_or("Missing 'source range start' field in mapping")?
            .parse()
            .map_err(|_| "Could not parse")?;

        let range = tokens
            .next()
            .ok_or("Missing 'range length' field in mapping")?
            .parse::<usize>()
            .map_err(|_| "Could not parse")?;

        let None = tokens.next() else {
            return Err("Unexpected token");
        };

        let source = source_start..(source_start + range);
        let destination = destination_start..(destination_start + range);

        Ok(Rule {
            source,
            destination,
        })
    }
}

impl Rule {
    fn apply_range(
        &self,
        source: Range<usize>,
    ) -> (
        Option<Range<usize>>,
        Option<Range<usize>>,
        Option<Range<usize>>,
    ) {
        let left = source.start.min(self.source.start)..source.end.min(self.source.start);
        let centre = source.start.max(self.source.start)..source.end.min(self.source.end);
        let right = source.start.max(self.source.end)..source.end.max(self.source.end);

        let left = (!left.is_empty()).then_some(left);
        let centre = (!centre.is_empty()).then(|| {
            let start = self.destination.start + centre.start - self.source.start;
            let end = self.destination.start + centre.end - self.source.start;
            start..end
        });
        let right = (!right.is_empty()).then_some(right);

        (left, centre, right)
    }
}

#[derive(Debug)]
struct Map {
    from: String,
    to: String,
    rules: Vec<Rule>,
}

impl Map {
    fn map(&self, s: Range<usize>) -> Vec<Range<usize>> {
        let (mut a, b) = self.rules.iter().fold((vec![], vec![s]), |(a, b), rule| {
            b.into_iter().fold((a, vec![]), |(mut a, mut b), range| {
                let (too_small, mapped, too_large) = rule.apply_range(range);

                if let Some(range) = too_small {
                    b.push(range);
                }

                if let Some(range) = too_large {
                    b.push(range);
                }

                if let Some(range) = mapped {
                    a.push(range);
                }

                (a, b)
            })
        });

        a.extend(b);

        a
    }
}

impl FromStr for Map {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.trim().lines();

        let mut split = lines
            .next()
            .ok_or("Missing header")?
            .strip_suffix("map:")
            .ok_or("Missing 'map' token")?
            .split("-to-");

        let from = split
            .next()
            .ok_or("Missing 'from' field in header")?
            .trim()
            .to_string();

        let to = split
            .next()
            .ok_or("Missing 'to' field in header")?
            .trim()
            .to_string();

        let None = split.next() else {
            return Err("Unexpected token");
        };

        let rules = lines.map(|line| line.parse()).collect::<Result<_, _>>()?;

        Ok(Map { from, to, rules })
    }
}

#[derive(Debug)]
struct Almanac {
    inventory: Inventory,
    maps: Vec<Map>,
}

impl FromStr for Almanac {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.trim().lines();

        Ok(Almanac {
            inventory: lines.next().ok_or("Missing inventory line")?.parse()?,
            maps: lines.try_fold(Vec::<Map>::new(), |mut maps, line| {
                if let Ok(map) = line.parse() {
                    maps.push(map);
                } else if let Ok(rule) = line.parse() {
                    maps.last_mut().ok_or("Orphaned rule")?.rules.push(rule);
                }
                Ok(maps)
            })?,
        })
    }
}

impl Almanac {
    fn map_for(&self, item_type: &str) -> Option<&Map> {
        self.maps.iter().find(|map| map.from == item_type)
    }
}

fn solve_part_1(input: &str) -> Option<usize> {
    let almanac = Almanac::from_str(input).ok()?;

    let mut inventory = almanac.inventory.clone();

    while inventory.item_type != "location" {
        let map = almanac.map_for(inventory.item_type.as_str())?;
        inventory.item_type = map.to.clone();
        inventory.values = inventory
            .values
            .into_iter()
            .map(|value| map.map(value..(value + 1))[0].start)
            .collect();
    }

    inventory.values.into_iter().min()
}

fn solve_part_2(input: &str) -> Option<usize> {
    let almanac = Almanac::from_str(input).ok()?;

    let mut inventory = almanac.inventory.clone();

    while inventory.item_type != "location" {
        let map = almanac.map_for(inventory.item_type.as_str())?;
        inventory.item_type = map.to.clone();
        inventory.values = inventory
            .values
            .chunks(2)
            .map(|chunk| chunk[0]..(chunk[0] + chunk[1]))
            .flat_map(|range| map.map(range))
            .flat_map(|range| [range.start, range.len()])
            .collect();
    }

    inventory.values.chunks(2).map(|chunk| chunk[0]).min()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_example_rule() {
        let rule = "50 98 2"
            .parse::<Rule>()
            .expect("Must be able to parse rule");

        let expected = (Some(0..98), Some(50..52), Some(100..usize::MAX));

        let mapped = rule.apply_range(0..usize::MAX);

        assert_eq!(mapped, expected);
    }

    #[test]
    fn example_map_range() {
        let map = Map::from_str(
            r#"seed-to-soil map:
50 98 2
52 50 48"#,
        )
        .expect("Must be able to parse example map");

        let expected = vec![0..50, 50..52, 52..100, 100..usize::MAX];

        let mut mapped = map.map(0..usize::MAX);

        mapped.sort_by(|a, b| a.end.cmp(&b.end));

        assert_eq!(mapped, expected);
    }

    #[test]
    fn example_part_1() {
        const INPUT: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;
        const RESULT: Option<usize> = Some(35);

        assert_eq!(solve_part_1(INPUT), RESULT);
    }

    #[test]
    fn example_part_2() {
        const INPUT: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;
        const RESULT: Option<usize> = Some(46);

        assert_eq!(solve_part_2(INPUT), RESULT);
    }
}
