use crate::Solver;
use std::collections::HashSet;

enum Color {
    Red,
    Yellow,
    Blue,
    Green,
    Black,
}

/// Finds solutions to Aztec diamond problem
///
pub struct Aztec {
    n: usize,
    pub solver: Solver,
}

impl Aztec {
    /// Creates new `Queens` set up with constraints for the `n` queens problem
    pub fn new(n: usize) -> Aztec {
        // Create blank solver

        // Only constraint is that each square must be covered, and there are

        // (1+2+...+n)*4 = 2*n*(n+1) squares for the order n Aztec diamon

        // Squares are laid out sequentially in memory running left to right from the top down i.e.

        //         1  2
        //      3  4  5  6
        //      7  8  9  10
        //         11 12

        //         1  2
        //      3  4  5  6
        //   7  8  9  10 11 12
        //   13 14 15 16 17 18
        //      19 20 21 22
        //         23 24

        let mut solver = Solver::new(2 * n * (n + 1));

        // Now add options: each option corresponds to either a vertical or horizontal dominos
        // We first add the horizontal dominos, which run along every tile except for the last on each row, which means we have
        // 2*n*(n+1) - 2*n = 2*n*n horizontal domino constraints
        let max = 2 * n * (n + 1);

        // Construct positions at end of row
        let row_ends_top = (1..=n).map(|x| x * (x + 1));
        let row_ends_bottom = (1..=n).map(|x| max - x * (x - 1));

        let row_ends = row_ends_top.chain(row_ends_bottom);
        let row_ends_set: HashSet<usize> = HashSet::from_iter(row_ends);

        //            println!("Row ends: ");
        //       for r in row_ends_set.clone() {
        //          print!("{} ", r);
        //     }
        println!();

        // Horizontal dominoes

        for x in 1..=2 * n * (n + 1) {
            if !row_ends_set.contains(&x) {
                let pos1 = x;
                let pos2 = pos1 + 1;
                let con_name = format!("H{}#{}", pos1, pos2);
                solver.add_option(&con_name, &[pos1, pos2]);

                // Now rotate these positions one quarter turn to the right to pick up vertical dominos
            }
        }

        // Vertical dominoes
        // First n-1 rows are trivial: for row j pick square i, and square i + j*(j+1) - j*(j-1) + 1 = i + 2*j + 1
        for j in 1..=n - 1 {
            for i in 1..=2 * j {
                let pos1 = j * (j - 1) + i;
                let pos2 = pos1 + 2 * j + 1;

                let con_name = format!("V{}#{}", pos1, pos2);
                solver.add_option(&con_name, &[pos1, pos2]);

                // Now add mirror image of this
                let mpos1 = max - pos1 + 1;
                let mpos2 = max - pos2 + 1;

                let mcon_name = format!("V{}#{}", mpos2, mpos1);
                solver.add_option(&mcon_name, &[mpos2, mpos1]);
            }
        }
        // Final (biggest row), runs from n(n-1) + 1 -> n(n-1) + 1 + 2n -1, and pairs with
        let final_min = n * (n - 1) + 1;
        let final_max = final_min + 2 * n - 1;
        for x in final_min..=final_max {
            let pos1 = x;
            let pos2 = x + 2 * n;

            let con_name = format!("V{}#{}", pos1, pos2);
            solver.add_option(&con_name, &[pos1, pos2]);
        }

        Aztec { solver, n }
    }

    pub fn pretty_print_sol(sol: &[(usize, usize)]) {
        let n = (sol.len() as f64).sqrt() as usize;
        //        println!("N: {}",n);
        let max = 2 * n * (n + 1);

        // Construct positions at end of row
        let row_ends_top = (1..=n).map(|x| x * (x + 1));
        let row_ends_bottom = (1..=n).map(|x| max - x * (x - 1));

        let row_ends = row_ends_top.chain(row_ends_bottom);
        let row_ends_set: HashSet<usize> = HashSet::from_iter(row_ends);
        //       println!("{:?}", row_ends_set);

        //        println!("Printing pretty sol");

        let mut solc: Vec<Color> = Vec::with_capacity(2 * sol.len());
        for _ in 1..=2 * sol.len() {
            solc.push(Color::Black);
        }

        // Go through items in solution
        for item in sol {
            let min = (item.0).min(item.1);
            let max = (item.0).max(item.1);

            let par = match min {
                x if x > n * (n + 1) => 1,
                _ => 0,
            };
            // If horizontal bond
            if max == min + 1 {
                if min % 2 == par {
                    solc[min - 1] = Color::Blue;
                    solc[max - 1] = Color::Blue;
                } else {
                    solc[min - 1] = Color::Yellow;
                    solc[max - 1] = Color::Yellow;
                }
            } else {
                if min % 2 == par {
                    solc[min - 1] = Color::Green;
                    solc[max - 1] = Color::Green;
                } else {
                    solc[min - 1] = Color::Red;
                    solc[max - 1] = Color::Red;
                }
            }
        }

        // Now print first n rows
        let mut row_dir = true;
        let mut row_pad = n;
        let mut rr = " ".repeat(row_pad);

        print!("{}", rr);
        for i in 0..solc.len() {
            match solc[i] {
                Color::Red => print!("\x1b[41m \x1b[0m"),
                Color::Green => print!("\x1b[42m \x1b[0m"),
                Color::Yellow => print!("\x1b[43m \x1b[0m"),
                Color::Blue => print!("\x1b[44m \x1b[0m"),
                Color::Black => print!("\x1b[40m \x1b[0m"),
            };

            if row_ends_set.contains(&(i + 1)) {
                if row_dir {
                    row_pad -= 1;
                    if row_pad == 0 {
                        row_dir = false;
                        row_pad += 1;
                    }
                } else {
                    row_pad += 1;
                }
                println!();
                rr = " ".repeat(row_pad);
                print!("{}", rr);
            }
        }
        println!();
    }
}

impl Iterator for Aztec {
    type Item = Vec<(usize, usize)>;
    /// Returns the next solution, which is a vector of tuples denoting the Row and Column of the N queens in the solution
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(sol) = self.solver.next() {
            let mut dom_solved = Vec::with_capacity(self.n);
            for i in sol {
                let i = i.as_str();
                let s: Vec<&str> = i.split(&['H', 'V', '#']).collect();
                let p1: usize = s[1].parse().unwrap();
                let p2: usize = s[2].parse().unwrap();
                dom_solved.push((p1, p2));
            }
            Some(dom_solved)
        } else {
            None
        }
    }
}
