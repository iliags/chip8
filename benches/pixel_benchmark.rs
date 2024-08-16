use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;
use std::hint::black_box;

fn pixel_for(vec: Vec<u8>, new_vec: &mut Vec<bool>) {
    for (i, pixel) in vec.iter().enumerate() {
        new_vec[i] = *pixel > 0;
    }
}

fn pixel_map(vec: Vec<u8>) -> Vec<bool> {
    vec.iter().map(|&pixel| pixel > 0).collect()
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let length = 8192;
    let test_vec: Vec<u8> = (0..length).map(|_| rng.gen_range(0..1)).collect();

    let mut temp_vec = vec![false; length];
    c.bench_function("pixel for", |b| {
        b.iter(|| pixel_for(black_box(test_vec.clone()), black_box(&mut temp_vec)))
    });
    c.bench_function("pixel map", |b| {
        b.iter(|| pixel_map(black_box(test_vec.clone())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
