use crate::Solver;

/// Implements solution to the N queens problem
///
/// ```
///# use dlx_rs::queens::Queens;
/// let n_queens_solutions = vec![0,1,0,0,2,10,4,40,92,352,724];
/// for i in 1..=10 {
///     let qi = Queens::new(i);
///     assert_eq!(qi.count(), n_queens_solutions[i]);
/// }
/// ```
pub struct Queens {
    n: usize,
    solver: Solver,
}

impl Queens {
    /// Creates new `Queens` set up with constraints for the N queens problem
    pub fn new(n: usize) -> Queens {
        // Create blank solver

        // Add constraints

        // What are the constraints?
        // We are putting N queens on a chess board

        // 1. Each of the N columns must have exactly 1 queen on it
        // 2. Each of the N rows must have exactly 1 queen on it
        // 3. Each of the 2*N-1 right diagonals may have at most one queen
        // 4. Each of the 2*N-1 left diagonals may have at most one queen
        // 5. Each of the N^2 squares may have at most one queen

        // So this gives us 2*N mandatory items, and N^2 + 6*N -2 optional ones

        let mut solver = Solver::new_optional(2 * n, n * n + 6 * n - 2);

        // Now add options: each option corresponds to a queen in a particular

        for r in 1..=n {
            for c in 1..=n {
                let con_name = format!("R{}C{}", r, c);
                // 1 -> N
                let col_con = c;
                // N+1 -> 2*N
                let row_con = n + r;
                // 2*N+1 -> 4*N - 1
                let rd_con = 2 * n + c - r + n;
                // 4*N ->  6*N - 2
                let ld_con = 4 * n - 1 + r + c - 1;
                // 6*N-1 -> N**2 + 6*N - 2
                let is_queen = 6 * n - 2 + r + n * (c - 1);

                solver.add_option(&con_name, &[col_con, row_con, rd_con, ld_con, is_queen]);
            }
        }

        Queens { solver, n }
    }
}

impl Iterator for Queens {
    type Item = Vec<(usize, usize)>;
    /// Returns the next solution, which is a vector of tuples denoting the Row and Column of the N queens in the solution
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(sol) = self.solver.next() {
            let mut n_queens_solved = Vec::with_capacity(self.n);
            for i in sol {
                let i = i.as_str();
                let s: Vec<&str> = i.split(&['R', 'C']).collect(); //.split('C').split('#');
                let r: usize = s[1].parse().unwrap();
                let c: usize = s[2].parse().unwrap();
                n_queens_solved.push((r, c));
            }
            Some(n_queens_solved)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_queens() {
        let q8 = Queens::new(8);
        let n8 = q8.count();
        println!("N8: {}", n8);
    }
}
