use crate::{handlers, AddressMode, Instruction, Modifier, OpCode};

#[derive(Debug)]
pub struct VirtualMachine {
    memory: Vec<Instruction>,
    /// Each element in this vector represents all the processes for a particular user
    /// The index of the current user process is the first element in the tuple
    /// The list of processes (the index where each process is at in memory) is the second element.
    user_processes: Vec<(usize, Vec<usize>)>,
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
                    AddressMode::Direct,
                    0,
                    AddressMode::Direct,
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
            user_processes: vec![(0, vec![random_i])],
        }
    }

    pub fn get_memory(&self) -> &Vec<Instruction> {
        &self.memory
    }

    pub fn get_user_processes(&self) -> &Vec<(usize, Vec<usize>)> {
        &self.user_processes
    }

    pub fn get_cur_user(&self) -> usize {
        self.cur_user
    }

    /// Runs one iteration of the virtual machine
    pub fn cycle(&mut self) {
        // The index of the current instruction in memory
        let (cur_process, processes) = &mut self.user_processes[self.cur_user];

        // Get the program counter
        let pc = processes[*cur_process];

        // Get the current instruction
        let instruction = self.memory[pc];

        let memory_len = self.memory.len();

        // Advance the PC by 1 for this process (for this user)
        processes[*cur_process] = (pc + 1) % memory_len;

        use OpCode::*;

        // Run different code for each instruction
        match instruction.op_code {
            MOV => handlers::mov(instruction, pc, memory_len, &mut self.memory),
            DAT => unimplemented!("This should kill current process"),
            JMP => {
                let new_addr = handlers::jmp(instruction, pc, memory_len, &mut self.memory);
                // Set the program counter for this process to the new address
                processes[*cur_process] = new_addr;
            }
            SPL => {
                let new_addr = handlers::spl(instruction, pc, memory_len, &mut self.memory);

                processes.push(new_addr);
            }
        }

        // Advance the user process counter
        *cur_process = (*cur_process + 1) % processes.len();
        // Advance the user counter
        self.cur_user = (self.cur_user + 1) % self.user_processes.len();
    }
}
