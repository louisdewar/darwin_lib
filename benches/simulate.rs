#[macro_use]
extern crate criterion;

use criterion::black_box;
use criterion::Criterion;

use darwin_lib::{Instruction, InstructionType::MOV, VirtualMachine};

fn vm_init(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "init VM over inputs",
        |b, &&size| {
            b.iter(|| {
                let vm = VirtualMachine::new(size, vec![Instruction::new(MOV, 0, 1)]);

                black_box(vm);
            });
        },
        &[10, 100, 1_000, 10_000, 100_000],
    );
}

fn vm_run(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "iterations with imp",
        |b, &&iterations| {
            let mut vm = VirtualMachine::new(10000, vec![Instruction::new(MOV, 0, 1)]);
            b.iter(|| {
                for _ in 0..iterations {
                    vm.cycle();
                }
            });
        },
        &[10, 100, 1_000, 10_000],
    );
}

criterion_group!(benches, vm_run, vm_init);
criterion_main!(benches);
