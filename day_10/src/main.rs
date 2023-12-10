use std::{collections::HashMap, str::FromStr};

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
            Some(Tile::VerticalPipe) | Some(Tile::BendSouthEast) | Some(Tile::BendSouthWest) => {
                true
            }
            _ => false,
        };

        let below = match below {
            Some(Tile::VerticalPipe) | Some(Tile::BendNorthEast) | Some(Tile::BendNorthWest) => {
                true
            }
            _ => false,
        };

        let left = match left {
            Some(Tile::HorizontalPipe) | Some(Tile::BendSouthEast) | Some(Tile::BendNorthEast) => {
                true
            }
            _ => false,
        };

        let right = match right {
            Some(Tile::HorizontalPipe) | Some(Tile::BendSouthWest) | Some(Tile::BendNorthWest) => {
                true
            }
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
            return Err("Start Tile Invalid");
        };

        let Some(tile) = row.get_mut(self.start.1) else {
            return Err("Start Tile Invalid");
        };

        *tile = start_tile;

        Ok(self)
    }

    fn path(&self) -> Result<Vec<(usize, usize)>, &'static str> {
        let mut path = Vec::new();

        path.push(self.start.clone());

        while path.len() == 1 || path.last() != Some(&self.start) {
            let position = *path.last().unwrap();
            let last = path.iter().nth_back(1).copied();

            let tile = self
                .tiles
                .get(position.0)
                .ok_or("Invalid Row")?
                .get(position.1)
                .ok_or("Invalid Column")?;

            let (option_1, option_2) = match tile {
                Tile::VerticalPipe => (
                    (position.0.saturating_add(1), position.1),
                    (position.0.saturating_sub(1), position.1),
                ),
                Tile::HorizontalPipe => (
                    (position.0, position.1.saturating_add(1)),
                    (position.0, position.1.saturating_sub(1)),
                ),
                Tile::BendNorthEast => (
                    (position.0.saturating_sub(1), position.1),
                    (position.0, position.1.saturating_add(1)),
                ),
                Tile::BendNorthWest => (
                    (position.0.saturating_sub(1), position.1),
                    (position.0, position.1.saturating_sub(1)),
                ),
                Tile::BendSouthWest => (
                    (position.0.saturating_add(1), position.1),
                    (position.0, position.1.saturating_sub(1)),
                ),
                Tile::BendSouthEast => (
                    (position.0.saturating_add(1), position.1),
                    (position.0, position.1.saturating_add(1)),
                ),
                Tile::Ground | Tile::Start => return Err("Landed on Invalid Tile"),
            };

            if Some(option_1) == last {
                path.push(option_2);
            } else {
                path.push(option_1);
            }
        }

        Ok(path)
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
    let cycle = input
        .parse::<Map>()
        .ok()?
        .try_replace_start()
        .ok()?
        .path()
        .ok()?
        .len();

    Some(cycle / 2)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FillState {
    Unknown,
    Left,
    Right,
    Path,
}

