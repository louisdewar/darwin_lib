use crate::{handlers, AddressMode, Instruction, Modifier, OpCode};

use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct MatchSettings {
    /// The minimum separation between warriors when they are loaded
    pub min_separation: usize,
    /// The maximum number of processes for an individual user
    pub max_processes: usize,
    /// The size of the core
    pub core_size: usize,
}

impl Default for MatchSettings {
    fn default() -> MatchSettings {
        MatchSettings {
            min_separation: 100,
            max_processes: 8000,
            core_size: 8000,
        }
    }
}

#[derive(Debug)]
pub struct VirtualMachine {
    memory: Vec<Instruction>,
    /// Each element in this vector represents the queue of a user's processes
    /// The next instruction index is at the front of the queue for each user
    users_pcs: Vec<VecDeque<usize>>,
    /// The id of the user whose process should run next
    cur_user: usize,
    /// The maximum number of processes for an individual user
    max_processes: usize,
}

fn generate_random_insertion_points(
    size: usize,
    programs: &[Vec<Instruction>],
    min_separation: usize,
) -> Vec<usize> {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    #[derive(Debug)]
    struct Block {
        start: usize,
        len: usize,
    }

    // The First program is inserted `min_separation` instructions into memory
    let mut indices = vec![min_separation];

    // A list of blocks of memory that are free
    let mut free_blocks = {
        // The total space occupied by the first program plus padding
        let first_program_total_len = min_separation * 2 + programs[0].len();

        // Therefore the start of the free block of memory is equivalent to the total length of the first program
        // and the length is the core size - total program length
        vec![Block {
            start: first_program_total_len % size,
            len: size.saturating_sub(first_program_total_len),
        }]
    };

    for program in programs.iter().skip(1) {
        println!("{:?}", free_blocks);

        let total_free_spaces: usize = free_blocks
            .iter()
            .map(|block| {
                if block.len < program.len() {
                    0
                } else {
                    // If there is exactly enough room (block - program == 0) then there is room
                    // for one program hence +1
                    (block.len - program.len()) + 1
                }
            })
            .sum();

        if total_free_spaces == 0 {
            panic!("Not enough room to insert all the programs");
        }

        let mut n: usize = rng.gen_range(0, total_free_spaces);

        for i in 0..free_blocks.len() {
            let block = &free_blocks[i];

            if n > block.len {
                // Program should be inserted outside this block, so continue to the next and deduct the length
                n -= block.len;
            } else {
                // Program is within this block
                indices.push(block.start + n);

                // The block before this program now has length n - min_separation or 0 if there isn't enough room
                free_blocks.insert(
                    i,
                    Block {
                        start: free_blocks[i].start,
                        len: n.saturating_sub(min_separation),
                    },
                );

                // The distance from the previous start to the new start
                let new_start_distance = n + (2 * min_separation) + program.len();
                // If there is enough room for another free_block after this new program and its padding
                if free_blocks[i + 1].len > new_start_distance {
                    free_blocks[i + 1].start = (free_blocks[i].start + new_start_distance) % size;
                } else {
                    // Remove the block since there isn't enough room
                    free_blocks.remove(i + 1);
                }

                // Move on to the next program
                break;
            }
        }
    }

    indices
}

fn generate_empty_memory(size: usize) -> Vec<Instruction> {
    (0..size)
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
        .collect()
}

fn handle_pre_decrement(
    reg: isize,
    mode: AddressMode,
    cur_address: usize,
    max: usize,
    memory: &mut [Instruction],
) {
    use handlers::relative_address;
    use AddressMode::*;

    match mode {
        PreDecrementIndirectA => {
            let index = relative_address(max, cur_address, reg);
            memory[index].a_reg = (memory[index].a_reg - 1 + max as isize) % max as isize;
        }
        PreDecrementIndirectB => {
            let index = relative_address(max, cur_address, reg);
            memory[index].b_reg = (memory[index].b_reg - 1 + max as isize) % max as isize;
        }
        _ => {}
    }
}

fn handle_post_increment(
    reg: isize,
    mode: AddressMode,
    cur_address: usize,
    max: usize,
    memory: &mut [Instruction],
) {
    use handlers::relative_address;
    use AddressMode::*;

    match mode {
        PostIncrementIndirectA => {
            let index = relative_address(max, cur_address, reg);
            memory[index].a_reg = (memory[index].a_reg + 1) % max as isize;
        }
        PostIncrementIndirectB => {
            let index = relative_address(max, cur_address, reg);
            memory[index].b_reg = (memory[index].b_reg + 1) % max as isize;
        }
        _ => {}
    }
}

