use std::io::{stdin, Read};

/// The States struct is used to represent each component of the Universal Machine.
///
/// <br>
///
/// The Universal Machine consist of 8-purpose registers, a collection of memory
/// (Allocated/Deallocated), and a program counter.
pub struct State {
    registers: [u32; 8],
    allocated_memory: Vec<Vec<u32>>,
    freed_memory: Vec<u32>,
    prog_counter: usize,
}

impl State {
    /// Creates a new empty instance of the State Struct.
    pub fn new() -> Self {
        Self {
            registers: [0_u32; 8],
            allocated_memory: Vec::new(),
            freed_memory: Vec::new(),
            prog_counter: 0,
        }
    }

    /// Adds the instruction set to the state object representing the UM state.
    ///
    /// <br>
    ///
    /// # Argument:
    /// * `instruction_set`: the set of instruction code words stored as vec of u32
    pub fn boot_up_instructions(&mut self, instruction_set: Vec<u32>) {
        self.allocated_memory.push(instruction_set)
    }

    /// Gets the instruction that should be decoded at the current point in time in the UM.
    /// After the instruction is extracted, the program counter is to the next instruction id.
    /// Returns the instruction as a code word represented as u32.
    pub unsafe fn get_instruction(&mut self) -> u32 {
        let inst = self
            .allocated_memory
            .get_unchecked(0)
            .get_unchecked(self.prog_counter);
        self.prog_counter += 1;
        return *inst;
    }

    /// This functions represents a Conditional Move, which the UM identifies as Opcode 0.
    /// If the values at register position C is not 0, then the register a position A takes
    /// the value of the register at position B.
    ///
    /// <br>
    ///
    /// # Argument:
    /// * `a`: The index of the register at position A.
    /// * `b`: The index of the register at position B.
    /// * `c`: The index of the register at position C.
    /// * `state`: The struct representing the current state of the UM.
    pub fn cmov(&mut self, a: u32, b: u32, c: u32) {
        if self.registers[c as usize] != 0 {
            self.registers[a as usize] = self.registers[b as usize]
        }
    }

    /// Loads to register at position A the value of the allocated memory at row as the value
    /// been hold in register B, and column as the value been hold in register C.
    ///
    /// <br>
    ///
    /// # Argument:
    /// * `a`: The index of the register at position A.
    /// * `b`: The index of the register at position B.
    /// * `c`: The index of the register at position C.
    /// * `state`: The struct representing the current state of the UM.
    pub unsafe fn seg_load(&mut self, a: u32, b: u32, c: u32) {
        self.registers[a as usize] = *self
            .allocated_memory
            .get_unchecked(self.registers[b as usize] as usize)
            .get_unchecked(self.registers[c as usize] as usize)
    }

    /// Stores the value of the register at position C inside the allocated memory at row as
    /// value at register A, and column as value at register B.
    ///
    /// <br>
    ///
    /// # Argument:
    /// * `a`: The index of the register at position A.
    /// * `b`: The index of the register at position B.
    /// * `c`: The index of the register at position C.
    /// * `state`: The struct representing the current state of the UM.
    pub unsafe fn seg_store(&mut self, a: u32, b: u32, c: u32) {
        self.allocated_memory[self.registers[a as usize] as usize]
            [self.registers[b as usize] as usize] = self.registers[c as usize]
    }

    /// Adds the value of the register at position B plus the value of the register at position C.
    /// The final result is stored in the register at position A.
    ///
    /// <br>
    ///
    /// # Argument:
    /// * `a`: The index of the register at position A.
    /// * `b`: The index of the register at position B.
    /// * `c`: The index of the register at position C.
    /// * `state`: The struct representing the current state of the UM.
    pub fn add(&mut self, a: u32, b: u32, c: u32) {
        self.registers[a as usize] =
            self.registers[b as usize].wrapping_add(self.registers[c as usize])
    }

    /// Multiplies the value of the register at position B times the value of the register at position C.
    /// The final result is stored in the register at position A.
    ///
    /// <br>
    ///
    /// # Argument:
    /// * `a`: The index of the register at position A.
    /// * `b`: The index of the register at position B.
    /// * `c`: The index of the register at position C.
    /// * `state`: The struct representing the current state of the UM.
    pub fn mul(&mut self, a: u32, b: u32, c: u32) {
        self.registers[a as usize] =
            self.registers[b as usize].wrapping_mul(self.registers[c as usize])
    }

