use std::sync::{Arc, RwLock};

pub mod board_solver {
    use std::sync::{Arc, RwLock};
    use std::thread::Thread;
    const DEBUG: bool = false;
    //#[derive(Clone, Copy)]
    struct Board {
        data: Vec<Vec<bool>>,
        size: usize,
    }
    //#[derive(Clone, Copy)]
    pub struct BoardSolver {
        board: Board,
        left_diagonal: Vec<bool>,
        right_diagonal: Vec<bool>,
        horizontal: Vec<bool>,
    }
    impl Board {
        pub fn new(size: usize) -> Self {
            Self {
                data: vec![vec![false; size]; size],
                size: size,
            }
        }
        pub fn set(&mut self, r: usize, c: usize) {
            self.data[r][c] = true;
        }
        pub fn unset(&mut self, r: usize, c: usize) {
            self.data[r][c] = false;
        }
        pub fn print(&self) {
            for row in &self.data {
                for item in row {
                    let val = if *item { 1 } else { 0 };
                    print!("{} ", val);
                }
                println!("");
            }
        }
    }
    impl BoardSolver {
        pub fn new(size: usize) -> Self {
            Self {
                board: Board::new(size as usize),
                left_diagonal: vec![false; 2 * size - 1],
                right_diagonal: vec![false; 2 * size - 1],
                horizontal: vec![false; size],
            }
        }
        fn get_right_diagonal(&self, r: usize, c: usize) -> usize {
            if c >= r {
                c - r
            } else {
                r - c + self.board.size - 1
            }
        }
        fn get_left_diagonal(&self, r: usize, c: usize) -> usize {
            let size = self.board.size;
            if r <= (size - 1) - c {
                size - 1 - c - r
            } else {
                r - ((size - 1) - c) + self.board.size - 1
            }
        }
        pub fn solve(&mut self, count_reference: &mut u128) {
            self.try_place(0, count_reference);
        }
        pub fn try_place(&mut self, r: usize, count: &mut u128) {
            let size = self.board.size;
            if r == size {
                *count += 1;
                if DEBUG {
                    self.board.print();
                    println!("");
                }
                return;
            }
            for column in 0..size {
                let can_place = self.valid_placement(r, column);
                if can_place {
                    self.horizontal[column] = true;
                    let left_diag_index = self.get_left_diagonal(r, column);
                    let right_diag_index = self.get_right_diagonal(r, column);

                    self.left_diagonal[left_diag_index] = true;
                    self.right_diagonal[right_diag_index] = true;
                    if DEBUG {
                        self.board.set(r, column);
                    }
                    self.try_place(r + 1, count);
                    if DEBUG {
                        self.board.unset(r, column);
                    }
                    self.left_diagonal[left_diag_index] = false;
                    self.right_diagonal[right_diag_index] = false;
                    self.horizontal[column] = false;
                }
            }
        }
        pub fn try_place_mt(&mut self, r: usize, count: Arc<RwLock<u128>>) {
            let size = self.board.size;
            if r == size {
                let clone = Arc::clone(&count);
                let mut val = clone.write().unwrap();
                *val += 1;
                if DEBUG {
                    self.board.print();
                    println!("");
                }
                return;
            }
            for column in 0..size {
                let can_place = self.valid_placement(r, column);
                if can_place {
                    self.horizontal[column] = true;
                    let left_diag_index = self.get_left_diagonal(r, column);
                    let right_diag_index = self.get_right_diagonal(r, column);

                    self.left_diagonal[left_diag_index] = true;
                    self.right_diagonal[right_diag_index] = true;
                    if DEBUG {
                        self.board.set(r, column);
                    }
                    self.try_place_mt(r + 1, Arc::clone(&count));
                    if DEBUG {
                        self.board.unset(r, column);
                    }
                    self.left_diagonal[left_diag_index] = false;
                    self.right_diagonal[right_diag_index] = false;
                    self.horizontal[column] = false;
                }
            }
        }
        fn valid_placement(&self, r: usize, c: usize) -> bool {
            if self.horizontal[c] == true {
                return false;
            }
            //let size = self.board.size;
            let left_diag_index = self.get_left_diagonal(r, c);
            let right_diag_index = self.get_right_diagonal(r, c);
            if self.left_diagonal[left_diag_index] || self.right_diagonal[right_diag_index] {
                return false;
            }
            true
        }
    }
}
/*mod board_solver_multithreaded{
    use crate::board_solver::BoardSolver;

  pub struct BoardSolverMultithreaded{
    board_solver:BoardSolver,
    thread_count:u16,
  }
  impl BoardSolverMultithreaded{
    pub fn new(size:usize)->Self{
      Self { board_solver: BoardSolver::new(size), thread_count: 1 }
    }
    pub fn with_thread_count(&self,count:u16)->Self{
      Self { board_solver: self.board_solver, thread_count: count }
    }
  }
}*/

use board_solver::BoardSolver;
fn main() {
    let mut bs = BoardSolver::new(8);
    let mut count = 0;
    bs.try_place(0, &mut count);
    println!("Solutions: {}", count);
    bs.try_place_mt(0, Arc::new(RwLock::new(count)));
    println!("Solutions: {}", count);
}
