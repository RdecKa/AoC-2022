mod stack;

use stack::Stack;
use std::iter;

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

fn star1(input: &str) -> String {
    let (mut stacks, moves) = parse_input(input);
    execute_plan(&mut stacks, &moves);
    get_top_crates(&stacks)
}

fn star2(input: &str) -> String {
    let (mut stacks, moves) = parse_input(input);
    execute_plan_9001(&mut stacks, &moves);
    get_top_crates(&stacks)
}

fn parse_input(input: &str) -> (Vec<Stack>, Vec<(usize, usize, usize)>) {
    let mut split = input.split("\n\n");
    let stacks_str = split.next().unwrap();
    let moves_str = split.next().unwrap();
    (
        parse_stacks_from_str(stacks_str),
        parse_moves_from_str(moves_str),
    )
}

fn parse_stacks_from_str(stacks_str: &str) -> Vec<Stack> {
    let mut rows = stacks_str.lines().rev();
    // The row with enumeration:
    let num_columns = rows.next().unwrap().split_whitespace().count();
    let mut stacks: Vec<Stack> = iter::repeat(Stack { items: Vec::new() })
        .take(num_columns)
        .collect();
    rows.for_each(|row| parse_stack_row(row, &mut stacks));
    stacks
}

fn parse_stack_row(row: &str, stacks: &mut [Stack]) {
    let chars: Vec<char> = row.chars().collect();
    let columns = chars.chunks(4);
    columns.enumerate().for_each(|(idx, item)| {
        if item[1] != ' ' {
            stacks[idx].push(item[1]);
        }
    })
}

fn parse_moves_from_str(moves_str: &str) -> Vec<(usize, usize, usize)> {
    moves_str
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            split.next(); // "move"
            let count = split.next().unwrap().parse().unwrap();
            split.next(); // "from"
            let source = split.next().unwrap().parse().unwrap();
            split.next(); // "to"
            let destination = split.next().unwrap().parse().unwrap();
            (count, source, destination)
        })
        .collect()
}

fn execute_plan(stacks: &mut [Stack], moves: &[(usize, usize, usize)]) {
    moves.iter().for_each(|m| execute_move(stacks, *m))
}

fn execute_move(stacks: &mut [Stack], (count, source, destination): (usize, usize, usize)) {
    for _ in 0..count {
        let moved_item = stacks[source - 1].pop().unwrap();
        stacks[destination - 1].push(moved_item);
    }
}

fn execute_plan_9001(stacks: &mut [Stack], moves: &[(usize, usize, usize)]) {
    moves.iter().for_each(|m| execute_move_9001(stacks, *m))
}

fn execute_move_9001(stacks: &mut [Stack], (count, source, destination): (usize, usize, usize)) {
    let moved_items = stacks[source - 1].pop_n(count);
    stacks[destination - 1].push_n(moved_items);
}

fn get_top_crates(stacks: &[Stack]) -> String {
    stacks
        .iter()
        .map(|stack| stack.peek().unwrap())
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star1() {
        let result = star1(TEST_INPUT);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn full_star1() {
        let result = star1(INPUT);
        assert_eq!(result, "WCZTHTMPS");
    }

    #[test]
    fn test_star2() {
        let result = star2(TEST_INPUT);
        assert_eq!(result, "MCD");
    }

    #[test]
    fn full_star2() {
        let result = star2(INPUT);
        assert_eq!(result, "BLSGJSDTS");
    }
}
