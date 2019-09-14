use darwin_lib::{create_program, VirtualMachine};

#[test]
fn spl() {
    let mut vm = VirtualMachine::new(
        20,
        vec![create_program! {
            SPL(None, 1, Direct, 0, Direct)
            JMP(None, -1, Direct, 0, Direct)
        }],
    );

    vm.cycle();

    assert_eq!(vm.get_users_pcs()[0].len(), 2);

    // 16189 was found to be the time at which point there should be 8000 processes
    // 16500 should allow more SPL command to be executed so that if there are more than 8000 processes
    // a bug would be shown
    for _ in 0..16500 {
        vm.cycle();
    }

    assert_eq!(vm.get_users_pcs()[0].len(), 8000);
}

#[test]
fn nop() {
    let mut vm = VirtualMachine::new(
        20,
        vec![create_program! {
            NOP(None, 0, Direct, 0, Direct)
        }],
    );

    vm.cycle();

    assert_eq!(vm.get_users_pcs()[0][0], 1);
}
