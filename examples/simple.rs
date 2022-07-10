use dlx_rs::solver::Solver;

// Example constraint problem
//     i1  i2  i3  i4  i5  i6  i7
// o1   0   0   1   0   1   0   0
// o2   1   0   0   1   0   0   0
// o3   0   1   1   0   0   0   0
// o4   1   0   0   1   0   1   0
// o5   0   1   0   0   0   0   1
// o6   0   0   0   1   1   0   1
//
//
// The only valid solution is [o1,o4,o5]

fn main() {
    let mut s = Solver::new(7);

    s.add_option("o1", &[3, 5]);
    s.add_option("o2", &[1, 4, 7]);
    s.add_option("o3", &[2, 3, 6]);
    s.add_option("o4", &[1, 4, 6]);
    s.add_option("o5", &[2, 7]);
    s.add_option("o6", &[4, 5, 7]);

    println!("Set up problem which looks like:");
    println!("{}", s);
    println!("Now finding solutions");

    for solution in s {
        println!("Solution: {:?}", solution);
    }
}
