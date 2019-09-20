use darwin_lib::{cmd, create_program, MatchSettings, VirtualMachine};

fn main() {
    let program = create_program! { MOV(I, 0, Direct, 1, Direct) };

    let mut vm = VirtualMachine::new_battle(
        &[program],
        &MatchSettings {
            min_separation: 10,
            core_size: 20,
            ..Default::default()
        },
    );

    vm.cycle();
    vm.cycle();

    assert_eq!(
        vm.get_memory()
            .iter()
            .filter(|x| x == &&cmd!(MOV(I, 0, Direct, 1, Direct)))
            .count(),
        3,
        "After 2 cycles of the VM there should be exactly 3 copies of the imp"
    );

    for (i, instruction) in vm.get_memory().iter().enumerate() {
        println!("{:02}. {}", i, instruction);
    }
}
