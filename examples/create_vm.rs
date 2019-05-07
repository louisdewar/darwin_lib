use darwin_lib::{cmd, create_program, VirtualMachine};

fn main() {
    let program = create_program! { MOV(0, Direct, 1, Direct) };

    let mut vm = VirtualMachine::new(20, program);

    vm.cycle();
    vm.cycle();

    assert_eq!(
        vm.get_memory()
            .iter()
            .filter(|x| x == &&cmd!(MOV(0, Direct, 1, Direct)))
            .count(),
        3,
        "After 2 cycles of the VM there should be exactly 3 copies of the imp"
    );

    println!("VM: {:?}", vm);
}
