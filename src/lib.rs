#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InstructionType {
    /// Copy instruction from source (a) to address (b)
    MOV,
    ADD,
    DAT, // JMP, DAT
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Instruction {
    pub instruction_type: InstructionType,
    pub reg_a: usize,
    pub reg_b: usize,
}

impl Instruction {
    pub fn new(instruction_type: InstructionType, reg_a: usize, reg_b: usize) -> Instruction {
        Instruction {
            instruction_type,
            reg_a,
            reg_b,
        }
    }
}

#[derive(Debug)]
pub struct VirtualMachine {
    memory: Vec<Instruction>,
    /// A list of processes of a user, it shows what the index of the next instruction in memory should (the index is the process id)
    processes: Vec<usize>,
    /// The id of the process which should run next
    cur_process: usize,
}

impl VirtualMachine {
    pub fn new(size: usize, program: Vec<Instruction>) -> VirtualMachine {
        let mut memory: Vec<Instruction> = (0..size)
            .map(|_| Instruction::new(InstructionType::DAT, 0, 0))
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
            cur_process: 0,
            processes: vec![random_i],
        }
    }

    pub fn get_memory(&self) -> &Vec<Instruction> {
        &self.memory
    }

    pub fn cycle(&mut self) {
        // The index of the current instruction in memory
        let cur_index = self.processes[self.cur_process];

        // Get the current instruction
        let Instruction {
            instruction_type: i_type,
            reg_a,
            reg_b,
        } = self.memory[cur_index];

        let memory_len = self.memory.len();

        use InstructionType::*;

        // Next time run the instruction 1 ahead of this
        self.processes[self.cur_process] = (self.processes[self.cur_process] + 1) % memory_len;

        // Run different code for each instruction
        match i_type {
            // Copy instruction from source to address
            MOV => {
                let source = self.memory[(reg_a + cur_index) % memory_len];
                self.memory[(reg_b + cur_index) % memory_len] = source;
            }
            ADD => unimplemented!("ADD"),
            DAT => unimplemented!("DAT"),
        }

        self.cur_process = (self.cur_process + 1) % self.processes.len()
    }
}
