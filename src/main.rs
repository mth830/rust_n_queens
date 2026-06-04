#![allow(unused)]
pub mod board_solver {
    const DEBUG: bool = false;
    #[derive(Clone, Copy)]
    pub struct Board<const N: usize> {
        data: [[bool; N]; N],
        size: usize,
    }
    #[derive(Clone, Copy)]
    pub struct BoardSolver<const N: usize> {
        board: Board<N>,
        left_diagonal: [[bool; N]; 2],
        right_diagonal: [[bool; N]; 2],
        horizontal: [bool; N],
    }

    impl<const N: usize> Board<N> {
        pub fn new() -> Self {
            Self {
                data: [[false; N]; N],
                size: N,
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
            println!("");
        }
    }

    impl<const N: usize> BoardSolver<N> {
        pub fn new() -> Self {
            assert!(N > 0, "N must be 1 or above");
            Self {
                board: Board::<N>::new(),
                left_diagonal: [[false; N]; 2],
                right_diagonal: [[false; N]; 2],
                horizontal: [false; N],
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
        pub fn solve(&mut self) -> u128 {
            self.try_place(0)
        }
        pub fn try_place(&mut self, row: usize) -> u128 {
            let size = N;
            if row == size {
                if DEBUG {
                    self.board.print();
                    println!("");
                }
                return 1;
            }
            let mut count = 0;
            for column in 0..size {
                let can_place = self.valid_placement(row, column);
                if can_place {
                    self.set_position(row, column, true);

                    if DEBUG {
                        self.board.set(row, column);
                    }

                    count += self.try_place(row + 1);

                    if DEBUG {
                        self.board.unset(row, column);
                    }

                    self.set_position(row, column, false);
                }
            }
            count
        }
        pub fn set_board(&mut self, board: Board<N>) {
            self.board = board;
        }
        pub fn valid_placement(&self, r: usize, c: usize) -> bool {
            if self.horizontal[c] == true {
                return false;
            }
            let left_diag_index = self.get_left_diagonal(r, c);
            let right_diag_index = self.get_right_diagonal(r, c);

            let left_diag_row = left_diag_index / N;
            let left_diag_col = left_diag_index % N;

            let right_diag_row = right_diag_index / N;
            let right_diag_col = right_diag_index % N;

            if self.left_diagonal[left_diag_row][left_diag_col]
                || self.right_diagonal[right_diag_row][right_diag_col]
            {
                return false;
            }
            true
        }
        pub fn set_position(&mut self, r: usize, c: usize, state: bool) {
            self.horizontal[c] = state;
            let left_diag_index = self.get_left_diagonal(r, c);
            let left_diag_row = left_diag_index / N;
            let left_diag_col = left_diag_index % N;

            let right_diag_index = self.get_right_diagonal(r, c);
            let right_diag_row = right_diag_index / N;
            let right_diag_col = right_diag_index % N;

            self.left_diagonal[left_diag_row][left_diag_col] = state;
            self.right_diagonal[right_diag_row][right_diag_col] = state;
        }
    }
}
mod board_solver_multithreaded {
    use crate::board_solver::BoardSolver;
    use std::{sync::Arc, sync::RwLock, thread};
    pub struct BoardSolverMultithreaded<const N: usize> {
        thread_count: u32,
    }
    impl<const N: usize> BoardSolverMultithreaded<N> {
        pub fn new() -> Self {
            let default_thread_count = 8_u32;
            assert!(N > 0, "N must be 1 or above");
            Self {
                thread_count: default_thread_count,
            }
        }
        pub fn with_thread_count(thread_count: u32) -> Self {
            assert!(thread_count > 0, "thread_count must be 1 or above");
            assert!(N > 0, "N must be 1 or above");
            Self {
                thread_count: thread_count,
            }
        }
        fn get_board_state(state_number: usize) -> Option<BoardSolver<N>> {
            assert!(
                state_number < N * N,
                "state_number must be within the range 0 to {}",
                N * N - 1
            );
            let mut bs = BoardSolver::<N>::new();
            //gets the first and second rows column positions
            let row_0_index = state_number / N;
            let row_1_index = state_number % N;
            bs.set_position(0, row_0_index, true);
            if bs.valid_placement(1, row_1_index) {
                bs.set_position(1, row_1_index, true);
                return Some(bs);
            } else {
                return None;
            }
        }
        pub fn solve_single_state(state_counter: Arc<RwLock<usize>>, limit: usize) -> u128 {
            let state_number_lock = Arc::clone(&state_counter);
            let state_number = {
                let mut state_number_writer = state_number_lock.write().unwrap();
                let original_state_number = state_number_writer.clone();
                *state_number_writer += 1;
                original_state_number
            };
            if state_number >= limit {
                return 0;
            }
            let mut result = 0_u128;
            if let Some(mut bs) = Self::get_board_state(state_number) {
                result = bs.try_place(2);
            }

            result + Self::solve_single_state(state_counter, limit)
        }

