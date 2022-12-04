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

fn star1(input: &str) -> usize {
    parse_input(input)
        .iter()
        .filter(|(first, second)| first.contains(second) || second.contains(first))
        .count()
}

fn star2(input: &str) -> usize {
    parse_input(input)
        .iter()
        .filter(|(first, second)| first.overlaps_with(second))
        .count()
}

fn parse_input(input: &str) -> Vec<(Interval, Interval)> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(",").into_iter();
            let a: Interval = split.next().unwrap().parse().unwrap();
            let b: Interval = split.next().unwrap().parse().unwrap();
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
        assert_eq!(result, 4);
    }

    #[test]
    fn full_star2() {
        let result = star2(INPUT);
        assert_eq!(result, 841);
    }
}
