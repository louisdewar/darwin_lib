#[macro_use]
extern crate criterion;

use criterion::black_box;
use criterion::Criterion;

use darwin_lib::{cmd, VirtualMachine};

fn vm_init(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "init VM over inputs",
        |b, &&size| {
            b.iter(|| {
                let vm = VirtualMachine::new(size, vec![cmd! { MOV(0, Direct, 1, Direct) }]);

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
            let mut vm = VirtualMachine::new(10000, vec![cmd! { MOV(0, Direct, 1, Direct) }]);
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
