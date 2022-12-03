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
    let turns = parse_input(input);
    calculate_score(turns)
}

fn star2(input: &str) -> i32 {
    0
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Choice {
    Rock = 1,
    Paper,
    Scissors,
}

// Result of the turn from my perspective
#[derive(Debug)]
enum TurnResult {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

#[derive(Debug)]
struct Turn {
    opponent: Choice,
    me: Choice,
}

fn parse_input(input: &str) -> Vec<Turn> {
    input
        .lines()
        .map(|line| {
            let values: Vec<char> = line.chars().collect();
            Turn {
                opponent: match values[0] {
                    'A' => Choice::Rock,
                    'B' => Choice::Paper,
                    'C' => Choice::Scissors,
                    _ => panic!("Invalid input"),
                },
                me: match values[2] {
                    'X' => Choice::Rock,
                    'Y' => Choice::Paper,
                    'Z' => Choice::Scissors,
                    _ => panic!("Invalid input"),
                },
            }
        })
        .collect()
}

fn calculate_score(turns: Vec<Turn>) -> i32 {
    turns
        .iter()
        .map(|turn| calculate_score_for_turn(turn))
        .sum()
}

fn calculate_score_for_turn(turn: &Turn) -> i32 {
    let turn_result = get_result(&turn);
    (turn_result as i32) + (turn.me as i32)
}

fn get_result(turn: &Turn) -> TurnResult {
    match (turn.me as i8) - (turn.opponent as i8) {
        0 => TurnResult::Draw,
        1 | -2 => TurnResult::Win,
        2 | -1 => TurnResult::Lose,
        _ => panic!("Invalid result"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star1() {
        let result = star1(TEST_INPUT);
        assert_eq!(result, 15);
    }

    #[test]
    fn full_star1() {
        let result = star1(INPUT);
        assert_eq!(result, 15422);
    }

    // #[test]
    // fn test_star2() {
    //     let result = star2(TEST_INPUT);
    //     assert_eq!(result, 45000);
    // }

    // #[test]
    // fn full_star2() {
    //     let result = star2(INPUT);
    //     assert_eq!(result, 210367);
    // }
}
