use crate::{handlers, AddressMode, Instruction, Modifier, OpCode};

use std::collections::VecDeque;

#[derive(Debug)]
pub struct VirtualMachine {
    memory: Vec<Instruction>,
    /// Each element in this vector represents the queue of a user's processes
    /// The next instruction index is at the front of the queue for each user
    users_pcs: Vec<VecDeque<usize>>,
    /// The id of the user whose process should run next
    cur_user: usize,
}

// Export this into a function to make it easier to implement more insertion strategies in the future
fn generate_random_insertion_points(size: usize, programs: &[Vec<Instruction>]) -> Vec<usize> {
    let program_lengths: Vec<usize> = programs.iter().map(|v| v.len()).collect();
    let total_program_length: usize = program_lengths.iter().sum();

    let default_padding = 10;

    let mut budget: usize = size
        .checked_sub(total_program_length + default_padding * programs.len())
        .expect("Total program length plus padding is greater than memory length");

    use rand::Rng;
    let mut rng = rand::thread_rng();

    // First program is inserted at 0
    let mut indices = vec![0];
    let mut cur_index = program_lengths[0] + default_padding;

    for (i, program_len) in program_lengths.iter().enumerate().skip(1) {
        // Mean of a shared amount of the *remaining* (hence -i) budget
        // note: if there were two programs there would be 3 gaps
        let mean = budget as f64 / (programs.len() + 1 - i) as f64;

        let tenth_budget = budget as f64 * 0.1;

        // Generate a number in range ±10% of the budget around the mean
        let n: usize = rng.gen_range(
            (mean - tenth_budget) as usize,
            (mean + tenth_budget) as usize,
        );

        // Find the start index, including default padding and extra random padding
        cur_index = cur_index + program_len + default_padding + n;
        budget -= program_len + default_padding + n;

        indices.push(cur_index);
    }

    indices
}

impl VirtualMachine {
    pub fn new(size: usize, programs: Vec<Vec<Instruction>>) -> VirtualMachine {
        let mut memory: Vec<Instruction> = (0..size)
            .map(|_| {
                Instruction::new(
                    OpCode::DAT,
                    Modifier::None,
                    0,
                    AddressMode::Immediate,
                    0,
                    AddressMode::Immediate,
                )
            })
            .collect();

        let indices = generate_random_insertion_points(size, &programs);

        for (start_index, program) in indices.iter().zip(programs.iter()) {
            for (instruction_i, instruction) in program.iter().enumerate() {
                memory[(start_index + instruction_i) % size] = *instruction
            }
        }

        VirtualMachine {
            memory,
            cur_user: 0,
            users_pcs: (0..programs.len())
                .map(|i| VecDeque::from(vec![indices[i]]))
                .collect(),
        }
    }

    pub fn get_memory(&self) -> &[Instruction] {
        &self.memory
    }

    pub fn get_users_pcs(&self) -> &[VecDeque<usize>] {
        &self.users_pcs
    }

    pub fn get_cur_user(&self) -> usize {
        self.cur_user
    }

    /// Runs one iteration of the virtual machine
    pub fn cycle(&mut self) {
        // Get the user's process queue
        let process_queue = &mut self.users_pcs[self.cur_user];

        // Get the program counter (the index of the current instruction in memory)
        // from the front of the PC queue
        let pc = process_queue
            .pop_front()
            .expect("All user processes have been killed"); // TODO: Better handling of end of users processes

        // Get the current instruction
        let instruction = self.memory[pc];

        let memory_len = self.memory.len();

        // Advance the PC by 1 and add it to the back of the queue (for this user)
        // For the commands that don't want to have the PC advanced by 1, they must override this
        process_queue.push_back((pc + 1) % memory_len);

        use OpCode::*;

        // Run different code for each instruction
        match instruction.op_code {
            MOV => handlers::mov(instruction, pc, memory_len, &mut self.memory),
            ADD => handlers::add(instruction, pc, memory_len, &mut self.memory),
            DAT => {
                // Remove the last queued process (kill it)
                process_queue.pop_back().unwrap();
            }
            JMP => {
                let new_addr = handlers::jmp(instruction, pc, memory_len, &self.memory);
                // Override the program counter for this process to the new address
                *(process_queue.back_mut().unwrap()) = new_addr;
            }
            SPL => {
                let new_addr = handlers::spl(instruction, pc, memory_len, &self.memory);

                // Queue an additional process
                process_queue.push_back(new_addr);
            }
        }

        // Advance the user counter
        self.cur_user = (self.cur_user + 1) % self.users_pcs.len();
    }
}
