use std::str::FromStr;

use clap::Parser;

/// Command arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file from AoC
    #[arg(short, long, default_value_t = String::from("inputs/day_11.txt"))]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Empty,
    Galaxy,
}

impl TryFrom<char> for Tile {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Tile::Empty),
            '#' => Ok(Tile::Galaxy),
            _ => Err("Unknown Tile"),
        }
    }
}

#[derive(Debug)]
struct Map {
    galaxies: Vec<(usize, usize)>,
}

impl FromStr for Map {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let galaxies = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.char_indices()
                    .filter_map(move |(x, char)| {
                        char.try_into().ok().map(move |tile: Tile| ((x, y), tile))
                    })
                    .filter(|(_, tile)| *tile == Tile::Galaxy)
            })
            .map(|((x, y), _)| (x, y))
            .collect();

        Ok(Self { galaxies })
    }
}

impl Map {
    fn save(&self, name: &str) {
        let ((_, max_x), (_, max_y)) = self.galaxies.iter().fold(
            ((usize::MAX, usize::MIN), (usize::MAX, usize::MIN)),
            |((min_x, max_x), (min_y, max_y)), &(x, y)| {
                ((min_x.min(x), max_x.max(x)), (min_y.min(y), max_y.max(y)))
            },
        );

        let mut imgbuf = image::ImageBuffer::new(max_x as u32 + 1, max_y as u32 + 1);

        imgbuf.fill(0);

        for &(x, y) in self.galaxies.iter() {
            let pixel = imgbuf.get_pixel_mut(x as u32, y as u32);
            *pixel = image::Rgb([255u8, 255u8, 255u8]);
        }

        imgbuf.save(name).unwrap();
    }

    fn pairs(&self) -> impl Iterator<Item = ((usize, usize), (usize, usize))> + '_ {
        self.galaxies
            .iter()
            .enumerate()
            .flat_map(|(index, &a)| self.galaxies.iter().skip(index + 1).map(move |&b| (a, b)))
    }

    fn expand_by(mut self, size: usize) -> Self {
        let ((min_x, max_x), (min_y, max_y)) = self.galaxies.iter().fold(
            ((usize::MAX, usize::MIN), (usize::MAX, usize::MIN)),
            |((min_x, max_x), (min_y, max_y)), &(x, y)| {
                ((min_x.min(x), max_x.max(x)), (min_y.min(y), max_y.max(y)))
            },
        );

        let width = min_x..=max_x;
        let height = min_y..=max_y;

        let mut expansions = 0;

        for column in width {
            let column = column + expansions;

            if self.galaxies.iter().any(|&(x, _)| x == column) {
                continue;
            }

            for position in self.galaxies.iter_mut().filter(|(x, _)| *x > column) {
                position.0 += size;
            }

            expansions += size;
        }

        let mut expansions = 0;

        for row in height {
            let row = row + expansions;

            if self.galaxies.iter().any(|&(_, y)| y == row) {
                continue;
            }

            for position in self.galaxies.iter_mut().filter(|(_, y)| *y > row) {
                position.1 += size;
            }

            expansions += size;
        }

        self
    }
}

fn solve_part_1(input: &str) -> Result<usize, &'static str> {
    let sum = input
        .parse::<Map>()?
        .expand_by(1)
        .pairs()
        .map(|(a, b)| a.0.abs_diff(b.0) + a.1.abs_diff(b.1))
        .sum();

    Ok(sum)
}

fn solve_part_2(input: &str) -> Result<usize, &'static str> {
    let sum = input
        .parse::<Map>()?
        .expand_by(1_000_000 - 1)
        .pairs()
        .map(|(a, b)| a.0.abs_diff(b.0) + a.1.abs_diff(b.1))
        .sum();

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_part_1() {
        const INPUT: &str = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;
        const RESULT: Result<usize, &'static str> = Ok(374);

        assert_eq!(solve_part_1(INPUT), RESULT);
    }

    #[test]
    fn example_1_part_2() {
        const INPUT: &str = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;
        const RESULT: usize = 1030;

        let sum: usize = INPUT
            .parse::<Map>()
            .expect("Must be able to parse")
            .expand_by(9)
            .pairs()
            .map(|(a, b)| a.0.abs_diff(b.0) + a.1.abs_diff(b.1))
            .sum();

        assert_eq!(sum, RESULT);
    }

    #[test]
    fn example_2_part_2() {
        const INPUT: &str = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;
        const RESULT: usize = 8410;

        let sum: usize = INPUT
            .parse::<Map>()
            .expect("Must be able to parse")
            .expand_by(99)
            .pairs()
            .map(|(a, b)| a.0.abs_diff(b.0) + a.1.abs_diff(b.1))
            .sum();

        assert_eq!(sum, RESULT);
    }

    #[test]
    fn example_3_part_2() {
        const INPUT: &str = r#"#.#"#;
        const RESULT: usize = 2;

        let sum: usize = INPUT
            .parse::<Map>()
            .expect("Must be able to parse")
            .expand_by(0)
            .pairs()
            .map(|(a, b)| a.0.abs_diff(b.0) + a.1.abs_diff(b.1))
            .sum();

        assert_eq!(sum, RESULT);
    }

    #[test]
    fn example_4_part_2() {
        const INPUT: &str = r#"#.#"#;
        const RESULT: usize = 3;

        let sum: usize = INPUT
            .parse::<Map>()
            .expect("Must be able to parse")
            .expand_by(1)
            .pairs()
            .map(|(a, b)| a.0.abs_diff(b.0) + a.1.abs_diff(b.1))
            .sum();

        assert_eq!(sum, RESULT);
    }
}
