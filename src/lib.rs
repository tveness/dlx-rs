//! What does this do?
//!
//!
//!

#![cfg_attr(docsrs, feature(doc_cfg))]
pub mod queens;
pub mod solver;
pub mod sudoku;

pub use crate::queens::Queens;
pub use crate::solver::Solver;
pub use crate::sudoku::Sudoku;