impl VirtualMachine {
    /// Creates a new VM with specified programs and match settings
    /// This inserts programs randomly into memory
    pub fn new_battle(
        programs: &[Vec<Instruction>],
        match_settings: &MatchSettings,
    ) -> VirtualMachine {
        let mut memory = generate_empty_memory(match_settings.core_size);

        let indices = generate_random_insertion_points(
            match_settings.core_size,
            &programs,
            match_settings.min_separation,
        );

        for (start_index, program) in indices.iter().zip(programs.iter()) {
            for (instruction_i, instruction) in program.iter().enumerate() {
                memory[(start_index + instruction_i) % match_settings.core_size] = *instruction
            }
        }

        VirtualMachine {
            memory,
            cur_user: 0,
            users_pcs: (0..programs.len())
                .map(|i| VecDeque::from(vec![indices[i]]))
                .collect(),
            max_processes: match_settings.max_processes,
        }
    }

    /// Creates a new VM with one program inserted at index 0
    /// This is designed to be used as an actual VM, not a contest
    pub fn new_simple(size: usize, program: Vec<Instruction>) -> VirtualMachine {
        assert!(
            size >= program.len(),
            "Program length was greater than memory size"
        );

        let mut memory = generate_empty_memory(size);

        for (i, instruction) in program.iter().enumerate() {
            memory[i] = *instruction
        }

        VirtualMachine {
            memory,
            cur_user: 0,
            users_pcs: vec![VecDeque::from(vec![0])],
            max_processes: 8000,
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

        // Handle pre-decrement address modes:
        handle_pre_decrement(
            instruction.a_reg,
            instruction.a_mode,
            pc,
            memory_len,
            &mut self.memory,
        );
        handle_pre_decrement(
            instruction.b_reg,
            instruction.b_mode,
            pc,
            memory_len,
            &mut self.memory,
        );

        use OpCode::*;

        // Run different code for each instruction
        match instruction.op_code {
            MOV => handlers::mov(instruction, pc, memory_len, &mut self.memory),
            ADD => handlers::add(instruction, pc, memory_len, &mut self.memory),
            SUB => handlers::sub(instruction, pc, memory_len, &mut self.memory),
            MUL => handlers::mul(instruction, pc, memory_len, &mut self.memory),
            DIV => {
                // Will remove the last queued process if a division by zero occurs
                if !handlers::div(instruction, pc, memory_len, &mut self.memory) {
                    process_queue.pop_back().unwrap();
                }
            }
            MOD => {
                // Will remove the last queued process if a division by zero occurs
                if !handlers::modulo(instruction, pc, memory_len, &mut self.memory) {
                    process_queue.pop_back().unwrap();
                }
            }
            DAT => {
                // Remove the last queued process (kill it)
                process_queue.pop_back().unwrap();
            }
            JMP => {
                let new_addr = handlers::jmp(instruction, pc, memory_len, &self.memory);
                // Override the program counter for this process to the new address
                *(process_queue.back_mut().unwrap()) = new_addr;
            }
            JMZ => {
                if let Some(new_addr) = handlers::jmz(instruction, pc, memory_len, &self.memory) {
                    *(process_queue.back_mut().unwrap()) = new_addr;
                }
            }
            JMN => {
                if let Some(new_addr) = handlers::jmn(instruction, pc, memory_len, &self.memory) {
                    *(process_queue.back_mut().unwrap()) = new_addr;
                }
            }
            DJN => {
                if let Some(new_addr) = handlers::djn(instruction, pc, memory_len, &mut self.memory)
                {
                    *(process_queue.back_mut().unwrap()) = new_addr;
                }
            }
            SPL => {
                // If max processes is reached then this command behaves like NOP
                if process_queue.len() < self.max_processes {
                    let new_addr = handlers::spl(instruction, pc, memory_len, &self.memory);

                    // Queue an additional process
                    process_queue.push_back(new_addr);
                }
            }
            SEQ => {
                if handlers::seq(instruction, pc, memory_len, &self.memory) {
                    *(process_queue.back_mut().unwrap()) += 1
                }
            }
            SNE => {
                if handlers::sne(instruction, pc, memory_len, &self.memory) {
                    *(process_queue.back_mut().unwrap()) += 1
                }
            }
            SLT => {
                if handlers::slt(instruction, pc, memory_len, &self.memory) {
                    *(process_queue.back_mut().unwrap()) += 1
                }
            }
            // Does nothing
            NOP => {}
        }

        // Handle post-increment address modes:
        handle_post_increment(
            instruction.a_reg,
            instruction.a_mode,
            pc,
            memory_len,
            &mut self.memory,
        );
        handle_post_increment(
            instruction.b_reg,
            instruction.b_mode,
            pc,
            memory_len,
            &mut self.memory,
        );

        // Advance the user counter
        self.cur_user = (self.cur_user + 1) % self.users_pcs.len();
    }
}
