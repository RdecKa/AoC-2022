mod cave;
use cave::{Cave, Rock};

#[allow(dead_code)]
const INPUT: &str = include_str!("./../input.txt");
#[allow(dead_code)]
const TEST_INPUT: &str = include_str!("./../test_input.txt");

const ROCKS: &str = include_str!("./../rocks.txt");

fn main() {
    // Star 1
    let result = star1(INPUT);
    println!("Star 1: {}", result);

    // Star 2
    let result = star2(INPUT);
    println!("Star 2: {}", result);
}

fn star1(input: &str) -> usize {
    let parsed_input = parse_input(input);
    let mut draft_iter = parsed_input.iter().cycle();
    let rocks = parse_rocks(ROCKS);
    let mut rock_iter = rocks.iter().cycle();
    let mut cave = Cave::new(7);
    for _ in 0..2022 {
        let rock = rock_iter.next().unwrap();
        cave.increase_cave_height(rock.height + 3);
        cave.add_falling_rock(rock.clone());
        let mut success = true;
        while success {
            match draft_iter.next().unwrap() {
                Direction::Left => cave.move_rock_left(),
                Direction::Right => cave.move_rock_right(),
            };
            success = cave.move_rock_down();
            if !success {
                cave.fix_falling_rock();
            }
        }
    }

    cave.height
}

fn star2(input: &str) -> i32 {
    let parsed_input = parse_input(input);
    parsed_input.len() as i32
}

fn parse_rocks(input: &str) -> Vec<Rock> {
    input
        .split("\n\n")
        .map(|rock_str| parse_rock(rock_str))
        .collect()
}

fn parse_rock(rock_str: &str) -> Rock {
    Rock::new(rock_str)
}

enum Direction {
    Left,
    Right,
}

fn parse_input(input: &str) -> Vec<Direction> {
    return input
        .trim()
        .chars()
        .map(|c| {
            if c == '<' {
                Direction::Left
            } else if c == '>' {
                Direction::Right
            } else {
                panic!("Invalid input char");
            }
        })
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star1() {
        let result = star1(TEST_INPUT);
        assert_eq!(result, 3068);
    }

    #[test]
    fn full_star1() {
        let result = star1(INPUT);
        assert_eq!(result, 3149);
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
