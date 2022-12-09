use std::collections::HashMap;

#[allow(dead_code)]
const INPUT: &str = include_str!("./../input.txt");
#[allow(dead_code)]
const TEST_INPUT_1: &str = include_str!("./../test_input_1.txt");
#[allow(dead_code)]
const TEST_INPUT_2: &str = include_str!("./../test_input_2.txt");
#[allow(dead_code)]
const TEST_INPUT_3: &str = include_str!("./../test_input_3.txt");
#[allow(dead_code)]
const TEST_INPUT_4: &str = include_str!("./../test_input_4.txt");
#[allow(dead_code)]
const TEST_INPUT_5: &str = include_str!("./../test_input_5.txt");

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
    get_marker_location(&parsed_input).unwrap()
}

fn star2(input: &str) -> i32 {
    let parsed_input = parse_input(input);
    parsed_input.len() as i32
}

fn parse_input(input: &str) -> Vec<char> {
    input.trim().chars().collect()
}

fn get_marker_location(signal: &[char]) -> Option<usize> {
    let mut signal_iter_end = signal.iter(); // End of marker
    let mut recent_chars: HashMap<char, usize> = HashMap::new();
    // Insert first three chars
    for _ in 0..3 {
        let current_char = signal_iter_end.next().unwrap();
        add_single_char_to_recent_chars(*current_char, &mut recent_chars);
    }

    let mut signal_iter_start = signal.iter(); // Start of marker
    for (idx, current_char) in signal_iter_end.enumerate() {
        add_single_char_to_recent_chars(*current_char, &mut recent_chars);
        if recent_chars.len() == 4 {
            // +3 to compensate for the first three chars
            // +1 to compensate starting to count with 0
            return Some(idx + 4);
        }
        // Remove the oldest char from the map
        let oldest_char = signal_iter_start.next().unwrap();
        remove_single_char_from_recent_chars(*oldest_char, &mut recent_chars);
    }
    None
}

fn add_single_char_to_recent_chars(c: char, recent_chars: &mut HashMap<char, usize>) {
    recent_chars.insert(
        c,
        match recent_chars.get(&c) {
            Some(c) => c + 1,
            None => 1,
        },
    );
}

fn remove_single_char_from_recent_chars(c: char, recent_chars: &mut HashMap<char, usize>) {
    let count = recent_chars.get(&c).unwrap();
    match count {
        1 => {
            recent_chars.remove(&c);
        }
        _ => {
            recent_chars.insert(c, count - 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_star1() {
        let result = star1(TEST_INPUT_1);
        assert_eq!(result, 7);
    }
    #[test]
    fn test2_star1() {
        let result = star1(TEST_INPUT_2);
        assert_eq!(result, 5);
    }
    #[test]
    fn test3_star1() {
        let result = star1(TEST_INPUT_3);
        assert_eq!(result, 6);
    }
    #[test]
    fn test4_star1() {
        let result = star1(TEST_INPUT_4);
        assert_eq!(result, 10);
    }
    #[test]
    fn test5_star1() {
        let result = star1(TEST_INPUT_5);
        assert_eq!(result, 11);
    }
    #[test]
    fn full_star1() {
        let result = star1(INPUT);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_star2() {
        let result = star2(TEST_INPUT_1);
        assert_eq!(result, 0);
    }

    #[test]
    fn full_star2() {
        let result = star2(INPUT);
        assert_eq!(result, 0);
    }
}
