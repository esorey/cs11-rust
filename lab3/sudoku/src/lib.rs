use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

/// A struct representing a sudoku board.
pub struct Sudoku {
    board: [[u8; 9]; 9],
}

impl Sudoku {
    /// Load a sudoku board from a file
    pub fn load(file: &str) -> std::io::Result<Self> {
        let f = File::open(file)?;
        let reader = BufReader::new(f);
        let mut lines = reader.lines();

        let mut board = [[0u8; 9]; 9];

        for i in 0..9 {
            let line = lines.next().unwrap().unwrap();
            let mut chars = line.chars();
            for j in 0..9 {
                let char = chars.next().unwrap();
                if char == ' ' {
                    board[i][j] = 0u8;
                } else {
                    board[i][j] = char.to_string().parse::<u8>().unwrap();
                }
            }
        }
        Ok(Self { board })
    }

    /// Solve the sudoku board (in-place)
    pub fn solve(&mut self) {
        self.is_solvable();
    }

    fn is_solvable(&mut self) -> bool {
        let first_empty = self.get_first_empty_cell();

        if first_empty.is_none() {
            return true;
        }

        let (r, c) = first_empty.unwrap();

        let options = self.get_possible(r, c);
        if options.is_empty() {
            return false;
        }

        for num in options {
            // Try the option.
            self.board[r][c] = num;
            if !self.is_solvable() {
                // Undo the option.
                self.board[r][c] = 0u8;
                continue;
            }
            return true;
        }
        false
    }

    fn get_first_empty_cell(&self) -> Option<(usize, usize)> {
        // 9usize is out-of-bounds; use it as a placeholder.
        let mut r = 9usize;
        let mut c = 9usize;
        for i in 0..9 {
            // sum(1..=9) == 45, so that is the predicate for checking for a row with a missing number.
            if self.board[i].iter().sum::<u8>() < 45 {
                r = i;
                break;
            }
        }
        if r == 9usize {
            return None;
        }

        for j in 0..9 {
            if self.board[r][j] == 0 {
                c = j;
                break;
            }
        }
        if c == 9usize {
            return None;
        }

        Some((r, c))
    }

    fn get_row(&self, i: usize) -> Vec<u8> {
        self.board[i].to_vec()
    }

    fn get_col(&self, j: usize) -> Vec<u8> {
        (0..9).map(|x| self.board[x][j]).collect()
    }

    fn get_subgrid(&self, i: usize, j: usize) -> Vec<u8> {
        let row_start: usize = if i < 3 {
            0
        } else if i < 6 {
            3
        } else {
            6
        };

        let col_start: usize = if j < 3 {
            0
        } else if j < 6 {
            3
        } else {
            6
        };

        let mut result: Vec<u8> = vec![];
        for x in row_start..=row_start + 2 {
            for y in col_start..=col_start + 2 {
                result.push(self.board[x][y]);
            }
        }
        // With iterators: too gnarly
        // let result: Vec<u8> = (row_start..=row_start + 2)
        //     .map(|r| {
        //         (col_start..=col_start + 2)
        //             .map(|c| self.board[r][c])
        //             .collect::<Vec<u8>>()
        //     })
        //     .fold(vec![], |mut acc, mut subvec| {
        //         acc.append(&mut subvec);
        //         acc
        //     });
        result
    }

    fn get_possible(&self, i: usize, j: usize) -> Vec<u8> {
        let row = self.get_row(i);
        let col = self.get_col(j);
        let subgrid = self.get_subgrid(i, j);
        let mut possible = vec![];
        for option in 1u8..=9 {
            if !row.contains(&option) && !col.contains(&option) && !subgrid.contains(&option) {
                possible.push(option);
            }
        }
        possible
    }
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "┌───────┬───────┬───────┐")?;
        for (i, rowchunk) in self.board.chunks(3).enumerate() {
            for row in rowchunk.iter() {
                for block in row.chunks(3) {
                    write!(f, "│ ")?;
                    for &num in block.iter() {
                        if num == 0 {
                            write!(f, "  ")?;
                        } else {
                            write!(f, "{} ", num)?;
                        }
                    }
                }
                writeln!(f, "│")?;
            }
            if i != 2 {
                writeln!(f, "├───────┼───────┼───────┤")?;
            }
        }
        write!(f, "└───────┴───────┴───────┘")
    }
}
