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
    String::from_iter(get_top_crates(&stacks))
}

fn star2(input: &str) -> i32 {
    let (stacks, moves) = parse_input(input);
    stacks.len() as i32
}

fn parse_input(input: &str) -> (Vec<Stack>, Vec<(i32, usize, usize)>) {
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
    let chars = row.chars().collect::<Vec<char>>();
    let columns = chars.chunks(4);
    for (idx, item) in columns.enumerate() {
        if item[1] != ' ' {
            stacks[idx].push(item[1]);
        }
    }
}

fn parse_moves_from_str(moves_str: &str) -> Vec<(i32, usize, usize)> {
    moves_str
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            split.next(); // "move"
            let count: i32 = split.next().unwrap().parse().unwrap();
            split.next(); // "from"
            let source: usize = split.next().unwrap().parse().unwrap();
            split.next(); // "to"
            let destination: usize = split.next().unwrap().parse().unwrap();
            (count, source, destination)
        })
        .collect()
}

fn execute_plan(stacks: &mut [Stack], moves: &[(i32, usize, usize)]) {
    moves.iter().for_each(|m| execute_move(stacks, *m))
}

fn execute_move(stacks: &mut [Stack], (count, source, destination): (i32, usize, usize)) {
    for _ in 0..count {
        let moved_item = &stacks[source - 1].pop().unwrap();
        stacks[destination - 1].push(*moved_item);
    }
}

fn get_top_crates(stacks: &[Stack]) -> Vec<char> {
    stacks.iter().map(|stack| stack.peek().unwrap()).collect()
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
        assert_eq!(result, 0);
    }

    #[test]
    fn full_star2() {
        let result = star2(INPUT);
        assert_eq!(result, 0);
    }
}
