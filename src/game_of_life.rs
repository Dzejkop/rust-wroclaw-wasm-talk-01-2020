#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[repr(u8)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Alive,
    Dead,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Debug, Clone)]
/// A cell comes to life if it's number of neighbors is in `birth`
/// it survives if it's number of neighbors is in `survival`
/// otherwise the cell is dead
///
/// Can be parsed from standard syntax "B3/S23"
pub struct Rule {
    birth: Vec<u32>,
    survival: Vec<u32>,
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn validate_rule(s: &str) -> bool {
    s.parse::<Rule>().is_ok()
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct GameOfLife {
    width: u32,
    height: u32,
    board: Vec<Cell>,
    rule: Rule,
}

/// Methods not exported with wasm_bindgen
impl GameOfLife {
    fn new(rule: Rule, seed: u32, width: u32, height: u32) -> Self {
        use rand::{Rng, SeedableRng};

        // construct a seed out of a repeating u32
        let seed = seed as u128;
        let seed: u128 = seed + (seed << 8) + (seed << 16) + (seed << 24);
        let seed = seed.to_le_bytes();

        // having "real" randomness is actually quite complex
        // and using rand's default thread_rng requires a whole bunch of sys calls
        // to avoid that we'll use a pseudo random generator
        // and have the seed be provided from the host env
        let mut rng = rand::rngs::SmallRng::from_seed(seed);

        let mut board = vec![Cell::Dead; (width * height) as usize];
        for cell in board.iter_mut() {
            *cell = if rng.gen::<f32>() > 0.5 {
                Cell::Alive
            } else {
                Cell::Dead
            };
        }

        Self {
            width,
            height,
            board,
            rule,
        }
    }

    /// Cells are in linear memory, to access them by x and y
    /// we need to transform those coordinates to a linear index
    ///
    /// 0, 1, 2
    /// 3, 4, 5
    ///
    /// is actually
    /// 0, 1, 2, 3, 4, 5
    ///
    /// index of 3 is transform(x: 0, y: 1) -> y * width + x -> 1 * 3 + 0 -> 3
    fn translate(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }

    fn neighbors(&self, x: u32, y: u32) -> Vec<Cell> {
        #[rustfmt::skip]
        const OFFSETS: [(i64, i64); 8] = [
            ( 1,  0), // right
            (-1,  0), // left
            ( 0,  1), // top
            ( 0, -1), // bottom
            (-1, -1), // left bottom
            (-1,  1), // left top
            ( 1,  1), // right top
            ( 1, -1), // right bottom
        ];

        OFFSETS
            .iter()
            .filter_map(|(off_x, off_y)| {
                let x = x as i64;
                let y = y as i64;

                // underflow protection
                if x == 0 && *off_x < 0 {
                    return None;
                }

                // underflow protection
                if y == 0 && *off_y < 0 {
                    return None;
                }

                self.get((x + off_x) as u32, (y + off_y) as u32)
            })
            .collect()
    }
}

/// Methods for which binding will be generated using wasm_bindgen
#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl GameOfLife {
    /// Constructs a GameOfLife using the provided rule string
    /// panics if the rule string is invalid
    pub fn new_with_seed_and_rules(
        rule: String,
        seed: u32,
        width: u32,
        height: u32,
    ) -> Self {
        let rule: Rule = rule.parse().unwrap();
        Self::new(rule, seed, width, height)
    }

    /// Constructs a GameOfLife with default rule
    pub fn new_with_seed(seed: u32, width: u32, height: u32) -> Self {
        let rule = Rule {
            birth: vec![3],
            survival: vec![2, 3],
        };

        Self::new(rule, seed, width, height)
    }

    /// Get the value of a cell at coords (x, y)
    pub fn get(&self, x: u32, y: u32) -> Option<Cell> {
        self.board.get(self.translate(x, y)).copied()
    }

    /// Access to linear memory of cells
    pub fn cells(&self) -> *const Cell {
        self.board.as_ptr()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn tick(&mut self) {
        let mut new_board = self.board.clone();

        for x in 0..self.width {
            for y in 0..self.height {
                if let Some(cell) = self.get(x, y) {
                    let neighbors = self.neighbors(x, y);
                    let live_neighbors_count = neighbors
                        .into_iter()
                        .filter(|c| *c == Cell::Alive)
                        .count()
                        as u32;

                    let new_cell = match cell {
                        Cell::Alive => {
                            if self
                                .rule
                                .survival
                                .iter()
                                .any(|n| *n == live_neighbors_count)
                            {
                                Cell::Alive
                            } else {
                                Cell::Dead
                            }
                        }
                        Cell::Dead => {
                            if self
                                .rule
                                .birth
                                .iter()
                                .any(|n| *n == live_neighbors_count)
                            {
                                Cell::Alive
                            } else {
                                Cell::Dead
                            }
                        }
                    };

                    let i = self.translate(x, y);
                    new_board[i] = new_cell;
                }
            }
        }

        self.board = new_board;
    }
}

mod from_str {
    use super::Rule;
    use std::str::FromStr;

    fn parse_rule_nums(s: &str) -> Result<Vec<u32>, String> {
        s.chars()
            .map(|c: char| {
                let s = format!("{}", c);
                u32::from_str(&s)
            })
            .map(|r| r.map_err(|err| err.to_string()))
            .collect::<Result<_, _>>()
    }

    /// Parse from a standardized syntax
    /// B3/S23 meaning Birth [3] Survival [2, 3]
    impl FromStr for Rule {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut parts = s.splitn(2, '/');

            let birth: &str = parts.next().unwrap();
            let survival = parts
                .next()
                .ok_or_else(|| String::from("Expected 2 parts to the rule"))?;

            if !birth.starts_with('B') {
                return Err(String::from("Missing birth"));
            }

            if !survival.starts_with('S') {
                return Err(String::from("Missing survival"));
            }

            let birth: Vec<u32> = parse_rule_nums(&birth[1..])?;
            let survival: Vec<u32> = parse_rule_nums(&survival[1..])?;

            Ok(Rule { birth, survival })
        }
    }
}
