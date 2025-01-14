use criterion::{black_box, Criterion, criterion_group, criterion_main};

use advent_of_rust_2024::day06::solve_day_06_part_02;
use advent_of_rust_2024::util::{cartesian_product_flat_map, cartesian_product_mut_push, cartesian_product_refs, read_string};

fn bench_day06_part2(c: &mut Criterion) {
    let input = read_string("./src/day06/input.txt").unwrap();

    c.bench_function("day06_part_02", |b| {
        b.iter(|| {
            let output = solve_day_06_part_02(black_box(input.clone()));
            black_box(output);
        });
    });
}

fn bench_cart_product_flat_map(c: &mut Criterion) {
    bench_cartesian_product(c, "cart_prod_flat_map", cartesian_product_flat_map);
}

fn bench_cart_product_mut_push(c: &mut Criterion) {
    bench_cartesian_product(c, "cart_prod_mut_push", cartesian_product_mut_push);
}

fn bench_cart_product_refs(c: &mut Criterion) {
    let vec1 = (0..10000).collect::<Vec<_>>();
    let vec2 = (0..10000).collect::<Vec<_>>();

    c.bench_function("cart_prod_refs", |b| {
        b.iter(|| {
            let count = black_box(cartesian_product_refs(&vec1, &vec2).count());
            assert_eq!(10000 * 10000, count);
        });
    });
}

fn bench_cartesian_product<F, I>(c: &mut Criterion, method_name: &str, func: F)
    where
        F: Fn(Vec<i32>, Vec<i32>) -> I,
        I: Iterator<Item=(i32, i32)>,
        i32: Clone
{
    let vec1 = (0..10000).collect::<Vec<_>>();
    let vec2 = (0..10000).collect::<Vec<_>>();

    c.bench_function(method_name, |b| {
        b.iter(|| {
            let vec1_copy = vec1.clone();
            let vec2_copy = vec2.clone();
            let count =  black_box(func(vec1_copy, vec2_copy).count());
            assert_eq!(10000 * 10000, count);
        });
    });
}

criterion_group!(
    benches,
    // bench_day06_part2,
    bench_cart_product_flat_map,
    bench_cart_product_mut_push,
    bench_cart_product_refs
);

criterion_main!(benches);
