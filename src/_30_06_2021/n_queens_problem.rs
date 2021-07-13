// The eight queens puzzle is the problem of placing eight chess queens
// on an 8×8 chessboard so that no two queens threaten each other.
// Thus, a solution requires that no two queens share the same row, column
// or diagonal. The eight queens puzzle is an example of the more general N
// queens problem of placing N non-attacking queens on an N×N chessboard.
// You can read about the problem on its Wikipedia page: Eight queens puzzle.
// You will receive a (possibly large) number N and have to place N queens on
// a N×N chessboard, so that no two queens attack each other. This requires
// that no two queens share the same row, column or diagonal. You will also
// receive the mandatory position of one queen. This position is given 0-based
// with 0 <= row < N and 0 <= col < N. The coordinates {0, 0} are in the top
// left corner of the board. For many given parameters multiple solutions are
// possible. You have to find one of the possible solutions, all that fit the
// requirements will be accepted. You have to return the solution board as a
// string, indicating empty fields with '.' (period) and Queens with 'Q'
// (uppercase Q), ending each row with '\n'. If no solution is possible for
// the given parameters, return None.

// https://en.wikipedia.org/wiki/Eight_queens_puzzle

// - input parameters are size and (mandatory_column, mandatory_row)
// - there are 8 tests for very small boards (N <= 10)
// - there are 8 tests for cases without solution
// - there are 5 tests for small boards (10 < N <= 50)
// - there are 5 tests for medium boards (100 < N <= 500)
// - there are 5 tests for large boards (500 < N <= 1000)

#[allow(dead_code)]
#[derive(Debug)]
pub struct NQueensPuzzle {
    board: Vec<bool>,
    n: usize,
}

impl NQueensPuzzle {
    #[allow(dead_code)]
    fn new(n: usize) -> Self {
        let mut b: Vec<bool> = Vec::new();
        let v: Vec<u32> = Self::random_spots(n);
        for i in 0..(n * n) {
            b.push(v.contains(&(i as u32)));
        }
        Self { board: b, n: n }
    }

    fn random_spots(n: usize) -> Vec<u32> {
        use rand::distributions::Standard;
        use rand::{thread_rng, Rng};
        let mut rng = thread_rng();
        loop {
            let v: Vec<u32> = (&mut rng)
                .sample_iter(Standard)
                .take(n)
                .collect::<Vec<f32>>()
                .iter()
                .map(|v| (v * (n * n) as f32).round() as u32)
                .collect::<Vec<u32>>();
            if v.len() == n {
                return v;
            }
        }
    }
    #[allow(dead_code)]
    fn place_queens(&mut self, algo: fn(usize) -> Vec<(u32, u32)>) {
        let indexes = algo(self.n)
            .iter()
            .map(|v| self.coord_to_index(v.0, v.1).unwrap())
            .collect::<Vec<usize>>();
        for v in indexes {
            self.board[v] = true;
        }
    }

    #[allow(dead_code)]
    fn print_table(&self) {
        let s = self.to_string();
        let mut arr = s.split("\n").into_iter().map(|v| v).collect::<Vec<&str>>();
        arr = arr[1..arr.len()].to_vec();
        for item in arr.iter() {
            println!("{:?}", item);
        }
    }

    #[allow(dead_code)]
    fn coord_to_index(&self, x: u32, y: u32) -> Option<usize> {
        match (x + self.n as u32 * y) as usize {
            i if (i < self.board.len()) => Some(i),
            _ => None,
        }
    }

    #[allow(dead_code)]
    fn get_row(&self, row: usize) -> Vec<bool> {
        if row >= self.board.len() {
            panic!("invalid row!");
        }
        let range = 0..self.board.len();
        let mut res: Vec<bool> = vec![];
        for i in range {
            let val: bool = self.board[self.coord_to_index(i as u32, row as u32).unwrap()];
            res.push(val);
        }
        res
    }

    #[allow(dead_code)]
    #[allow(unused_variables)]
    fn solve(&self, mandatory_coords: (usize, usize)) -> Option<String> {
        Some("".to_string())
    }
}

impl ToString for NQueensPuzzle {
    fn to_string(&self) -> String {
        let mut res: String = String::from("");
        for i in 0..self.board.len() {
            let is_queens: bool = self.board[i];
            if i % self.n == 0 {
                res.push_str("\n");
            }
            res.push_str({
                if is_queens {
                    "Q"
                } else {
                    "."
                }
            });
        }
        res
    }
}

use std::fmt::{Debug, Display};

trait Printable<T: ToString + Display + Debug> {
    fn print(&self) {
        // let s = self.to_string();
        // println!("{:?}", self);
    }
}

#[allow(dead_code)]
pub fn main() {
    let q = NQueensPuzzle::new(3);
    q.print_table();
}

pub mod sigbbe {
    pub mod set {
        #[allow(dead_code)]
        struct Set<T: Eq + PartialOrd + ?Sized + 'static> {
            collection: &'static T,
            len: usize,
        }
    }
}
