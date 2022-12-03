use std::collections::HashSet;

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
    let parsed_input = parse_input_star1(input);
    get_sum_of_priorities(parsed_input)
}

fn star2(input: &str) -> i32 {
    let parsed_input = parse_input_star2(input);
    get_sum_of_badges(parsed_input)
}

fn parse_input_star1(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .map(|line| {
            let num_items = line.trim().len();
            (&line[..num_items / 2], &line[num_items / 2..])
        })
        .collect()
}

fn parse_input_star2(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn find_common_item(compartments: (&str, &str)) -> char {
    let compartment0 = HashSet::<_>::from_iter(compartments.0.chars());
    for item in compartments.1.chars() {
        if compartment0.contains(&item) {
            return item;
        }
    }
    panic!("No common item found");
}

fn get_item_priority(item: char) -> i32 {
    if 'A' <= item && item <= 'Z' {
        return 27 + (item as i32) - ('A' as i32);
    }
    1 + (item as i32) - ('a' as i32)
}

fn get_sum_of_priorities(input: Vec<(&str, &str)>) -> i32 {
    input
        .iter()
        .map(|rucksack| get_item_priority(find_common_item(*rucksack)))
        .sum()
}

fn get_badge(rucksacks: Vec<&str>) -> char {
    let intersection =
        rucksacks
            .iter()
            .skip(1)
            .fold(HashSet::from_iter(rucksacks[0].chars()), |acc, rucksack| {
                let rucksack_set: HashSet<char> = rucksack.chars().collect();
                acc.intersection(&rucksack_set).copied().collect()
            });
    intersection.into_iter().next().unwrap()
}

fn get_sum_of_badges(input: Vec<&str>) -> i32 {
    let mut sum = 0;
    for i in (0..input.len()).step_by(3) {
        let badge = get_badge(input[i..i + 3].to_vec());
        sum += get_item_priority(badge);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star1() {
        let result = star1(TEST_INPUT);
        assert_eq!(result, 157);
    }

    #[test]
    fn full_star1() {
        let result = star1(INPUT);
        assert_eq!(result, 7674);
    }

    #[test]
    fn test_star2() {
        let result = star2(TEST_INPUT);
        assert_eq!(result, 70);
    }

    #[test]
    fn full_star2() {
        let result = star2(INPUT);
        assert_eq!(result, 2805);
    }
}
