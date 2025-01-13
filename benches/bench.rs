use criterion::{black_box, Criterion, criterion_group, criterion_main};

use advent_of_rust_2024::day06::solve_day_06_part_02;
use advent_of_rust_2024::util::read_string;

fn bench_day06_part2(c: &mut Criterion) {
    let input = read_string("./src/day06/input.txt").unwrap();

    c.bench_function("day06_part_02", |b| {
        b.iter(|| {
            let output = solve_day_06_part_02(black_box(input.clone()));
            black_box(output);
            println!("bench???")
        });
    });
}

// Register our benchmark functions:
criterion_group!(day06_benches, bench_day06_part2);
criterion_main!(day06_benches);
