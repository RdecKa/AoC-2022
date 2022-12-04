mod interval;
use interval::Interval;

#[allow(dead_code)]
const INPUT: &str = include_str!("./../input.txt");
#[allow(dead_code)]
const TEST_INPUT: &str = include_str!("./../test_input.txt");

fn main() {
    // Star 1
    let result = star1(INPUT);
    println!("Star 1: {}", result);

    // Star 2
    let result = star2(INPUT);
    println!("Star 2: {}", result);
}

fn star1(input: &str) -> i32 {
    let parsed_input = parse_input(input);
    parsed_input
        .iter()
        .map(|(first, second)| (first.contains(second) || second.contains(first)) as i32)
        .sum()
}

fn star2(input: &str) -> i32 {
    let parsed_input = parse_input(input);
    parsed_input.len() as i32
}

fn parse_input(input: &str) -> Vec<(Interval, Interval)> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(",").into_iter();
            let a = Interval::from(split.next().unwrap());
            let b = Interval::from(split.next().unwrap());
            (a, b)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star1() {
        let result = star1(TEST_INPUT);
        assert_eq!(result, 2);
    }

    #[test]
    fn full_star1() {
        let result = star1(INPUT);
        assert_eq!(result, 534);
    }

    #[test]
    fn test_star2() {
        let result = star2(TEST_INPUT);
        assert_eq!(result, 0);
    }

    #[test]
    fn full_star2() {
        let result = star2(INPUT);
        assert_eq!(result, 0);
    }
}
