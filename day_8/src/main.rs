use std::{collections::HashMap, str::FromStr};

use clap::Parser;

/// Command arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file from AoC
    #[arg(short, long, default_value_t = String::from("inputs/day_8.txt"))]
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

enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err("Unknown Direction"),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct NodeId([char; 3]);

impl NodeId {
    fn is_start(&self) -> bool {
        self.0[2] == 'A'
    }

    fn is_end(&self) -> bool {
        self.0[2] == 'Z'
    }
}

impl FromStr for NodeId {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.trim().chars();

        let result = Self([
            chars.next().ok_or("Missing ID Symbol")?,
            chars.next().ok_or("Missing ID Symbol")?,
            chars.next().ok_or("Missing ID Symbol")?,
        ]);

        let None = chars.next() else {
            return Err("Unexpected Symbols");
        };

        Ok(result)
    }
}

struct Node {
    id: NodeId,
    left: NodeId,
    right: NodeId,
}

impl FromStr for Node {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split('=');

        let id = tokens.next().ok_or("Missing Node ID")?.trim().parse()?;

        let mut children = tokens
            .next()
            .ok_or("Missing Child Node IDs")?
            .trim()
            .strip_prefix('(')
            .ok_or("Missing Opening Brace")?
            .strip_suffix(')')
            .ok_or("Missing Closing Brace")?
            .split(',')
            .map(|id| id.parse());

        let left = children.next().ok_or("Missing Left Child")??;

        let right = children.next().ok_or("Missing Right Child")??;

        let None = children.next() else {
            return Err("Unexpected Children");
        };

        let None = tokens.next() else {
            return Err("Unexpected Tokens");
        };

        Ok(Self { id, left, right })
    }
}

struct Map {
    instructions: Vec<Direction>,
    graph: HashMap<NodeId, (NodeId, NodeId)>,
}

impl FromStr for Map {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let instructions = lines
            .next()
            .ok_or("Missing Instructions")?
            .chars()
            .map(|direction| direction.try_into())
            .collect::<Result<_, _>>()?;

        lines.next();

        let graph = lines
            .map(|line| {
                line.parse::<Node>()
                    .map(|node| (node.id, (node.left, node.right)))
            })
            .collect::<Result<_, _>>()?;

        Ok(Self {
            instructions,
            graph,
        })
    }
}

impl Map {
    fn steps_to_end(&self, start: NodeId) -> Option<u128> {
        self.instructions
            .iter()
            .cycle()
            .scan(start, |position, direction| {
                if position.is_end() {
                    return None;
                }

                let Some(&(left, right)) = self.graph.get(&position) else {
                    return Some(Err("At impossible position!"));
                };

                *position = match direction {
                    Direction::Left => left,
                    Direction::Right => right,
                };

                Some(Ok(1))
            })
            .try_fold(0, |sum, step| step.map(|step| step + sum))
            .ok()
    }
}

/// Get the Greatest Common Devisor (GCD) of the provided numbers.
/// From [Victor I. Afolabi](https://gist.github.com/victor-iyi/8a84185c1d52419b0d4915a648d5e3e1)
fn gcd(mut n: u128, mut m: u128) -> u128 {
    assert!(n != 0 && m != 0);

    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }

    n
}

fn solve_part_1(input: &str) -> Option<u128> {
    input
        .parse::<Map>()
        .ok()?
        .steps_to_end(NodeId(['A', 'A', 'A']))
}

fn solve_part_2(input: &str) -> Option<u128> {
    let map = input.parse::<Map>().ok()?;

    map.graph
        .keys()
        .filter(|node| node.is_start())
        .map(|&start| map.steps_to_end(start))
        .try_fold(0, |cycle, steps| {
            let steps = steps?;

            if cycle == 0 {
                Some(steps)
            } else {
                Some(steps / gcd(steps, cycle) * cycle)
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_part_1() {
        const INPUT: &str = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;
        const RESULT: Option<u128> = Some(2);

        assert_eq!(solve_part_1(INPUT), RESULT);
    }

    #[test]
    fn example_2_part_1() {
        const INPUT: &str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;
        const RESULT: Option<u128> = Some(6);

        assert_eq!(solve_part_1(INPUT), RESULT);
    }

    #[test]
    fn example_1_part_2() {
        const INPUT: &str = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;
        const RESULT: Option<u128> = Some(6);

        assert_eq!(solve_part_2(INPUT), RESULT);
    }
}
