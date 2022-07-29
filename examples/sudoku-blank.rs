use dlx_rs::sudoku::Sudoku;

// Solve sudoku

fn main() {
    // Define sudoku grid, 0 is unknown number
    let sudoku = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    println!("Input:");
    println!("{}", Sudoku::pretty(&sudoku));
    println!();
    let s = Sudoku::new_from_input(&sudoku);
    for solution in s {
        println!("Solution:");
        println!("{}", Sudoku::pretty(&solution));
    }
}
