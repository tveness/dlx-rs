//!
//! dlx_rs is a  Rust library for solving exact cover/constraint problems
//! problems using Knuth's [Dancing Links](https://en.wikipedia.org/wiki/Dancing_Links) (DLX) algorithm.
//!
//! It also provides specific interfaces for some common exact cover problems,
//! specifically:
//!
//! * arbitrary Sudokus
//! * N queens problem
//! * Pentomino tilings (TODO)
//! * graph colouring (TODO)
//!
//!
//! ## Setting up a general constraint problem
//!
//! A constraint problem may be expressed in terms of a number of items \[i1,...,i_N\] and options \[o1,...,o_M\].
//! Each of the options "covers" some of the items, e.g. picking option o1 might involve selecting items i1, i5, and i7.
//! The constraint problem is to find a collection of options which cover all of the items exactly once.
//!
//! We can do this in code as
//!
//! ```
//! use dlx_rs::Solver;
//! let mut s = Solver::new(7);
//! s.add_option("o1",&[3,5]);
//! s.add_option("o2",&[1,5,7]);
//! s.add_option("o3",&[2,3,6]);
//! s.add_option("o4",&[1,4,6]);
//! s.add_option("o5",&[2,7]);
//! s.add_option("o6",&[4,5,7]);
//!
//! let sol = s.next().unwrap();
//! assert_eq!(sol,["o4","o5","o1"]);
//!
//! ```
//!
//! ## Solving a Sudoku
//!
//!
//! ```
//! use dlx_rs::Sudoku;
//! // Define sudoku grid, 0 is unknown number
//! let sudoku = vec![
//!     5, 3, 0, 0, 7, 0, 0, 0, 0,
//!     6, 0, 0, 1, 9, 5, 0, 0, 0,
//!     0, 9, 8, 0, 0, 0, 0, 6, 0,
//!     8, 0, 0, 0, 6, 0, 0, 0, 3,
//!     4, 0, 0, 8, 0, 3, 0, 0, 1,
//!     7, 0, 0, 0, 2, 0, 0, 0, 6,
//!     0, 6, 0, 0, 0, 0, 2, 8, 0,
//!     0, 0, 0, 4, 1, 9, 0, 0, 5,
//!     0, 0, 0, 0, 8, 0, 0, 7, 9,
//! ];
//!
//! // Create new sudoku from this grid
//! let mut s = Sudoku::new_from_input(&sudoku);
//!
//! let true_solution = vec![
//!     5, 3, 4, 6, 7, 8, 9, 1, 2,
//!     6, 7, 2, 1, 9, 5, 3, 4, 8,
//!     1, 9, 8, 3, 4, 2, 5, 6, 7,
//!     8, 5, 9, 7, 6, 1, 4, 2, 3,
//!     4, 2, 6, 8, 5, 3, 7, 9, 1,
//!     7, 1, 3, 9, 2, 4, 8, 5, 6,
//!     9, 6, 1, 5, 3, 7, 2, 8, 4,
//!     2, 8, 7, 4, 1, 9, 6, 3, 5,
//!     3, 4, 5, 2, 8, 6, 1, 7, 9,
//! ];
//! // Checks only solution is true solution
//! let solution = s.next().unwrap();
//! assert_eq!(solution, true_solution);
//! assert_eq!(s.next(), None);
//! ```
//!

#![cfg_attr(docsrs, feature(doc_cfg))]
pub mod queens;
pub mod solver;
pub mod sudoku;

pub use crate::queens::Queens;
pub use crate::solver::Solver;
pub use crate::sudoku::Sudoku;
