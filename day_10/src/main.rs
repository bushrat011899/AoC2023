use std::str::FromStr;

use clap::Parser;

/// Command arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file from AoC
    #[arg(short, long, default_value_t = String::from("inputs/day_10.txt"))]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    VerticalPipe,
    HorizontalPipe,
    BendNorthEast,
    BendNorthWest,
    BendSouthWest,
    BendSouthEast,
    Ground,
    Start,
}

impl TryFrom<char> for Tile {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Self::VerticalPipe),
            '-' => Ok(Self::HorizontalPipe),
            'L' => Ok(Self::BendNorthEast),
            'J' => Ok(Self::BendNorthWest),
            '7' => Ok(Self::BendSouthWest),
            'F' => Ok(Self::BendSouthEast),
            '.' => Ok(Self::Ground),
            'S' => Ok(Self::Start),
            _ => Err("Unknown Tile"),
        }
    }
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    start: (usize, usize),
}

impl Map {
    fn try_replace_start(mut self) -> Result<Self, &'static str> {
        let above = (self.start.0.saturating_sub(1), self.start.1);
        let below = (self.start.0.saturating_add(1), self.start.1);
        let left = (self.start.0, self.start.1.saturating_sub(1));
        let right = (self.start.0, self.start.1.saturating_add(1));

        let above = if above != self.start {
            self.tiles.get(above.0).and_then(|row| row.get(above.1))
        } else {
            None
        };

        let below = if below != self.start {
            self.tiles.get(below.0).and_then(|row| row.get(below.1))
        } else {
            None
        };

        let left = if left != self.start {
            self.tiles.get(left.0).and_then(|row| row.get(left.1))
        } else {
            None
        };

        let right = if right != self.start {
            self.tiles.get(right.0).and_then(|row| row.get(right.1))
        } else {
            None
        };

        let above = match above {
            Some(Tile::VerticalPipe) | Some(Tile::BendSouthEast) | Some(Tile::BendSouthWest) => true,
            _ => false,
        };

        let below = match below {
            Some(Tile::VerticalPipe) | Some(Tile::BendNorthEast) | Some(Tile::BendNorthWest) => true,
            _ => false,
        };

        let left = match left {
            Some(Tile::HorizontalPipe) | Some(Tile::BendSouthEast) | Some(Tile::BendNorthEast) => true,
            _ => false,
        };

        let right = match right {
            Some(Tile::HorizontalPipe) | Some(Tile::BendSouthWest) | Some(Tile::BendNorthWest) => true,
            _ => false,
        };

        let start_tile = match (above, below, left, right) {
            (true, true, false, false) => Ok(Tile::VerticalPipe),
            (false, false, true, true) => Ok(Tile::HorizontalPipe),
            (true, false, true, false) => Ok(Tile::BendNorthWest),
            (true, false, false, true) => Ok(Tile::BendNorthEast),
            (false, true, true, false) => Ok(Tile::BendSouthWest),
            (false, true, false, true) => Ok(Tile::BendSouthEast),
            _ => Err("Starting tile has ambiguous connections"),
        }?;

        let Some(row) = self.tiles.get_mut(self.start.0) else {
            return Err("Start Tile Invalid")
        };

        let Some(tile) = row.get_mut(self.start.1) else {
            return Err("Start Tile Invalid")
        };

        *tile = start_tile;

        Ok(self)
    }

    fn measure_cycle(&self) -> Result<usize, &'static str> {
        let mut last = None;
        let mut position = self.start.clone();
        let mut length = 0;

        while last.is_none() || position != self.start {
            let tile = self.tiles.get(position.0).ok_or("Invalid Row")?.get(position.1).ok_or("Invalid Column")?;

            let (option_1, option_2) = match tile {
                Tile::VerticalPipe => {
                    ((position.0.saturating_add(1), position.1), (position.0.saturating_sub(1), position.1))
                },
                Tile::HorizontalPipe => {
                    ((position.0, position.1.saturating_add(1)), (position.0, position.1.saturating_sub(1)))
                },
                Tile::BendNorthEast => {
                    ((position.0.saturating_sub(1), position.1), (position.0, position.1.saturating_add(1)))
                },
                Tile::BendNorthWest => {
                    ((position.0.saturating_sub(1), position.1), (position.0, position.1.saturating_sub(1)))
                },
                Tile::BendSouthWest => {
                    ((position.0.saturating_add(1), position.1), (position.0, position.1.saturating_sub(1)))
                },
                Tile::BendSouthEast => {
                    ((position.0.saturating_add(1), position.1), (position.0, position.1.saturating_add(1)))
                },
                Tile::Ground | Tile::Start => return Err("Landed on Invalid Tile"),
            };

            if Some(option_1) == last {
                last = Some(position);
                position = option_2;
            } else {
                last = Some(position);
                position = option_1;
            }

            length += 1;
        }

        Ok(length)
    }
}

impl FromStr for Map {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = None;

        let tiles = s
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, char)| {
                        let result = char.try_into();

                        if let Ok(Tile::Start) = result {
                            start = Some((y, x));
                        }

                        result
                    })
                    .collect::<Result<_, _>>()
            })
            .collect::<Result<_, _>>()?;

        let start = start.ok_or("Could not find starting position")?;

        Ok(Self { tiles, start })
    }
}

fn solve_part_1(input: &str) -> Option<usize> {
    let map = input.parse::<Map>().ok()?.try_replace_start().ok()?;

    let cycle = map.measure_cycle().ok()?;
    
    Some(cycle / 2)
}

fn solve_part_2(input: &str) -> Option<u128> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example_1() {
        const INPUT: &str = r#".....
.S-7.
.|.|.
.L-J.
....."#;
        
        let map = INPUT.parse::<Map>().expect("Must be able to parse map").try_replace_start().expect("Must be able to replace start");

        assert_eq!(map.tiles[map.start.0][map.start.1], Tile::BendSouthEast);
    }

    #[test]
    fn parse_example_2() {
        const INPUT: &str = r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ..."#;
        
        let map = INPUT.parse::<Map>().expect("Must be able to parse map").try_replace_start().expect("Must be able to replace start");

        assert_eq!(map.tiles[map.start.0][map.start.1], Tile::BendSouthEast);
    }

    #[test]
    fn example_1_part_1() {
        const INPUT: &str = r#".....
.S-7.
.|.|.
.L-J.
....."#;
        const RESULT: Option<usize> = Some(4);

        assert_eq!(solve_part_1(INPUT), RESULT);
    }

    #[test]
    fn example_2_part_1() {
        const INPUT: &str = r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ..."#;
        const RESULT: Option<usize> = Some(8);

        assert_eq!(solve_part_1(INPUT), RESULT);
    }
}
