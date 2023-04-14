use dlx_rs::solver::Solver;

// Example constraint problem
//     i1  i2  i3  i4  i5  i6  i7
// A   x           x           x
// B   x           x
// C               x   x       x
// D           x       x   x
// E       x   x           x   x
// F       x                   x
//
//
// The only valid solution is [B,D,F]

fn main() {
    let mut s = Solver::new(7);

    s.add_option("A", &[1, 4, 7])
        .add_option("B", &[1, 4])
        .add_option("C", &[4, 5, 7])
        .add_option("D", &[3, 5, 6])
        .add_option("E", &[2, 3, 6, 7])
        .add_option("F", &[2, 7]);

    println!("Set up problem which looks like:");
    println!("{}", s);
    println!("Now finding solutions");

    for solution in s {
        println!("Solution: {:?}", solution);
    }
}
