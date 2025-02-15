#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_docs)]

/// Aztec diamond solver
#[cfg(feature = "aztec")]
pub mod aztec;
#[cfg(feature = "aztec")]
pub use crate::aztec::Aztec;

/// N queens problem solver
#[cfg(feature = "queens")]
pub mod queens;
#[cfg(feature = "queens")]
pub use crate::queens::Queens;

/// General dancing links solver
pub mod solver;
pub use crate::solver::Solver;

#[cfg(feature = "sudoku")]
/// Sudoku solver
pub mod sudoku;
#[cfg(feature = "sudoku")]
pub use crate::sudoku::Sudoku;
