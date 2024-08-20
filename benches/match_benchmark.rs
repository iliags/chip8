use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;
use std::hint::black_box;

const OPCODES: &[u16] = &[
    0x0000, 0x00E0, 0x00EE, 0x1000, 0x2000, 0x3000, 0x4000, 0x5000, 0x6000, 0x7000, 0x8000,
];

fn match_instruction_single(opcode_list: Vec<u16>) -> Vec<usize> {
    let mut result = vec![0, opcode_list.len()];

    for opcode in opcode_list {
        match opcode {
            0x0000 => result.push(opcode.into()),
            0x00E0 => result.push(opcode.into()),
            0x00EE => result.push(opcode.into()),
            0x1000 => result.push(opcode.into()),
            0x2000 => result.push(opcode.into()),
            0x3000 => result.push(opcode.into()),
            0x4000 => result.push(opcode.into()),
            0x5000 => result.push(opcode.into()),
            0x6000 => result.push(opcode.into()),
            0x7000 => result.push(opcode.into()),
            0x8000 => result.push(opcode.into()),
            _ => {}
        }
    }

    result
}

fn match_instruction_multi(opcode_list: Vec<u16>) -> Vec<usize> {
    let mut result = vec![0, opcode_list.len()];

    for opcode in opcode_list {
        let op_1 = (opcode & 0xF000) >> 12;
        let op_2 = (opcode & 0x0F00) >> 8;
        let op_3 = (opcode & 0x00F0) >> 4;
        let op_4 = opcode & 0x000F;

        match (op_1, op_2, op_3, op_4) {
            (0, 0, 0, 0) => result.push(opcode.into()),
            (0, 0, 0xE, 0) => result.push(opcode.into()),
            (0, 0, 0xE, 0xE) => result.push(opcode.into()),
            (0x1, _, _, _) => result.push(opcode.into()),
            (0x2, _, _, _) => result.push(opcode.into()),
            (0x3, _, _, _) => result.push(opcode.into()),
            (0x4, _, _, _) => result.push(opcode.into()),
            (0x5, _, _, 0) => result.push(opcode.into()),
            (0x6, _, _, _) => result.push(opcode.into()),
            (0x7, _, _, _) => result.push(opcode.into()),
            (0x8, _, _, _) => result.push(opcode.into()),
            _ => {}
        }
    }

    result
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let length = 2048;
    let test_vec: Vec<u16> = (0..length)
        .map(|_| OPCODES[rng.gen_range(0..OPCODES.len())])
        .collect();

    c.bench_function("single match", |b| {
        b.iter(|| match_instruction_single(black_box(test_vec.clone())))
    });

    c.bench_function("multi match", |b| {
        b.iter(|| match_instruction_multi(black_box(test_vec.clone())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
