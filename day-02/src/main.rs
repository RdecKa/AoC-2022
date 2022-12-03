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
    let turns = parse_input(input);
    let correct_turns = construct_correct_turns(turns);
    calculate_score(correct_turns)
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

// Star 1
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

// Star 2
fn i8_to_choice(choice: i8) -> Choice {
    match choice {
        1 | 4 => Choice::Rock,
        2 => Choice::Paper,
        0 | 3 => Choice::Scissors,
        _ => panic!("Invalid choice i8"),
    }
}

fn choose_action(opponent: Choice, planned_result: TurnResult) -> Choice {
    match planned_result {
        TurnResult::Draw => opponent,
        TurnResult::Win => i8_to_choice((opponent as i8) + 1),
        TurnResult::Lose => i8_to_choice((opponent as i8) - 1),
    }
}

fn construct_correct_turns(turns: Vec<Turn>) -> Vec<Turn> {
    turns
        .iter()
        .map(|turn| {
            let planned_result = match turn.me {
                Choice::Rock => TurnResult::Lose,
                Choice::Paper => TurnResult::Draw,
                Choice::Scissors => TurnResult::Win,
            };
            Turn {
                opponent: turn.opponent,
                me: choose_action(turn.opponent, planned_result),
            }
        })
        .collect()
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

    #[test]
    fn test_star2() {
        let result = star2(TEST_INPUT);
        assert_eq!(result, 12);
    }

    #[test]
    fn full_star2() {
        let result = star2(INPUT);
        assert_eq!(result, 15442);
    }
}
