use crate::solver::Solver;

/// Implements sudoku solver
///
/// ```
///# use dlx_rs::sudoku::Sudoku;
/// // Define sudoku grid, 0 is unknown number
/// let sudoku = vec![
///     5, 3, 0, 0, 7, 0, 0, 0, 0,
///     6, 0, 0, 1, 9, 5, 0, 0, 0,
///     0, 9, 8, 0, 0, 0, 0, 6, 0,
///     8, 0, 0, 0, 6, 0, 0, 0, 3,
///     4, 0, 0, 8, 0, 3, 0, 0, 1,
///     7, 0, 0, 0, 2, 0, 0, 0, 6,
///     0, 6, 0, 0, 0, 0, 2, 8, 0,
///     0, 0, 0, 4, 1, 9, 0, 0, 5,
///     0, 0, 0, 0, 8, 0, 0, 7, 9,
/// ];
///
/// // Create new sudoku from this grid
/// let mut s = Sudoku::new_from_input(&sudoku);
///
/// let true_solution = vec![
///     5, 3, 4, 6, 7, 8, 9, 1, 2,
///     6, 7, 2, 1, 9, 5, 3, 4, 8,
///     1, 9, 8, 3, 4, 2, 5, 6, 7,
///     8, 5, 9, 7, 6, 1, 4, 2, 3,
///     4, 2, 6, 8, 5, 3, 7, 9, 1,
///     7, 1, 3, 9, 2, 4, 8, 5, 6,
///     9, 6, 1, 5, 3, 7, 2, 8, 4,
///     2, 8, 7, 4, 1, 9, 6, 3, 5,
///     3, 4, 5, 2, 8, 6, 1, 7, 9,
/// ];
/// // Checks only solution is true solution
/// let solution = s.next().unwrap();
/// assert_eq!(solution, true_solution);
/// assert_eq!(s.next(), None);
/// ```
pub struct Sudoku {
    pub solver: Solver,
    input: Vec<usize>,
    n: usize,
}

impl Sudoku {
    // Initialises the constraints for an n*n sudoku-grid (regular is n=3, as the grid is 9x9)
    // This corresponds to a matrix with dimension (n**6)x(4*n**4)
    pub fn new(n: usize) -> Sudoku {
        // What are the constraints we need to meet?
        // 1. Each cell must contain a number i.e. R1C1 must have precisely one number in it
        // 2. Each row must have a 1, each row must have a 2, ...n^2
        // 3. Each col must have a 1, each col must have a 2, ...n^2
        // 4. Each sub-square must have a 1, each sub-square must have a 2, ...n^2
        #[allow(non_snake_case)]
        let N = n * n; // Sudoku edge length

        //1: N*N constraints
        //2: N rows * N numbers
        //3: N cols * N numbers
        //4: N cols * N numbers
        //T: 4 N**2 items

        let mut solver = Solver::new(4 * N * N);

        // And how many options are there?
        // Each cell may contain N options, and there are N*N, so N*N*N options
        // e.g. R1C1#1: inserting a 1 into R1, C1

        // For standard sudoku: 4*9^2 x 9^3 = 324 x 729

        // 1. First constraints run R1C1,R1C2,...,R1CN,R2C1,...,RNCN
        // After N^2 of these, we then have
        // 2. Row constraint runs R1#1 R1#2 ... R2#1 R2#2
        // 3. Col constraint runs C1#1 C1#2 ... C2#1 C2#2
        // 4. Sub constraint runs S1#1 S1#2 ... S2#1 S2#2

        // Add all of the options
        for row in 1..=N {
            for col in 1..=N {
                for val in 1..=N {
                    let constraint_name = format!("R{}C{}#{}", row, col, val);
                    // Now add option
                    // Runs 1->N*(N-1)+N = N*N
                    let cell_con = col + (row - 1) * N;
                    // Runs N*N+1 -> N*N + N*(N-1) + N = 2*N*N
                    let row_con = N * N + N * (row - 1) + val;
                    // Runs 2*N*N+1 -> 2*N*N + N*(N-1) + N = 3*N*N
                    let col_con = 2 * N * N + N * (col - 1) + val;
                    let sub = (col - 1) / n + n * ((row - 1) / n);
                    // Runs 3*N*N+1 -> 3*N*N + N*(N-1) + N = 4*N*N
                    let sub_con = 3 * N * N + N * (sub) + val;
                    //println!("Adding constraint: {}",constraint_name);
                    solver.add_option(&constraint_name, &[cell_con, row_con, col_con, sub_con]);

                    /*
                    if !(0 < cell_con && cell_con <= N*N) {
                        panic!("Woops! cell_con = {}, MIN = {}, MAX = {}", cell_con, 0*N*N,1*N*N );
                    }
                    if !(N*N < row_con && row_con <= 2*N*N) {
                        panic!("Woops! row_con = {}, MIN = {}, MAX = {}", row_con, 1*N*N,2*N*N );
                    }
                    if !(2*N*N < col_con && col_con <= 3*N*N) {
                        panic!("Woops! col_con = {}, MIN = {}, MAX = {}", col_con, 2*N*N,3*N*N );
                    }
                    if !(3*N*N < sub_con && sub_con <= 4*N*N) {
                        panic!("Woops! sub_con = {}, MIN = {}, MAX = {}", sub_con, 3*N*N,4*N*N );
                    }
                    */
                }
            }
        }

        Sudoku {
            solver,
            n,
            input: vec![],
        }
    }

