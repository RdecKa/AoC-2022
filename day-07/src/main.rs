mod node;

use node::Tree;
use regex::Regex;

#[allow(dead_code)]
const INPUT: &str = include_str!("./../input.txt");
#[allow(dead_code)]
const TEST_INPUT: &str = include_str!("./../test_input.txt");
#[allow(dead_code)]
const TEST_INPUT_2: &str = include_str!("./../test_input_2.txt");

fn main() {
    // Star 1
    let result = star1(INPUT);
    println!("Star 1: {}", result);

    // Star 2
    let result = star2(INPUT);
    println!("Star 2: {}", result);
}

fn star1(input: &str) -> i32 {
    let mut file_system = parse_input(input);
    file_system.calculate_node_sizes("/");
    file_system.sum_small_directories()
}

fn star2(input: &str) -> i32 {
    let mut file_system = parse_input(input);
    file_system.calculate_node_sizes("/");
    file_system.select_dir_to_free_space()
}

fn parse_input(input: &str) -> Tree {
    let re_ls = Regex::new(r"\$ ls").unwrap();
    let re_cd_root = Regex::new(r"\$ cd /").unwrap();
    let re_cd = Regex::new(r"\$ cd ([[:alpha:]]+)").unwrap();
    let re_cd_dot = Regex::new(r"\$ cd ..").unwrap();
    let re_dir = Regex::new(r"dir ([[:alpha:]]+)").unwrap();
    let re_file = Regex::new(r"([0-9]+) (([[:alpha:]]|.)+)").unwrap();

    let mut file_system = Tree::new();
    let mut pwd: String = "/".to_string();

    input.lines().for_each(|line| {
        let matched_ls = re_ls.captures(line);
        if matched_ls.is_some() {
            return;
        }
        let matched_cd_root = re_cd_root.captures(line);
        if matched_cd_root.is_some() {
            pwd = "/".to_string();
            return;
        }
        let matched_cd = re_cd.captures(line);
        if matched_cd.is_some() {
            pwd += matched_cd.unwrap().get(1).unwrap().as_str();
            pwd += "/";
            return;
        }
        let matched_cd_dot = re_cd_dot.captures(line);
        if matched_cd_dot.is_some() {
            let split: Vec<&str> = pwd.rsplitn(3, "/").collect();
            pwd = split[2].to_string() + "/";
            return;
        }
        let matched_dir = re_dir.captures(line);
        if matched_dir.is_some() {
            let name = matched_dir.unwrap().get(1).unwrap().as_str();
            file_system.create_dir(&pwd, name);
            return;
        }
        let matched_file = re_file.captures(line);
        if let Some(matched) = matched_file {
            let name = matched.get(2).unwrap().as_str();
            let size = matched.get(1).unwrap().as_str().parse().unwrap();
            file_system.create_file(&pwd, name, size);
            return;
        }
        panic!("Invalid command {}", line);
    });
    file_system
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star1() {
        let result = star1(TEST_INPUT);
        assert_eq!(result, 95437);
    }

    #[test]
    fn test2_star1() {
        let result = star1(TEST_INPUT_2);
        assert_eq!(result, 95437);
    }

    #[test]
    fn full_star1() {
        let result = star1(INPUT);
        assert_eq!(result, 1350966);
    }

    #[test]
    fn test_star2() {
        let result = star2(TEST_INPUT);
        assert_eq!(result, 24933642);
    }

    #[test]
    fn test2_star2() {
        let result = star2(TEST_INPUT_2);
        assert_eq!(result, 24933642);
    }

    #[test]
    fn full_star2() {
        let result = star2(INPUT);
        assert_eq!(result, 6296435);
    }
}
