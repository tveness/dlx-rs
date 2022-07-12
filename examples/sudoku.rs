use dlx_rs::sudoku::Sudoku;

fn main() {
    // Define sudoku grid, 0 is unknown number
    let sudoku = vec![
        5, 3, 0, 0, 7, 0, 0, 0, 0, 6, 0, 0, 1, 9, 5, 0, 0, 0, 0, 9, 8, 0, 0, 0, 0, 6, 0, 8, 0, 0,
        0, 6, 0, 0, 0, 3, 4, 0, 0, 8, 0, 3, 0, 0, 1, 7, 0, 0, 0, 2, 0, 0, 0, 6, 0, 6, 0, 0, 0, 0,
        2, 8, 0, 0, 0, 0, 4, 1, 9, 0, 0, 5, 0, 0, 0, 0, 8, 0, 0, 7, 9,
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