    /// Initialises an appropriately sized Sudoku with all of the correct
    /// constraints, and then selects all of the options corresponding the the
    /// non-zero entires in `input`
    pub fn new_from_input(input: &[usize]) -> Self {
        let inputv = input.to_vec();
        let nsq: usize = inputv.len();
        let n: usize = (nsq as f64).sqrt().sqrt() as usize;

        if nsq != n * n * n * n {
            panic!("Input must be an array of length n**4")
        }
        let mut s = Self::new(n);
        s.input = inputv;

        for (i, item) in input.iter().enumerate() {
            if *item != 0 {
                let row = i / (n * n);
                let col = i - n * n * row;
                let opt_string = format!("R{}C{}#{}", row + 1, col + 1, *item);
                //            println!("{}",opt_string);
                s.solver.select(&opt_string).unwrap();
            }
        }

        s
    }
}

impl Iterator for Sudoku {
    type Item = Vec<usize>;

    /// If a remaining solution exists, returns `Some(v)` where `v` is a `Vec<usize>` containing the flat solve Sudoku grid.
    /// Otherwise returns `None`
    /// ```
    ///# use dlx_rs::sudoku::Sudoku;
    /// // Define sudoku grid, 0 is unknown number
    /// let sudoku = vec![
    ///     5, 3, 0, 0, 7, 0, 0, 0, 0,
    ///     6, 0, 0, 1, 9, 5, 0, 0, 0,
    ///     0, 9, 8, 0, 0, 0, 0, 6, 0,
    ///     8, 0, 0, 0, 6, 0, 0, 0, 3,
    ///     4, 0, 0, 8, 0, 3, 0, 0, 1,
    ///     7, 0, 0, 0, 2, 0, 0, 0, 6,
    ///     0, 6, 0, 0, 0, 0, 2, 8, 0,
    ///     0, 0, 0, 4, 1, 9, 0, 0, 5,
    ///     0, 0, 0, 0, 8, 0, 0, 7, 9,
    /// ];
    ///
    /// // Create new sudoku from this grid
    /// let mut s = Sudoku::new_from_input(&sudoku);
    ///
    /// let true_solution = vec![
    ///     5, 3, 4, 6, 7, 8, 9, 1, 2,
    ///     6, 7, 2, 1, 9, 5, 3, 4, 8,
    ///     1, 9, 8, 3, 4, 2, 5, 6, 7,
    ///     8, 5, 9, 7, 6, 1, 4, 2, 3,
    ///     4, 2, 6, 8, 5, 3, 7, 9, 1,
    ///     7, 1, 3, 9, 2, 4, 8, 5, 6,
    ///     9, 6, 1, 5, 3, 7, 2, 8, 4,
    ///     2, 8, 7, 4, 1, 9, 6, 3, 5,
    ///     3, 4, 5, 2, 8, 6, 1, 7, 9,
    /// ];
    /// // Checks solution
    /// let solution =s.next();
    /// assert_eq!(solution, Some(true_solution));
    ///
    /// let another = s.next();
    /// assert_eq!(another, None);
    ///
    /// ```
    ///
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(sol) = self.solver.next() {
            let mut sudoku_solved = self.input.clone();
            for i in sol {
                let i = i.as_str();
                let s: Vec<&str> = i.split(&['R', 'C', '#']).collect(); //.split('C').split('#');
                let r: usize = s[1].parse().unwrap();
                let c: usize = s[2].parse().unwrap();
                let v: usize = s[3].parse().unwrap();
                sudoku_solved[(c - 1) + self.n * self.n * (r - 1)] = v;
            }
            Some(sudoku_solved)
        } else {
            None
        }
    }
}

