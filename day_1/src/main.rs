use clap::Parser;

/// Command arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file from AoC
    #[arg(short, long, default_value_t = String::from("inputs/day_1.txt"))]
    input: String,
}

fn main() {
    let args = Args::parse();

    let input = std::fs::read_to_string(args.input).expect("must be able to read input file");
    
    let result = parse_part_1(input.as_str()).expect("Must be able to parse input");

    println!("Part 1: {}", result);
    
    let result = parse_part_2(input.as_str()).expect("Must be able to parse input");

    println!("Part 2: {}", result);
}

fn parse_part_1(input: &str) -> Option<u128> {
    let mut result = 0;

    for line in input.lines() {
        let (first, second) = first_and_last_digit(line)?;
        result += (10 * first + second) as u128;
    }

    Some(result)
}

fn first_and_last_digit(input: &str) -> Option<(u32, u32)> {
    let mut first = None;
    let mut last = None;

    for character in input.chars() {
        let Some(digit) = character.to_digit(10) else {
            continue
        };

        if first.is_none() {
            first = Some(digit);
        }

        last = Some(digit);
    }

    match (first, last) {
        (None, None) => None,
        (None, Some(_)) => unreachable!(),
        (Some(value), None) => Some((value, value)),
        (Some(first), Some(last)) => Some((first, last)),
    }
}

fn parse_part_2(input: &str) -> Option<u128> {
    let mut result = 0;

    for line in input.lines() {
        let search = [
            ("0", 0),
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
            ("zero", 0),
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ];

        let mut first: Option<(usize, u8)> = None;
        let mut last: Option<(usize, u8)> = None;

        for &(pattern, value) in search.iter() {
            first = match (first, line.find(pattern)) {
                (None, None) => None,
                (None, Some(position)) => Some((position, value)),
                (Some((old_position, _)), Some(new_position)) if new_position < old_position => Some((new_position, value)),
                (existing, _) => existing,
            };

            last = match (last, line.rfind(pattern)) {
                (None, None) => None,
                (None, Some(position)) => Some((position, value)),
                (Some((old_position, _)), Some(new_position)) if new_position > old_position => Some((new_position, value)),
                (existing, _) => existing,
            };
        }

        let value = match (first, last) {
            (None, None) => panic!("Could not find digit in string"),
            (None, Some(_)) => unreachable!(),
            (Some((_, value)), None) => 11 * value,
            (Some((_, first)), Some((_, last))) => 10 * first + last,
        };
        
        result += value as u128;
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

        assert_eq!(parse_part_1(input), Some(142));
    }

    #[test]
    fn example_2() {
        let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;

        assert_eq!(parse_part_2(input), Some(281));
    }

    #[test]
    fn example_2_1() {
        let input = r#"two1nine"#;

        assert_eq!(parse_part_2(input), Some(29));
    }

    #[test]
    fn example_2_2() {
        let input = r#"eightwothree"#;

        assert_eq!(parse_part_2(input), Some(83));
    }

    #[test]
    fn example_2_3() {
        let input = r#"abcone2threexyz"#;

        assert_eq!(parse_part_2(input), Some(13));
    }

    #[test]
    fn example_2_4() {
        let input = r#"xtwone3four"#;

        assert_eq!(parse_part_2(input), Some(24));
    }

    #[test]
    fn example_2_5() {
        let input = r#"4nineeightseven2"#;

        assert_eq!(parse_part_2(input), Some(42));
    }

    #[test]
    fn example_2_6() {
        let input = r#"zoneight234"#;

        assert_eq!(parse_part_2(input), Some(14));
    }

    #[test]
    fn example_2_7() {
        let input = r#"7pqrstsixteen"#;

        assert_eq!(parse_part_2(input), Some(76));
    }
}