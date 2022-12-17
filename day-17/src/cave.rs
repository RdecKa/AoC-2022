use std::cmp;
use std::fmt;

// Row ////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct Row {
    pub occupancy: Vec<bool>,
}

impl fmt::Debug for Row {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.occupancy.iter().for_each(|o| {
            write!(f, "{}", if *o { "#" } else { "." });
        });
        write!(f, "")
    }
}

impl Row {
    fn new(size: usize) -> Row {
        Row {
            occupancy: vec![false; size],
        }
    }
}

// Pattern ////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct Pattern {
    pub rows: Vec<Row>, // First row is bottom row
}

impl fmt::Debug for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n");
        self.rows.iter().rev().for_each(|r| {
            write!(f, "{:?}\n", *r);
        });
        write!(f, "")
    }
}

impl Pattern {
    fn new() -> Pattern {
        Pattern { rows: Vec::new() }
    }

    fn is_occupied(&self, row: usize, col: usize) -> bool {
        self.rows[row].occupancy[col]
    }

    fn set(&mut self, row: usize, col: usize) {
        self.rows[row].occupancy[col] = true;
    }
}

// Cave ///////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct Cave {
    pub width: usize,
    pub height: usize, // Height of the top rock
    pub pattern: Pattern,
    pub falling_rock: Option<Rock>,
    pub falling_rock_x: usize,
    pub falling_rock_y: usize,
}

impl Cave {
    pub fn new(size: usize) -> Cave {
        Cave {
            width: size,
            height: 0,
            pattern: Pattern::new(),
            falling_rock: None,
            falling_rock_x: 0,
            falling_rock_y: 0,
        }
    }

    /**
     * Returns true if the falling_rock would overlap with the existing rocks
     * in the cave, if falling_rock was at position (pos_x, pos_y).
     */
    fn rock_overlaps(&self, pos_x: usize, pos_y: usize) -> bool {
        let falling_rock = self.falling_rock.as_ref().unwrap();
        for row_idx in 0..falling_rock.height {
            for col_idx in 0..falling_rock.width {
                if (self.pattern.is_occupied(pos_y + row_idx, pos_x + col_idx))
                    && (falling_rock.pattern.is_occupied(row_idx, col_idx))
                {
                    return true;
                }
            }
        }
        false
    }

    pub fn add_falling_rock(&mut self, rock: Rock) {
        self.falling_rock = Some(rock);
        self.falling_rock_x = 2;
        self.falling_rock_y = self.height + 3;
    }

    pub fn fix_falling_rock(&mut self) {
        let falling_rock = self.falling_rock.as_ref().unwrap();
        for row_idx in 0..falling_rock.height {
            for col_idx in 0..falling_rock.width {
                if falling_rock.pattern.is_occupied(row_idx, col_idx) {
                    if self
                        .pattern
                        .is_occupied(self.falling_rock_y + row_idx, self.falling_rock_x + col_idx)
                    {
                        println!("{:?}", self);
                        panic!("Already occupied");
                    }
                    self.pattern
                        .set(self.falling_rock_y + row_idx, self.falling_rock_x + col_idx);
                }
            }
        }

        self.height = cmp::max(self.height, self.falling_rock_y + falling_rock.height);
    }

    pub fn move_rock_left(&mut self) -> bool {
        // println!("< Moving left");
        if self.falling_rock_x == 0 {
            return false;
        }

        if self.rock_overlaps(self.falling_rock_x - 1, self.falling_rock_y) {
            return false;
        }

        self.falling_rock_x -= 1;
        true
    }

    pub fn move_rock_right(&mut self) -> bool {
        // println!("> Moving right");
        if self.falling_rock_x == self.width - self.falling_rock.as_ref().unwrap().width {
            return false;
        }

        if self.rock_overlaps(self.falling_rock_x + 1, self.falling_rock_y) {
            return false;
        }

        self.falling_rock_x += 1;
        true
    }

    pub fn move_rock_down(&mut self) -> bool {
        // println!("v Moving down");
        if self.falling_rock_y == 0 {
            return false;
        }

        if self.rock_overlaps(self.falling_rock_x, self.falling_rock_y - 1) {
            return false;
        }

        self.falling_rock_y -= 1;
        true
    }

    pub fn increase_cave_height(&mut self, needed_free_rows: usize) {
        let free_rows = self.pattern.rows.len() - self.height;
        if free_rows > needed_free_rows {
            return;
        }
        for _ in 0..(needed_free_rows - free_rows) {
            self.pattern.rows.push(Row::new(self.width));
        }
    }
}

// Rock ///////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct Rock {
    pub width: usize,
    pub height: usize,
    pub pattern: Pattern,
}

impl Rock {
    pub fn new(input: &str) -> Rock {
        let rows: Vec<Row> = input
            .lines()
            .rev()
            .map(|row| Row {
                occupancy: row.trim().chars().map(|c| c == '#').collect(),
            })
            .collect();
        let num_rows = rows.len();
        Rock {
            width: rows[0].occupancy.len(),
            height: num_rows,
            pattern: Pattern { rows },
        }
    }
}
