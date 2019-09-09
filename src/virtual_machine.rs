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

impl VirtualMachine {
    pub fn new(size: usize, program: Vec<Instruction>) -> VirtualMachine {
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

        // TODO: Make this random
        let random_i = 5;

        for (m_instruction, p_instruction) in memory[random_i..(random_i + program.len())]
            .iter_mut()
            .zip(program.iter())
        {
            *m_instruction = *p_instruction
        }

        VirtualMachine {
            memory,
            cur_user: 0,
            users_pcs: vec![VecDeque::from(vec![random_i])],
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
