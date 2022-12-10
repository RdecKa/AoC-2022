#[allow(dead_code)]
const INPUT: &str = include_str!("./../input.txt");
#[allow(dead_code)]
const TEST_INPUT: &str = include_str!("./../test_input.txt");

type TreeGrid = Vec<Vec<u32>>;
type VisibilityGrid = Vec<Vec<bool>>;

fn main() {
    // Star 1
    let result = star1(INPUT);
    println!("Star 1: {}", result);

    // Star 2
    let result = star2(INPUT);
    println!("Star 2: {}", result);
}

fn star1(input: &str) -> usize {
    let tree_grid = parse_input(input);
    let mut visibility_grid = create_visibility_grid(tree_grid.len(), tree_grid[0].len());
    mark_visible_trees(&tree_grid, &mut visibility_grid);
    count_visible_trees_from_outside(&visibility_grid)
}

fn star2(input: &str) -> usize {
    let tree_grid = parse_input(input);
    get_max_score_tree(&tree_grid)
}

fn parse_input(input: &str) -> TreeGrid {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn create_visibility_grid(height: usize, width: usize) -> VisibilityGrid {
    let mut visibility_grid: VisibilityGrid = Vec::new();
    for _ in 0..height {
        let mut row = Vec::new();
        for _ in 0..width {
            row.push(false);
        }
        visibility_grid.push(row);
    }
    visibility_grid
}

fn mark_visible_trees(tree_grid: &TreeGrid, visibility_grid: &mut VisibilityGrid) {
    mark_visible_trees_from_left(tree_grid, visibility_grid);
    mark_visible_trees_from_right(tree_grid, visibility_grid);
    mark_visible_trees_from_top(tree_grid, visibility_grid);
    mark_visible_trees_from_bottom(tree_grid, visibility_grid);
}

fn mark_visible_trees_from_left(tree_grid: &TreeGrid, visibility_grid: &mut VisibilityGrid) {
    tree_grid.iter().enumerate().for_each(|(row_idx, row)| {
        visibility_grid[row_idx][0] = true;
        let mut last_visible_height = row[0];
        row.iter().enumerate().for_each(|(col_idx, &tree_height)| {
            if tree_height > last_visible_height {
                visibility_grid[row_idx][col_idx] = true;
                last_visible_height = tree_height;
            }
        })
    })
}

fn mark_visible_trees_from_right(tree_grid: &TreeGrid, visibility_grid: &mut VisibilityGrid) {
    tree_grid.iter().enumerate().for_each(|(row_idx, row)| {
        visibility_grid[row_idx][row.len() - 1] = true;
        let mut last_visible_height = row[row.len() - 1];
        row.iter()
            .enumerate()
            .rev()
            .for_each(|(col_idx, &tree_height)| {
                if tree_height > last_visible_height {
                    visibility_grid[row_idx][col_idx] = true;
                    last_visible_height = tree_height;
                }
            })
    })
}

fn mark_visible_trees_from_top(tree_grid: &TreeGrid, visibility_grid: &mut VisibilityGrid) {
    for col_idx in 0..tree_grid[0].len() {
        visibility_grid[0][col_idx] = true;
        let mut last_visible_height = tree_grid[0][col_idx];
        for row_idx in 0..tree_grid.len() {
            let tree_height = tree_grid[row_idx][col_idx];
            if tree_height > last_visible_height {
                visibility_grid[row_idx][col_idx] = true;
                last_visible_height = tree_height;
            }
        }
    }
}

fn mark_visible_trees_from_bottom(tree_grid: &TreeGrid, visibility_grid: &mut VisibilityGrid) {
    for col_idx in 0..tree_grid[0].len() {
        visibility_grid[tree_grid.len() - 1][col_idx] = true;
        let mut last_visible_height = tree_grid[tree_grid.len() - 1][col_idx];
        for row_idx in (0..tree_grid.len()).rev() {
            let tree_height = tree_grid[row_idx][col_idx];
            if tree_height > last_visible_height {
                visibility_grid[row_idx][col_idx] = true;
                last_visible_height = tree_height;
            }
        }
    }
}

fn count_visible_trees_from_outside(visibility_grid: &VisibilityGrid) -> usize {
    visibility_grid
        .iter()
        .map(|row| row.iter().filter(|&&visibile| visibile).count())
        .sum()
}

fn get_max_score_tree(tree_grid: &TreeGrid) -> usize {
    let mut best_score = 0;
    for row_idx in 0..tree_grid.len() {
        for col_idx in 0..tree_grid[0].len() {
            let score = get_tree_score(row_idx, col_idx, tree_grid);
            if score > best_score {
                best_score = score;
            }
        }
    }
    best_score
}

fn get_tree_score(row_idx: usize, col_idx: usize, tree_grid: &TreeGrid) -> usize {
    let ego_tree_height = tree_grid[row_idx][col_idx];
    let grid_height = tree_grid.len();
    let grid_width = tree_grid[0].len();

    // left
    let mut count_left = 0;
    for idx in (0..col_idx).rev() {
        count_left += 1;
        if tree_grid[row_idx][idx] >= ego_tree_height {
            break;
        }
    }

    // right
    let mut count_right = 0;
    for idx in col_idx + 1..grid_width {
        count_right += 1;
        if tree_grid[row_idx][idx] >= ego_tree_height {
            break;
        }
    }

    // up
    let mut count_up = 0;
    for idx in (0..row_idx).rev() {
        count_up += 1;
        if tree_grid[idx][col_idx] >= ego_tree_height {
            break;
        }
    }

    // down
    let mut count_down = 0;
    for idx in row_idx + 1..grid_height {
        count_down += 1;
        if tree_grid[idx][col_idx] >= ego_tree_height {
            break;
        }
    }

    count_left * count_right * count_up * count_down
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star1() {
        let result = star1(TEST_INPUT);
        assert_eq!(result, 21);
    }

    #[test]
    fn full_star1() {
        let result = star1(INPUT);
        assert_eq!(result, 1711);
    }

    #[test]
    fn test_star2() {
        let result = star2(TEST_INPUT);
        assert_eq!(result, 8);
    }

    #[test]
    fn full_star2() {
        let result = star2(INPUT);
        assert_eq!(result, 301392);
    }
}