    /// Divides the value of the register at position B by the value of the register at position C.
    /// The final result is stored in the register at position A.
    ///
    /// <br>
    ///
    /// # Argument:
    /// * `a`: The index of the register at position A.
    /// * `b`: The index of the register at position B.
    /// * `c`: The index of the register at position C.
    /// * `state`: The struct representing the current state of the UM.
    pub fn div(&mut self, a: u32, b: u32, c: u32) {
        self.registers[a as usize] =
            self.registers[b as usize].wrapping_div(self.registers[c as usize])
    }

    /// Computes the result of the bitwise negation of the value in the register at position B bitwise
    /// AND the value in the register at position C, and stores it in the register at position A.
    ///
    /// <br>
    ///
    /// # Argument:
    /// * `a`: The index of the register at position A.
    /// * `b`: The index of the register at position B.
    /// * `c`: The index of the register at position C.
    /// * `state`: The struct representing the current state of the UM.
    pub fn b_nand(&mut self, a: u32, b: u32, c: u32) {
        self.registers[a as usize] = !(self.registers[b as usize] & self.registers[c as usize])
    }

    /// Terminates the execution of the UM. Prints exit code 0 to standard out.
    pub fn halt(&self) {
        std::process::exit(0)
    }

    /// Creates a new allocation of words equal to the value been hold in the register at position
    /// C, and places the allocation identifier in the register at position B.
    ///
    /// <br>
    ///
    /// # Argument:
    /// * `b`: The index of the register at position B.
    /// * `c`: The index of the register at position C.
    /// * `state`: The struct representing the current state of the UM.
    pub fn map_seg(&mut self, b: u32, c: u32) {
        let new_allocation = vec![0_u32; self.registers[c as usize] as usize];
        match self.freed_memory.pop() {
            Some(idx) => {
                self.allocated_memory[idx as usize] = new_allocation;
                self.registers[b as usize] = idx;
            }
            None => {
                self.allocated_memory.push(new_allocation);
                self.registers[b as usize] = (self.allocated_memory.len() - 1) as u32;
            }
        }
    }

    /// Frees the memory inside the allocated memory at row denoted by the value inside
    /// the register at position C.
    ///
    /// <br>
    ///
    /// # Argument:
    /// * `c`: The index of the register at position C.
    /// * `state`: The struct representing the current state of the UM.
    pub fn unmap_seg(&mut self, c: u32) {
        let freed_location = self.registers[c as usize];
        self.allocated_memory[freed_location as usize].clear();
        self.freed_memory.push(freed_location)
    }

    /// Takes the values in the register at position C, and displays the value in
    /// std-out.
    ///
    /// <br>
    ///
    /// # Argument:
    /// * `c`: The index of the register at position C.
    /// * `state`: The struct representing the current state of the UM.///
    pub fn output(&mut self, c: u32) {
        match u8::try_from(self.registers[c as usize]) {
            Ok(val) => print!("{}", val as char),
            Err(error) => panic!("Value is not in range {:?}", error),
        }
    }

    /// Read from std-in and stores the values in the register at position C.
    ///
    /// <br>
    ///
    /// # Argument:
    /// * `c`: The index of the register at position C.
    /// * `state`: The struct representing the current state of the UM.
    pub fn input(&mut self, c: u32) {
        match stdin().bytes().next() {
            Some(input) => self.registers[c as usize] = input.unwrap() as u32,
            None => self.registers[c as usize] = !0_u32,
        }
    }

    ///
    ///
    /// <br>
    ///
    /// # Argument:
    /// * `b`: The index of the register at position B.
    /// * `c`: The index of the register at position C.
    /// * `state`: The struct representing the current state of the UM.
    pub unsafe fn load_prog(&mut self, b: u32, c: u32) {
        let location = self.registers[b as usize] as usize;
        if location == 0 {
            self.prog_counter = self.registers[c as usize] as usize;
        } else {
            self.allocated_memory[0] = self.allocated_memory[location].clone();
            self.prog_counter = self.registers[c as usize] as usize;
        }
    }

    /// Load a pre-define value in the register at position location.
    ///
    /// <br>
    ///
    /// # Argument:
    /// * `location`: The register where the value should be stored.
    /// * `val`: The value to store.
    /// * `state`: The struct representing the current state of the UM.
    pub unsafe fn load_val(&mut self, location: u32, val: u32) {
        self.registers[location as usize] = val
    }
}