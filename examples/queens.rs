use dlx_rs::queens::Queens;

// Solve the n queens problem

fn main() {
    println!("N   | Solutions");
    for n in 1..=9 {
        let q = Queens::new(n);
        let nq = q.count();
        let w = 3;

        println!("{:w$} | {} ", n, nq);
    }
}
