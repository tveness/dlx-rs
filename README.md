# dlx_rs

[![Crates.io](https://img.shields.io/crates/v/dlx-rs.svg?style=for-the-badge)](https://crates.io/crates/dlx-rs)
[![Documentation](https://img.shields.io/docsrs/dlx-rs?style=for-the-badge)](https://docs.rs/dlx-rs/)
[![Build status](https://img.shields.io/github/actions/workflow/status/tveness/dlx-rs/rust.yml?label=Tests&style=for-the-badge
)](https://github.com/tveness/dlx-rs/actions/workflows/rust.yml)
[![License](https://img.shields.io/github/license/tveness/dlx-rs?style=for-the-badge)](https://creativecommons.org/publicdomain/zero/1.0/legalcode)

dlx_rs is a  Rust library for solving exact cover/constraint problems
problems using Knuth's [Dancing Links](https://en.wikipedia.org/wiki/Dancing_Links) (DLX) algorithm.

It also provides specific interfaces for some common exact cover problems,
specifically:

* arbitrary Sudokus
* N queens problem
* Aztec diamond
* Pentomino tilings (TODO)
* graph colouring (TODO)


## Setting up a general constraint problem

A constraint problem may be expressed in terms of a number of items \[i_1,...,i_N\] and options \[o_1,...,o_M\].
Each of the options "covers" some of the items, e.g. picking option o1 might involve selecting items i1, i5, and i7.
The constraint problem is to find a collection of options which cover all of the items exactly once.

This can be expressed in terms of a matrix, where each option covers the
items for which the corresponding entry is 1, and doesn't if it is 0
```text
     i1  i2  i3  i4  i5  i6  i7
 o1   0   0   1   0   1   0   0
 o2   1   0   0   1   0   0   0
 o3   0   1   1   0   0   0   0
 o4   1   0   0   1   0   1   0
 o5   0   1   0   0   0   0   1
 o6   0   0   0   1   1   0   1
```
The exact cover problem is that of finding a collection of options such that
a 1 appears exactly once in each column.

This is achieved in the case above by selecting options \[o_1,o_4,o_5\].

The code to solve this is
```rust
use dlx_rs::Solver;

#[derive(Clone, PartialEq, Debug)]
enum Opts {
    O1,
    O2,
    O3,
    O4,
    O5,
    O6,
}

let mut s = Solver::new(7);
s.add_option(Opts::O1, &[3, 5])
    .add_option(Opts::O2, &[1, 5, 7])
    .add_option(Opts::O3, &[2, 3, 6])
    .add_option(Opts::O4, &[1, 4, 6])
    .add_option(Opts::O5, &[2, 7])
    .add_option(Opts::O6, &[4, 5, 7]);

let sol = s.next().unwrap_or_default();
assert_eq!(sol, [Opts::O4, Opts::O5, Opts::O1]);
```

Or, we can use strings in a case where we might want to generate the options at runtime

```rust
use dlx_rs::Solver;

let mut s = Solver::new(7);
s.add_option("o1", &[3, 5])
    .add_option("o2", &[1, 5, 7])
    .add_option("o3", &[2, 3, 6])
    .add_option("o4", &[1, 4, 6])
    .add_option("o5", &[2, 7])
    .add_option("o6", &[4, 5, 7]);

let sol = s.next().unwrap_or_default();
assert_eq!(sol, ["o4", "o5", "o1"]);
```
## Solving a Sudoku

```rust
use dlx_rs::Sudoku;
// Define sudoku grid, 0 is unknown number
let sudoku = vec![
    5, 3, 0, 0, 7, 0, 0, 0, 0,
    6, 0, 0, 1, 9, 5, 0, 0, 0,
    0, 9, 8, 0, 0, 0, 0, 6, 0,
    8, 0, 0, 0, 6, 0, 0, 0, 3,
    4, 0, 0, 8, 0, 3, 0, 0, 1,
    7, 0, 0, 0, 2, 0, 0, 0, 6,
    0, 6, 0, 0, 0, 0, 2, 8, 0,
    0, 0, 0, 4, 1, 9, 0, 0, 5,
    0, 0, 0, 0, 8, 0, 0, 7, 9,
];

// Create new sudoku from this grid
let mut s = Sudoku::new_from_input(&sudoku);

let true_solution = vec![
    5, 3, 4, 6, 7, 8, 9, 1, 2,
    6, 7, 2, 1, 9, 5, 3, 4, 8,
    1, 9, 8, 3, 4, 2, 5, 6, 7,
    8, 5, 9, 7, 6, 1, 4, 2, 3,
    4, 2, 6, 8, 5, 3, 7, 9, 1,
    7, 1, 3, 9, 2, 4, 8, 5, 6,
    9, 6, 1, 5, 3, 7, 2, 8, 4,
    2, 8, 7, 4, 1, 9, 6, 3, 5,
    3, 4, 5, 2, 8, 6, 1, 7, 9,
];
// Checks only solution is true solution
let solution = s.next().unwrap_or_default();
assert_eq!(solution, true_solution);
assert_eq!(s.next(), None);
```
