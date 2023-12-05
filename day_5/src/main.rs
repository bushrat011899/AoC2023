use std::{collections::VecDeque, ops::Range, str::FromStr};

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

#[derive(Debug)]
struct Seeds(Vec<usize>);

impl FromStr for Seeds {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.trim().split_ascii_whitespace();

        let Some("seeds:") = tokens.next() else {
            return Err("Missing 'seeds' token");
        };

        let inner = tokens
            .map(|token| token.parse())
            .collect::<Result<_, _>>()
            .map_err(|_| "Could not parse seed ID")?;

        Ok(Seeds(inner))
    }
}

#[derive(Debug, Clone)]
struct Rule {
    source: Range<usize>,
    destination: Range<usize>,
}

impl Rule {
    fn apply(&self, source: usize) -> Option<usize> {
        self.source
            .contains(&source)
            .then(|| self.destination.start + source - self.source.start)
    }

    fn apply_range(
        &self,
        source: Range<usize>,
    ) -> (
        Option<Range<usize>>,
        Option<Range<usize>>,
        Option<Range<usize>>,
    ) {
        if source.end <= self.source.start {
            // Provided range is before this rule
            return (Some(source), None, None);
        }

        if source.start >= self.source.end {
            // Provided range is after this rule
            return (None, None, Some(source));
        }

        if source == self.source {
            // Perfect overlap
            return (None, Some(self.destination.clone()), None);
        }

        if self.source.start <= source.start && source.end <= self.source.end {
            // Rule totally covers provided range
            let start = source.start + self.destination.start - self.source.start;
            let end = source.end + self.destination.start - self.source.start;
            return (None, Some(start..end), None);
        }

        if source.start <= self.source.start && source.end <= self.source.end {
            // Rule totally covers provided range
            let start = self.destination.start;
            let end = source.end + self.destination.start - self.source.start;
            return (
                Some(source.start..self.source.start),
                Some(start..end),
                None,
            );
        }

        if self.source.start <= source.start && self.source.end <= source.end {
            // Rule totally covers provided range
            let start = source.start + self.destination.start - self.source.start;
            let end = self.destination.end;
            return (None, Some(start..end), Some(self.source.end..source.end));
        }

        (
            Some(source.start..self.source.start),
            Some(self.destination.clone()),
            Some(self.source.end..source.end),
        )
    }
}

#[derive(Debug)]
struct Map {
    from: String,
    to: String,
    overrides: Vec<Rule>,
}

impl Map {
    fn map(&self, source: usize) -> usize {
        for rule in self.overrides.iter() {
            let Some(destination) = rule.apply(source) else {
                continue;
            };

            return destination;
        }

        source
    }

    fn map_range(&self, source: Range<usize>) -> Vec<Range<usize>> {
        let mut pending = VecDeque::new();
        let mut result = Vec::new();

        pending.push_back(source);

        for rule in self.overrides.iter() {
            let mut this_pass = VecDeque::new();
            std::mem::swap(&mut this_pass, &mut pending);

            for range in this_pass {
                let (too_small, mapped, too_large) = rule.apply_range(range);

                if let Some(range) = too_small {
                    pending.push_back(range);
                }

                if let Some(range) = too_large {
                    pending.push_back(range);
                }

                if let Some(range) = mapped {
                    result.push(range);
                }
            }
        }

        result.extend(pending);

        result
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

        let overrides = lines
            .map(|line| {
                let mut tokens = line.split_ascii_whitespace();

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
            })
            .collect::<Result<_, _>>()?;

        Ok(Map {
            from,
            to,
            overrides,
        })
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Seeds,
    maps: Vec<Map>,
}

impl FromStr for Almanac {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chunks = s.trim().split_terminator("\n\n");

        let seeds = chunks.next().ok_or("Missing 'seeds' chunk")?.parse()?;

        let maps = chunks
            .map(|chunk| chunk.parse())
            .collect::<Result<_, _>>()?;

        Ok(Almanac { seeds, maps })
    }
}

impl Almanac {
    fn map(&self, from: &str, value: usize) -> Option<(&str, usize)> {
        let map = self.maps.iter().find(|map| map.from == from)?;

        Some((map.to.as_str(), map.map(value)))
    }
}

fn solve_part_1(input: &str) -> Option<usize> {
    let almanac = Almanac::from_str(input).ok()?;

    let mut current_type = "seed";
    let mut ids = almanac.seeds.0.clone();

    while current_type != "location" {
        let mut temp_type = current_type;

        for id in ids.iter_mut() {
            let (new_type, new_id) = almanac.map(current_type, *id)?;
            temp_type = new_type;
            *id = new_id;
        }

        current_type = temp_type;
    }

    ids.into_iter().min()
}

fn solve_part_2(input: &str) -> Option<usize> {
    let almanac = Almanac::from_str(input).ok()?;

    let mut current_type = "seed";
    let mut current_ranges = almanac
        .seeds
        .0
        .chunks(2)
        .map(|chunk| chunk[0]..(chunk[0] + chunk[1]))
        .collect::<Vec<_>>();

    while current_type != "location" {
        let map = almanac.maps.iter().find(|map| map.from == current_type)?;
        current_type = map.to.as_str();
        current_ranges = current_ranges
            .into_iter()
            .flat_map(|range| map.map_range(range))
            .collect();
    }

    current_ranges.iter().map(|range| range.start).min()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_map() {
        let map = Map::from_str(
            r#"seed-to-soil map:
50 98 2
52 50 48"#,
        )
        .expect("Must be able to parse example map");

        (98..100)
            .zip(50..52)
            .for_each(|(source, destination)| assert_eq!(map.map(source), destination));
        (50..98)
            .zip(52..100)
            .for_each(|(source, destination)| assert_eq!(map.map(source), destination));
        (0..50)
            .zip(0..50)
            .for_each(|(source, destination)| assert_eq!(map.map(source), destination));
    }

    #[test]
    fn example_map_range() {
        let map = Map::from_str(
            r#"seed-to-soil map:
50 98 2
52 50 48"#,
        )
        .expect("Must be able to parse example map");

        let mut mapped = map.map_range(0..usize::MAX);

        mapped.sort_by(|a, b| a.end.cmp(&b.end));

        dbg!(mapped);
    }

    #[test]
    fn example_part_1() {
        const INPUT: &'static str = r#"seeds: 79 14 55 13

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
        const INPUT: &'static str = r#"seeds: 79 14 55 13

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