impl Sudoku {
    /// Takes an input sudoku array and produces a pretty printed version
    /// ```
    ///# use dlx_rs::sudoku::Sudoku;
    /// let sudoku = vec![
    /// 5, 3, 0, 0, 7, 0, 0, 0, 0,
    /// 6, 0, 0, 1, 9, 5, 0, 0, 0,
    /// 0, 9, 8, 0, 0, 0, 0, 6, 0,
    /// 8, 0, 0, 0, 6, 0, 0, 0, 3,
    /// 4, 0, 0, 8, 0, 3, 0, 0, 1,
    /// 7, 0, 0, 0, 2, 0, 0, 0, 6,
    /// 0, 6, 0, 0, 0, 0, 2, 8, 0,
    /// 0, 0, 0, 4, 1, 9, 0, 0, 5,
    /// 0, 0, 0, 0, 8, 0, 0, 7, 9,
    /// ];
    /// println!("{}",&Sudoku::pretty(&sudoku));
    /// ```
    /// produces
    /// ```text
    ///  5 3   ║   7   ║
    ///  6     ║ 1 9 5 ║
    ///    9 8 ║       ║   6
    /// ═══════╬═══════╬═══════
    ///  8     ║   6   ║     3
    ///  4     ║ 8   3 ║     1
    ///  7     ║   2   ║     6
    /// ═══════╬═══════╬═══════
    ///    6   ║       ║ 2 8
    ///        ║ 4 1 9 ║     5
    ///        ║   8   ║   7 9
    /// ```
    ///
    ///
    pub fn pretty(sudoku_solved: &[usize]) -> String {
        let mut result = String::from("");
        let n = (sudoku_solved.len() as f64).sqrt().sqrt() as usize;
        #[allow(non_snake_case)]
        let N = n * n;
        // Print the array in a pretty way
        for i in 0..N {
            result += " ";
            for j in 0..N {
                result += &match sudoku_solved[i * N + j] {
                    0 => String::from(" "),
                    v => v.to_string(),
                };
                result += " ";

                if (j + 1) % n == 0 && j < N - 1 {
                    result += "║ ";
                }
            }
            if i < N - 1 {
                result += "\n";
            }
            if (i + 1) % n == 0 && i < N - 1 {
                result += &("═".repeat(2 * n + 1));
                for _ in 1..n {
                    result += "╬";
                    result += &("═".repeat(2 * n + 1));
                }
                result += "\n";
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sudoku_solve() {
        let sudoku = vec![
            5, 3, 0, 0, 7, 0, 0, 0, 0, 6, 0, 0, 1, 9, 5, 0, 0, 0, 0, 9, 8, 0, 0, 0, 0, 6, 0, 8, 0,
            0, 0, 6, 0, 0, 0, 3, 4, 0, 0, 8, 0, 3, 0, 0, 1, 7, 0, 0, 0, 2, 0, 0, 0, 6, 0, 6, 0, 0,
            0, 0, 2, 8, 0, 0, 0, 0, 4, 1, 9, 0, 0, 5, 0, 0, 0, 0, 8, 0, 0, 7, 9,
        ];

        let mut s = Sudoku::new_from_input(&sudoku);

        let true_solution = vec![
            5, 3, 4, 6, 7, 8, 9, 1, 2, 6, 7, 2, 1, 9, 5, 3, 4, 8, 1, 9, 8, 3, 4, 2, 5, 6, 7, 8, 5,
            9, 7, 6, 1, 4, 2, 3, 4, 2, 6, 8, 5, 3, 7, 9, 1, 7, 1, 3, 9, 2, 4, 8, 5, 6, 9, 6, 1, 5,
            3, 7, 2, 8, 4, 2, 8, 7, 4, 1, 9, 6, 3, 5, 3, 4, 5, 2, 8, 6, 1, 7, 9,
        ];
        let sol = s.next().unwrap();
        assert_eq!(sol, true_solution);
    }
}
