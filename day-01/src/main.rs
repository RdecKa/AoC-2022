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
    let mut numbers = get_calories_per_elf(input);
    numbers.sort();

    // There is definitely at least one elf in the input
    numbers[numbers.len() - 1]
}

fn star2(input: &str) -> i32 {
    let mut numbers = get_calories_per_elf(input);
    numbers.sort();

    // There are definitely at least three elves in the input
    let len = numbers.len();
    numbers[len - 1] + numbers[len - 2] + numbers[len - 3]
}

fn get_calories_per_elf(input: &str) -> Vec<i32> {
    input
        .split("\n\n")
        .map(|inventory| count_calories_in_inventory(inventory))
        .collect()
}

fn count_calories_in_inventory(inventory: &str) -> i32 {
    inventory
        .lines()
        .map(|cal| cal.trim().parse::<i32>().unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star1() {
        let result = star1(TEST_INPUT);
        assert_eq!(result, 24000);
    }

    #[test]
    fn full_star1() {
        let result = star1(INPUT);
        assert_eq!(result, 72478);
    }

    #[test]
    fn test_star2() {
        let result = star2(TEST_INPUT);
        assert_eq!(result, 45000);
    }

    #[test]
    fn full_star2() {
        let result = star2(INPUT);
        assert_eq!(result, 210367);
    }
}
