#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_docs)]

/// Aztec diamond solver
pub mod aztec;

/// N queens problem solver
pub mod queens;

/// General dancing links solver
pub mod solver;

/// Sudoku solver
pub mod sudoku;

pub use crate::aztec::Aztec;
pub use crate::queens::Queens;
pub use crate::solver::Solver;
pub use crate::sudoku::Sudoku;
