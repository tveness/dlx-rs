#[macro_use]
extern crate criterion;
use std::time::Duration;

use criterion::Criterion;

use dlx_rs::solver::Solver;
use dlx_rs::sudoku::Sudoku;

fn sudoku(c: &mut Criterion) {
    let mut group = c.benchmark_group("sample-size");
    group
        .sample_size(2000)
        .measurement_time(Duration::from_secs(30));

    group.bench_function("sudoku", |b| {
        b.iter(|| {
            let sudoku = vec![
                5, 3, 0, 0, 7, 0, 0, 0, 0, 6, 0, 0, 1, 9, 5, 0, 0, 0, 0, 9, 8, 0, 0, 0, 0, 6, 0, 8,
                0, 0, 0, 6, 0, 0, 0, 3, 4, 0, 0, 8, 0, 3, 0, 0, 1, 7, 0, 0, 0, 2, 0, 0, 0, 6, 0, 6,
                0, 0, 0, 0, 2, 8, 0, 0, 0, 0, 4, 1, 9, 0, 0, 5, 0, 0, 0, 0, 8, 0, 0, 7, 9,
            ];
            //println!("Input:");
            //println!("{}", Sudoku::pretty(&sudoku));
            //println!();
            let s = Sudoku::new_from_input(&sudoku);
            for _solution in s {
                //println!("Solution:");
                //println!("{}", Sudoku::pretty(&solution));
            }
        })
    });
    group.finish()
}

fn simple(c: &mut Criterion) {
    let mut group = c.benchmark_group("sample-size");
    group
        .sample_size(2000)
        .measurement_time(Duration::from_secs(30));

    group.bench_function("simple", |b| {
        b.iter(|| {
            let mut s = Solver::new(7);

            s.add_option("o1", &[3, 5])
                .add_option("o2", &[1, 4, 7])
                .add_option("o3", &[2, 3, 6])
                .add_option("o4", &[1, 4, 6])
                .add_option("o5", &[2, 7])
                .add_option("o6", &[4, 5, 7]);

            for _solution in s {}
        })
    });
    group.finish()
}

criterion_group!(benches, sudoku, simple);
criterion_main!(benches);
