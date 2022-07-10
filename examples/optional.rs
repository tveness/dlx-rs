use dlx_rs::solver::Solver;

//
// Example, where optional elements are after |
//     i1  i2  i3  i4  i5  i6  i7  |  i8
// o1   0   0   1   0   1   0   0  |   0
// o2   1   0   0   1   0   0   0  |   0
// o3   0   1   1   0   0   0   0  |   0
// o4   1   0   0   1   0   1   0  |   0
// o5   0   1   0   0   0   0   1  |   0
// o6   0   0   0   1   1   0   1  |   0
// o7   0   0   1   0   1   0   0  |   1
//
// The only solutions are now
// [o1, o4, o5] (i8 not covered, as it is optional)
// [o7, o4, o5] (i8 now covered)

fn main() {
    let mut s = Solver::new_optional(7, 1);

    s.add_option("o1", &[3, 5]);
    s.add_option("o2", &[1, 4, 7]);
    s.add_option("o3", &[2, 3, 6]);
    s.add_option("o4", &[1, 4, 6]);
    s.add_option("o5", &[2, 7]);
    s.add_option("o6", &[4, 5, 7]);
    s.add_option("o7", &[3, 5, 8]);

    println!("Set up problem which looks like:");
    println!("{}", s);
    println!("Now finding solutions");

    for solution in s {
        println!("Solution: {:?}", solution);
    }
}