fn solve_part_2(input: &str) -> Option<usize> {
    let map = input.parse::<Map>().ok()?.try_replace_start().ok()?;

    let path = map.path().ok()?;

    let mut tiles = map
        .tiles
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, _)| ((y, x), FillState::Unknown))
        })
        .collect::<HashMap<_, _>>();

    for window in path.windows(2).cycle().take(path.len() + 1) {
        let &[last, this] = window else {
            unreachable!()
        };

        tiles.insert(this, FillState::Path);

        let (left, right) = if last.0 > this.0 {
            // Moved North
            (
                this.1.checked_sub(1).map(|x| (this.0, x)),
                this.1.checked_add(1).map(|x| (this.0, x)),
            )
        } else if last.0 < this.0 {
            // Moved South
            (
                this.1.checked_add(1).map(|x| (this.0, x)),
                this.1.checked_sub(1).map(|x| (this.0, x)),
            )
        } else if last.1 < this.1 {
            // Moved East
            (
                this.0.checked_sub(1).map(|y| (y, this.1)),
                this.0.checked_add(1).map(|y| (y, this.1)),
            )
        } else if last.1 > this.1 {
            // Moved West
            (
                this.0.checked_add(1).map(|y| (y, this.1)),
                this.0.checked_sub(1).map(|y| (y, this.1)),
            )
        } else {
            (None, None)
        };

        if let Some(left) = left {
            if let Some(left) = tiles.get_mut(&left) {
                if *left == FillState::Unknown {
                    *left = FillState::Left;
                }
            }
        }

        if let Some(right) = right {
            if let Some(right) = tiles.get_mut(&right) {
                if *right == FillState::Unknown {
                    *right = FillState::Right;
                }
            }
        }

        let (left, right) = if last.0 > this.0 {
            // Moved North
            (
                last.1.checked_sub(1).map(|x| (last.0, x)),
                last.1.checked_add(1).map(|x| (last.0, x)),
            )
        } else if last.0 < this.0 {
            // Moved South
            (
                last.1.checked_add(1).map(|x| (last.0, x)),
                last.1.checked_sub(1).map(|x| (last.0, x)),
            )
        } else if last.1 < this.1 {
            // Moved East
            (
                last.0.checked_sub(1).map(|y| (y, last.1)),
                last.0.checked_add(1).map(|y| (y, last.1)),
            )
        } else if last.1 > this.1 {
            // Moved West
            (
                last.0.checked_add(1).map(|y| (y, last.1)),
                last.0.checked_sub(1).map(|y| (y, last.1)),
            )
        } else {
            (None, None)
        };

        if let Some(left) = left {
            if let Some(left) = tiles.get_mut(&left) {
                if *left == FillState::Unknown {
                    *left = FillState::Left;
                }
            }
        }

        if let Some(right) = right {
            if let Some(right) = tiles.get_mut(&right) {
                if *right == FillState::Unknown {
                    *right = FillState::Right;
                }
            }
        }
    }

    while tiles.values().any(|&tile| tile == FillState::Unknown) {
        let positions = tiles
            .iter()
            .filter(|(_, tile)| **tile == FillState::Unknown)
            .map(|(&pos, _)| pos)
            .collect::<Vec<_>>();

        for position in positions {
            let north = position.0.checked_sub(1).map(|y| (y, position.1));
            let south = position.0.checked_add(1).map(|y| (y, position.1));
            let east = position.1.checked_add(1).map(|x| (position.0, x));
            let west = position.1.checked_sub(1).map(|x| (position.0, x));

            if let Some(north) = north {
                if let Some(north) = tiles.get(&north) {
                    match north {
                        fill @ FillState::Left | fill @ FillState::Right => {
                            tiles.insert(position, *fill);
                        }
                        _ => {}
                    }
                }
            }

            if let Some(south) = south {
                if let Some(south) = tiles.get(&south) {
                    match south {
                        fill @ FillState::Left | fill @ FillState::Right => {
                            tiles.insert(position, *fill);
                        }
                        _ => {}
                    }
                }
            }

            if let Some(east) = east {
                if let Some(east) = tiles.get(&east) {
                    match east {
                        fill @ FillState::Left | fill @ FillState::Right => {
                            tiles.insert(position, *fill);
                        }
                        _ => {}
                    }
                }
            }

            if let Some(west) = west {
                if let Some(west) = tiles.get(&west) {
                    match west {
                        fill @ FillState::Left | fill @ FillState::Right => {
                            tiles.insert(position, *fill);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    let (_, excluded) = tiles
        .iter()
        .filter(|((y, x), _)| y + x == map.tiles[0].len())
        .filter(|(_, state)| **state != FillState::Path)
        .next()?;

    let result = match excluded {
        FillState::Left => Some(
            tiles
                .values()
                .filter(|&&state| state == FillState::Right)
                .count(),
        ),
        FillState::Right => Some(
            tiles
                .values()
                .filter(|&&state| state == FillState::Left)
                .count(),
        ),
        _ => None,
    };

    // let height = map.tiles.len() as u32;
    // let width = map.tiles.first().unwrap().len() as u32;
    //
    // let mut imgbuf = image::ImageBuffer::new(width, height);
    //
    // for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
    //     let channels: [u8; 3] = match tiles.get(&(y as usize, x as usize)) {
    //         Some(FillState::Path) => [0, 255, 0],
    //         Some(FillState::Left) => [255, 0, 0],
    //         Some(FillState::Right) => [0, 0, 255],
    //         _ => [255, 255, 255],
    //     };
    //
    //     *pixel = image::Rgb(channels);
    // }
    //
    // imgbuf.save("map.png").unwrap();

    result
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

        let map = INPUT
            .parse::<Map>()
            .expect("Must be able to parse map")
            .try_replace_start()
            .expect("Must be able to replace start");

        assert_eq!(map.tiles[map.start.0][map.start.1], Tile::BendSouthEast);
    }

    #[test]
    fn parse_example_2() {
        const INPUT: &str = r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ..."#;

        let map = INPUT
            .parse::<Map>()
            .expect("Must be able to parse map")
            .try_replace_start()
            .expect("Must be able to replace start");

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

    #[test]
    fn example_1_part_2() {
        const INPUT: &str = r#"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."#;
        const RESULT: Option<usize> = Some(4);

        assert_eq!(solve_part_2(INPUT), RESULT);
    }
}
