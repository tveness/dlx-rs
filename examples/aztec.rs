use dlx_rs::aztec::Aztec;
use rand::seq::IteratorRandom;

// Solve the Aztec diamond of order n

fn main() {
    for n in 1..=4 {
        //let n = 4;

        // First, count all of the solutions
        let a = Aztec::new(n);
        let na = a.count();
        println!("Number of solutions for n={}: {}", n, na);

        // Get a random solution
        let a = Aztec::new(n);
        let mut rng = rand::thread_rng();
        let s = a.choose(&mut rng).unwrap();

        Aztec::pretty_print_sol(&s);
    }
}