        pub fn solve(&self) -> u128 {
            if N == 1 {
                return 1;
            }
            let mut handles = vec![];
            let mut results: Vec<_> = vec![];
            let state_counter = Arc::new(RwLock::new(0_usize));
            (0..self.thread_count).for_each(|_| {
                let state_counter_clone = Arc::clone(&state_counter);
                let handle = thread::spawn(|| Self::solve_single_state(state_counter_clone, N * N));
                handles.push(handle);
            });
            for handle in handles {
                let result = handle.join().unwrap();
                results.push(result);
            }
            results.iter().sum()
        }
    }
}

use board_solver_multithreaded::BoardSolverMultithreaded;
fn main() {
    let bsmt = BoardSolverMultithreaded::<14>::with_thread_count(14);
    let count = bsmt.solve();
    println!("count: {}", count);
}
mod tests {

    #[cfg(test)]
    use super::*;
    use crate::board_solver::BoardSolver;
    #[test]
    fn works_for_n_1_to_4() {
        let mut bs = BoardSolver::<1>::new();
        let count = bs.solve();
        assert!(count == 1, "solution for 1 invalid");

        let mut bs = BoardSolver::<2>::new();
        let count = bs.solve();
        assert!(count == 0, "solution for 2 invalid");

        let mut bs = BoardSolver::<3>::new();
        let count = bs.solve();
        assert!(count == 0, "solution for 3 invalid");

        let mut bs = BoardSolver::<4>::new();
        let count = bs.solve();
        assert!(count == 2, "solution for 4 invalid");
    }

    #[test]
    fn works_for_n_5_to_8() {
        let mut bs = BoardSolver::<5>::new();
        let count = bs.solve();
        assert!(count == 10, "solution for 5 invalid");

        let mut bs = BoardSolver::<6>::new();
        let count = bs.solve();
        assert!(count == 4, "solution for 6 invalid");

        let mut bs = BoardSolver::<7>::new();
        let count = bs.solve();
        assert!(count == 40, "solution for 7 invalid");

        let mut bs = BoardSolver::<8>::new();
        let count = bs.solve();
        assert!(count == 92, "solution for 8 invalid");
    }
    #[test]
    fn works_for_n_1_to_4_multithreaded() {
        let bsmt = BoardSolverMultithreaded::<1>::new();
        let count = bsmt.solve();
        assert!(count == 1, "solution for 1 invalid");

        let bsmt = BoardSolverMultithreaded::<2>::new();
        let count = bsmt.solve();
        assert!(count == 0, "solution for 2 invalid");

        let bsmt = BoardSolverMultithreaded::<3>::new();
        let count = bsmt.solve();
        assert!(count == 0, "solution for 3 invalid");

        let bsmt = BoardSolverMultithreaded::<4>::new();
        let count = bsmt.solve();
        assert!(count == 2, "solution for 4 invalid");
    }
    #[test]
    fn works_for_n_5_to_8_multithreaded() {
        let bsmt = BoardSolverMultithreaded::<5>::new();
        let count = bsmt.solve();
        assert!(count == 10, "solution for 5 invalid");

        let bsmt = BoardSolverMultithreaded::<6>::new();
        let count = bsmt.solve();
        assert!(count == 4, "solution for 6 invalid");

        let bsmt = BoardSolverMultithreaded::<7>::new();
        let count = bsmt.solve();
        assert!(count == 40, "solution for 7 invalid");

        let bsmt = BoardSolverMultithreaded::<8>::new();
        let count = bsmt.solve();
        assert!(count == 92, "solution for 8 invalid");
    }
    #[test]
    fn works_for_n_9_to_12_multithreaded() {
        let bsmt = BoardSolverMultithreaded::<9>::new();
        let count = bsmt.solve();
        assert!(count == 352, "solution for 9 invalid");

        let bsmt = BoardSolverMultithreaded::<10>::new();
        let count = bsmt.solve();
        assert!(count == 724, "solution for 10 invalid");

        let bsmt = BoardSolverMultithreaded::<11>::new();
        let count = bsmt.solve();
        assert!(count == 2680, "solution for 11 invalid");

        let bsmt = BoardSolverMultithreaded::<12>::new();
        let count = bsmt.solve();
        assert!(count == 14200, "solution for 12 invalid");
    }
}
