use std::sync::{Arc, RwLock};

pub mod board_solver {
    use std::sync::{Arc, RwLock};
    //use std::thread::Thread;
    const DEBUG: bool = false;

    struct Board<const N: usize> {
        data: [[bool; N]; N],
        size: usize,
    }

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
        }
    }

    impl<const N: usize> BoardSolver<N> {
        pub fn new() -> Self {
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
                    let left_diag_row = left_diag_index / N;
                    let left_diag_col = left_diag_index % N;

                    let right_diag_index = self.get_right_diagonal(r, column);
                    let right_diag_row = right_diag_index / N;
                    let right_diag_col = right_diag_index % N;

                    self.left_diagonal[left_diag_row][left_diag_col] = true;
                    self.right_diagonal[right_diag_row][right_diag_col] = true;

                    if DEBUG {
                        self.board.set(r, column);
                    }

                    self.try_place(r + 1, count);

                    if DEBUG {
                        self.board.unset(r, column);
                    }

                    self.left_diagonal[left_diag_row][left_diag_col] = false;
                    self.right_diagonal[right_diag_row][right_diag_col] = false;
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
                    let left_diag_row = left_diag_index / N;
                    let left_diag_col = left_diag_index % N;

                    let right_diag_index = self.get_right_diagonal(r, column);
                    let right_diag_row = right_diag_index / N;
                    let right_diag_col = right_diag_index % N;

                    self.left_diagonal[left_diag_row][left_diag_col] = true;
                    self.right_diagonal[right_diag_row][right_diag_col] = true;

                    if DEBUG {
                        self.board.set(r, column);
                    }

                    self.try_place_mt(r + 1, Arc::clone(&count));

                    if DEBUG {
                        self.board.unset(r, column);
                    }

                    self.left_diagonal[left_diag_row][left_diag_col] = false;
                    self.right_diagonal[right_diag_row][right_diag_col] = false;
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
    let mut bs = BoardSolver::<8>::new();
    let mut count = 0;
    bs.try_place(0, &mut count);
    println!("Solutions: {}", count);

    let  count =0;
    let arc_count =Arc::new(RwLock::new(count));
    bs.try_place_mt(0, Arc::clone(&arc_count));
    println!("Solutions: {}", arc_count.read().unwrap());
}
mod tests {
    #[cfg(test)]
    use super::*;
    #[test]
    fn works_for_n_1_to_4() {
        let mut bs = BoardSolver::<1>::new();
        let mut count = 0;
        bs.solve(&mut count);
        assert!(count == 1, "solution for 1 invalid");

        let mut bs = BoardSolver::<2>::new();
        let mut count = 0;
        bs.solve(&mut count);
        assert!(count == 0, "solution for 2 invalid");

        let mut bs = BoardSolver::<3>::new();
        let mut count = 0;
        bs.solve(&mut count);
        assert!(count == 0, "solution for 3 invalid");

        let mut bs = BoardSolver::<4>::new();
        let mut count = 0;
        bs.solve(&mut count);
        assert!(count == 2, "solution for 4 invalid");
    }

    #[test]
    fn works_for_n_5_to_8() {
        let mut bs = BoardSolver::<5>::new();
        let mut count = 0;
        bs.solve(&mut count);
        assert!(count == 10, "solution for 5 invalid");

        let mut bs = BoardSolver::<6>::new();
        let mut count = 0;
        bs.solve(&mut count);
        assert!(count == 4, "solution for 6 invalid");

        let mut bs = BoardSolver::<7>::new();
        let mut count = 0;
        bs.solve(&mut count);
        assert!(count == 40, "solution for 7 invalid");

        let mut bs = BoardSolver::<8>::new();
        let mut count = 0;
        bs.solve(&mut count);
        assert!(count == 92, "solution for 8 invalid");
    }
    #[test]
    fn works_for_n_1_to_4_multithreaded() {
        let mut bs = BoardSolver::<1>::new();
        let count= Arc::<_>::new(RwLock::new(0));
        bs.try_place_mt(0,Arc::clone(&count));
        let count = *count.read().unwrap();
        assert!(count == 1, "solution for 1 invalid");

        let mut bs = BoardSolver::<2>::new();
         let count= Arc::<_>::new(RwLock::new(0));
        bs.try_place_mt(0,Arc::clone(&count));
        let count = *count.read().unwrap();
        assert!(count  == 0, "solution for 2 invalid");

        let mut bs = BoardSolver::<3>::new();
         let count= Arc::<_>::new(RwLock::new(0));
        bs.try_place_mt(0,Arc::clone(&count));
        let count = *count.read().unwrap();
        assert!(count  == 0, "solution for 3 invalid");

        let mut bs = BoardSolver::<4>::new();
         let count= Arc::<_>::new(RwLock::new(0));
        bs.try_place_mt(0,Arc::clone(&count));
        let count = *count.read().unwrap();
        assert!(count  == 2, "solution for 4 invalid");
    }

}
