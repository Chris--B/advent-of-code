//! Intcode Virtual Machine
//!
//! This module implements a virtual machine to execute and manage intcode
//! programs

use smallvec::SmallVec;

use std::fmt;
use std::fmt::Write;

/// Individual entries in the VM's memory are represented as "atom" types
/// This can be a signed or unsigned integer of unspecified size.
/// We pick i64 to make sure have enough values for anything we want.
pub type Atom = i32;

/// The reason execution has stopped
///
/// The Vm may stop running while it waits for user input when executing `IN` instructions
#[derive(Debug, Clone)]
pub enum VmStopReason {
    /// A `HALT` instruction was executed and exection and will not continue
    Halted { ip: usize },

    /// An `IN` instruction was executed and the input buffer is empty
    BlockedOnInput { ip: usize },

    /// A known instruction was executed with illegal arguments, parameter modes, or some other
    /// invalid config.
    /// The behavior either doesn't make sense or is not well defined and execution cannot continue.
    IllegalInstruction { ip: usize, what: String },

    /// The vm attempted to decode an unrecognized opcode and cannot continue
    UnknownInstruction { ip: usize, what: String },
}

/// Consider stop reasons eqvuilent if they stop:
///     - at the same address (ip)
///     - for the same reason
/// Their error messages are ignored
impl PartialEq for VmStopReason {
    fn eq(&self, other: &VmStopReason) -> bool {
        use VmStopReason::*;

        match (self, other) {
            (Halted { ip: ip0, .. }, Halted { ip: ip1, .. }) => ip0 == ip1,
            (BlockedOnInput { ip: ip0, .. }, BlockedOnInput { ip: ip1, .. }) => ip0 == ip1,
            (IllegalInstruction { ip: ip0, .. }, IllegalInstruction { ip: ip1, .. }) => ip0 == ip1,
            (UnknownInstruction { ip: ip0, .. }, UnknownInstruction { ip: ip1, .. }) => ip0 == ip1,
            _ => false,
        }
    }
}

impl VmStopReason {
    fn mem_out_of_bounds(ip: usize, addr: usize, addr_size: usize, kind: &str) -> VmStopReason {
        VmStopReason::IllegalInstruction {
            ip,
            what: format!(
                r#"OutOfBoundsMemoryAccess {{
    ip:        {},
    addr:      {},
    addr_size: {},
    kind:      {},
}}
"#,
                ip, addr as usize, addr_size, kind
            ),
        }
    }
}

/// The state for a paused, running, or halted Intcode Vm
#[derive(Clone)]
pub struct Vm {
    /// Instruction Pointer
    ///
    /// Points to the Atom offset in memory that the VM is about to execute
    ip: usize,

    /// Tick count
    ///
    /// This tick is increased by 1 or more everytime an instruction is executed
    ticks: usize,

    /// Main Memory for the VM
    ///
    /// Instructions and data co-exist in this space, and instructions can
    /// modify all of this memory at any time.
    mem: Vec<Atom>,

    /// Pending input values for an `IN` instruction
    ///
    /// When a user specifies an input value, it is first pushed into this
    /// buffer. When it's used, it is removed.
    /// Users can specify multiple input without running instructions, so this
    /// buffer exists to store those values until they're used.
    input_buffer: SmallVec<[Atom; 16]>,

    /// Output values from an `OUT` instruction
    /// A single program can produce many output values
    output_buffer: SmallVec<[Atom; 16]>,
}

impl Vm {
    /// Construct a new Vm with initial memory
    ///
    /// The vm will begin executing int code at index 0
    pub fn with_memory_from_slice(mem: &[Atom]) -> Vm {
        let mem: Vec<Atom> = mem.iter().copied().collect();
        Vm::with_memory(mem)
    }

    /// Construct a new Vm with initial memory
    ///
    /// The vm will begin executing int code at index 0
    pub fn with_memory(mem: Vec<Atom>) -> Vm {
        Vm {
            ticks: 0,
            ip: 0,
            mem,
            input_buffer: SmallVec::new(),
            output_buffer: SmallVec::new(),
        }
    }

    /// Construct a new Vm with no memory
    ///
    /// Running this without calling `Vm::reset()` will error.
    /// Use this if you expect to call reset() before using the vm.
    pub fn empty() -> Vm {
        Vm {
            ticks: 0,
            ip: 0,
            mem: vec![],
            input_buffer: SmallVec::new(),
            output_buffer: SmallVec::new(),
        }
    }

    /// Reset the Vm to a fresh state
    ///
    /// Prefer this over creating and dropping instances in a loop
    pub fn reset(&mut self, new_mem: &[Atom]) {
        // Re-initialize internal states
        self.ip = 0;
        self.ticks = 0;
        self.mem.clear();
        self.input_buffer.clear();
        self.output_buffer.clear();

        // We need to resize `self.mem` so that it exactly matches the size of `new_mem`,
        // but `Vec::resize()` wastes cycles by inserting some value.
        // We immediately overwrite that value, and benchmarks show the compiler doesn't catch that.
        // Therefore, we reserve any additional space we need and and force the length to match
        // This is generally `unsafe`, but we know that it's safe in this intance.
        unsafe {
            let additional = usize::saturating_sub(new_mem.len(), self.mem.capacity());
            self.mem.reserve(additional);
            self.mem.set_len(new_mem.len());
        }

        self.mem[..new_mem.len()].copy_from_slice(new_mem);
    }

    /// Retrieve the current instruction pointer
    pub fn ip(&self) -> usize {
        self.ip
    }

    pub fn ticks(&self) -> usize {
        self.ticks
    }

    /// Retrieve a slice of the Vm's current memory
    pub fn mem(&self) -> &[Atom] {
        &self.mem
    }

    /// Retrieve a mutable slice of the Vm's current memory
    ///
    /// Be careful! You can modify any atom anywhere and break what may have otherwise been totally
    /// valid intcode input!
    pub fn mem_mut(&mut self) -> &mut [Atom] {
        &mut self.mem
    }

    /// Input a value to the machine
    ///
    /// This will buffer the value until an `IN` instruction is executed, that will then use this
    /// value as its input.
    /// Multiple input values can be inserted. They will be processed in the order that this
    /// method is called
    pub fn add_input(&mut self, atom: Atom) {
        self.input_buffer.push(atom);
    }

    /// Query unused input atoms
    ///
    /// These atoms have been supplied to the vm for `IN` instructions, but have not been
    /// consumed yet.
    pub fn get_unused_input(&self) -> &[Atom] {
        &self.input_buffer
    }

    /// Query output values
    ///
    /// Every value that is sent "out" with an `OUT` instruction is buffered and returned in order.
    pub fn get_output(&self) -> &[Atom] {
        &self.output_buffer
    }

    /// Internal method to read an atom from a vm address
    // Does bounds checking
    fn read_atom(&self, addr: usize) -> Result<Atom, VmStopReason> {
        match self.mem.get(addr) {
            Some(atom) => Ok(*atom),
            None => Err(VmStopReason::mem_out_of_bounds(
                self.ip,
                addr,
                self.mem.len(),
                "read",
            )),
        }
    }

    /// Internal method to write an atom to a vm address
    // Does bounds checking
    fn write_atom(&mut self, addr: usize, atom: Atom) -> Result<(), VmStopReason> {
        match self.mem.get_mut(addr) {
            Some(loc) => {
                *loc = atom;
                Ok(())
            }
            None => Err(VmStopReason::mem_out_of_bounds(
                self.ip,
                addr,
                self.mem.len(),
                "write",
            )),
        }
    }

    /// Run the Vm until it stops
    ///
    /// Returns Ok(self.ip()) if the vm executes `HALT`, otherwise Err() describes what happened.
    pub fn run(&mut self) -> Result<usize, VmStopReason> {
        use crate::opcodes::{Opcode, ParamMode};

        loop {
            self.ticks += 1;

            let ip_atom = self.read_atom(self.ip)?;
            let opcode = Opcode::from_digits(ip_atom % 100);

            // Instruction decoding can iteratively "strip" away values from this as it parses
            // the packed data - e.g. param modes.
            let mut ip_atom_num = ip_atom / 100;

            match opcode {
                Some(Opcode::Add) => {
                    // Fetch input values
                    let args: (Atom, Atom) = {
                        // Extract param values
                        let a0 = {
                            let p0 = self.read_atom(self.ip + 1)?;
                            let m0 = ParamMode::from_digit(ip_atom_num % 10).unwrap();
                            ip_atom_num /= 10;

                            match m0 {
                                // Fetch value from memory
                                ParamMode::Addr => self.read_atom(p0 as usize)?,

                                // Use immediate value
                                ParamMode::Imm => p0,
                            }
                        };

                        let a1 = {
                            let p1 = self.read_atom(self.ip + 2)?;
                            let m1 = ParamMode::from_digit(ip_atom_num % 10).unwrap();
                            ip_atom_num /= 10;

                            match m1 {
                                // Fetch value from memory
                                ParamMode::Addr => self.read_atom(p1 as usize)?,

                                // Use immediate value
                                ParamMode::Imm => p1,
                            }
                        };

                        (a0, a1)
                    };

                    // Verify that the output ParamMode is valid
                    #[cfg(debug_assertions)]
                    {
                        let mode = ParamMode::from_digit(ip_atom_num % 10).unwrap();
                        ip_atom_num /= 10;
                        if mode == ParamMode::Imm {
                            return Err(VmStopReason::IllegalInstruction {
                                ip: self.ip,
                                what: format!(
                                    "Opcode {:?} at {} used illegal \"Imm\" mode on writeout param",
                                    opcode, self.ip
                                ),
                            });
                        }
                    }
                    // Fetch output address
                    let a_out = self.read_atom(self.ip + 3)?;

                    // Write back result
                    self.write_atom(a_out as usize, args.0 + args.1)?;

                    self.ip += 4;
                }
                Some(Opcode::Mul) => {
                    // Fetch input values
                    let args: (Atom, Atom) = {
                        // Extract param values
                        let a0 = {
                            let p0 = self.read_atom(self.ip + 1)?;
                            let m0 = ParamMode::from_digit(ip_atom_num % 10).unwrap();
                            ip_atom_num /= 10;

                            match m0 {
                                // Fetch value from memory
                                ParamMode::Addr => self.read_atom(p0 as usize)?,

                                // Use immediate value
                                ParamMode::Imm => p0,
                            }
                        };

                        let a1 = {
                            let p1 = self.read_atom(self.ip + 2)?;
                            let m1 = ParamMode::from_digit(ip_atom_num % 10).unwrap();
                            ip_atom_num /= 10;

                            match m1 {
                                // Fetch value from memory
                                ParamMode::Addr => self.read_atom(p1 as usize)?,

                                // Use immediate value
                                ParamMode::Imm => p1,
                            }
                        };

                        (a0, a1)
                    };

                    // Verify that the output ParamMode is valid
                    #[cfg(debug_assertions)]
                    {
                        let mode = ParamMode::from_digit(ip_atom_num % 10).unwrap();
                        ip_atom_num /= 10;
                        if mode == ParamMode::Imm {
                            return Err(VmStopReason::IllegalInstruction {
                                ip: self.ip,
                                what: format!(
                                    "Opcode {:?} at {} used illegal \"Imm\" mode on writeout param",
                                    opcode, self.ip
                                ),
                            });
                        }
                    }
                    // Fetch output address
                    let a_out = self.read_atom(self.ip + 3)?;

                    // Write back result
                    self.write_atom(a_out as usize, args.0 * args.1)?;

                    self.ip += 4;
                }
                Some(Opcode::In) => {
                    let a_out = self.read_atom(self.ip + 1)? as usize;

                    if self.input_buffer.is_empty() {
                        return Err(VmStopReason::BlockedOnInput { ip: self.ip });
                    }

                    // Fetch the next input value
                    let value = self.input_buffer[0];
                    self.input_buffer.remove(0);

                    // and write it to memory
                    self.write_atom(a_out, value)?;

                    self.ip += 2;
                }

                Some(Opcode::Out) => {
                    // Fetch value to output
                    let a0 = {
                        let p0 = self.read_atom(self.ip + 1)?;
                        let m0 = ParamMode::from_digit(ip_atom_num % 10).unwrap();
                        ip_atom_num /= 10;

                        match m0 {
                            // Fetch value from memory
                            ParamMode::Addr => self.read_atom(p0 as usize)?,

                            // Use immediate value
                            ParamMode::Imm => p0,
                        }
                    };

                    // Write to "output"
                    self.output_buffer.push(a0);

                    self.ip += 2;
                }
                Some(Opcode::Hlt) => return Ok(self.ip),
                None => {
                    return Err(VmStopReason::UnknownInstruction {
                        ip: self.ip,
                        what: format!(
                            "Unrecognized opcode header at address {}: {:?}",
                            self.ip, opcode
                        ),
                    })
                }
            }

            assert_eq!(
                ip_atom_num, 0,
                "opcode {:?} didn't use all of its param modes",
                opcode
            );
        }
    }
}

pub fn pretty_fmt_memory(mem: &[Atom]) -> Result<String, fmt::Error> {
    let mut s = String::new();

    writeln!(&mut s, "==== INTCODE MEMORY ====")?;

    for line in mem.chunks(4) {
        write!(&mut s, "{:>4}", line[0])?;

        for atom in &line[1..] {
            write!(&mut s, " {:>4}", atom)?;
        }
        writeln!(&mut s)?;
    }

    Ok(s)
}

impl fmt::Debug for Vm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("intcode::Vm")
            .field("ip", &self.ip)
            .field("ticks", &self.ticks)
            .field("input_buffer", &self.input_buffer)
            .field("output_buffer", &self.output_buffer)
            .field("mem", &pretty_fmt_memory(&self.mem)?)
            .finish()
    }
}

#[cfg(test)]
mod day_02 {
    use super::*;

    #[test]
    fn check_example_0() {
        let intcode = vec![1, 0, 0, 0, 99];
        let mut vm = Vm::with_memory_from_slice(&intcode);
        assert_eq!(vm.run(), Ok(4));
        assert_eq!(vm.mem(), [2, 0, 0, 0, 99]);
    }

    #[test]
    fn check_example_1() {
        let intcode = vec![2, 3, 0, 3, 99];
        let mut vm = Vm::with_memory_from_slice(&intcode);
        assert_eq!(vm.run(), Ok(4));
        assert_eq!(vm.mem(), [2, 3, 0, 6, 99]);
    }

    #[test]
    fn check_example_2() {
        let intcode = vec![2, 4, 4, 5, 99, 0];
        let mut vm = Vm::with_memory_from_slice(&intcode);
        assert_eq!(vm.run(), Ok(4));
        assert_eq!(vm.mem(), [2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn check_example_3() {
        let intcode = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let mut vm = Vm::with_memory_from_slice(&intcode);
        assert_eq!(vm.run(), Ok(8));
        assert_eq!(vm.mem(), [30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn check_part1_input() {
        let mut intcode = vec![
            1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 10, 19, 1, 19, 6, 23, 2, 23, 13,
            27, 1, 27, 5, 31, 2, 31, 10, 35, 1, 9, 35, 39, 1, 39, 9, 43, 2, 9, 43, 47, 1, 5, 47,
            51, 2, 13, 51, 55, 1, 55, 9, 59, 2, 6, 59, 63, 1, 63, 5, 67, 1, 10, 67, 71, 1, 71, 10,
            75, 2, 75, 13, 79, 2, 79, 13, 83, 1, 5, 83, 87, 1, 87, 6, 91, 2, 91, 13, 95, 1, 5, 95,
            99, 1, 99, 2, 103, 1, 103, 6, 0, 99, 2, 14, 0, 0,
        ];
        assert_eq!(
            intcode[108], 99,
            "Expected an HLT instruction in the intcode"
        );

        intcode[1] = 12;
        intcode[2] = 02;

        let mut vm = Vm::with_memory(intcode);
        assert_eq!(
            vm.run(),
            Ok(108),
            "Expected to halt with HLT at address 108"
        );
        assert_eq!(vm.mem()[0], 3790645, "Expected solution to puzzle 2 part1");
    }

    #[test]
    fn check_part2_input() {
        let mut intcode = vec![
            1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 10, 19, 1, 19, 6, 23, 2, 23, 13,
            27, 1, 27, 5, 31, 2, 31, 10, 35, 1, 9, 35, 39, 1, 39, 9, 43, 2, 9, 43, 47, 1, 5, 47,
            51, 2, 13, 51, 55, 1, 55, 9, 59, 2, 6, 59, 63, 1, 63, 5, 67, 1, 10, 67, 71, 1, 71, 10,
            75, 2, 75, 13, 79, 2, 79, 13, 83, 1, 5, 83, 87, 1, 87, 6, 91, 2, 91, 13, 95, 1, 5, 95,
            99, 1, 99, 2, 103, 1, 103, 6, 0, 99, 2, 14, 0, 0,
        ];

        intcode[1] = 65;
        intcode[2] = 77;

        let mut vm = Vm::with_memory(intcode);
        assert_eq!(
            vm.run(),
            Ok(108),
            "Expected to halt with HLT at address 108"
        );
        assert_eq!(vm.mem()[0], 19690720, "Expected solution to puzzle 2 part2");
    }
}

#[cfg(test)]
mod day_05 {
    use super::*;

    #[test]
    fn check_echo() {
        for input in 100..=110 {
            let intcode = vec![3, 0, 4, 0, 99];
            let mut vm = Vm::with_memory(intcode);

            let why = vm.run();
            assert_eq!(
                why,
                Err(VmStopReason::BlockedOnInput { ip: 0 }),
                "vm didn't stop to wait on input"
            );

            vm.add_input(input);

            let why = vm.run();
            assert_eq!(why, Ok(4), "vm finished executing at an unexpected address");
            assert_eq!(
                vm.get_output(),
                &[input],
                "vm output didn't match expected output ({})",
                input
            );
        }
    }

    #[test]
    fn check_part1_input() {
        let intcode = vec![
            3, 225, 1, 225, 6, 6, 1100, 1, 238, 225, 104, 0, 101, 20, 183, 224, 101, -63, 224, 224,
            4, 224, 1002, 223, 8, 223, 101, 6, 224, 224, 1, 223, 224, 223, 1101, 48, 40, 225, 1101,
            15, 74, 225, 2, 191, 40, 224, 1001, 224, -5624, 224, 4, 224, 1002, 223, 8, 223, 1001,
            224, 2, 224, 1, 223, 224, 223, 1101, 62, 60, 225, 1102, 92, 15, 225, 102, 59, 70, 224,
            101, -885, 224, 224, 4, 224, 1002, 223, 8, 223, 101, 7, 224, 224, 1, 224, 223, 223, 1,
            35, 188, 224, 1001, 224, -84, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 2, 224, 1, 223,
            224, 223, 1001, 66, 5, 224, 1001, 224, -65, 224, 4, 224, 102, 8, 223, 223, 1001, 224,
            3, 224, 1, 223, 224, 223, 1002, 218, 74, 224, 101, -2960, 224, 224, 4, 224, 1002, 223,
            8, 223, 1001, 224, 2, 224, 1, 224, 223, 223, 1101, 49, 55, 224, 1001, 224, -104, 224,
            4, 224, 102, 8, 223, 223, 1001, 224, 6, 224, 1, 224, 223, 223, 1102, 43, 46, 225, 1102,
            7, 36, 225, 1102, 76, 30, 225, 1102, 24, 75, 224, 101, -1800, 224, 224, 4, 224, 102, 8,
            223, 223, 101, 2, 224, 224, 1, 224, 223, 223, 1101, 43, 40, 225, 4, 223, 99, 0, 0, 0,
            677, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1105, 0, 99999, 1105, 227, 247, 1105, 1, 99999,
            1005, 227, 99999, 1005, 0, 256, 1105, 1, 99999, 1106, 227, 99999, 1106, 0, 265, 1105,
            1, 99999, 1006, 0, 99999, 1006, 227, 274, 1105, 1, 99999, 1105, 1, 280, 1105, 1, 99999,
            1, 225, 225, 225, 1101, 294, 0, 0, 105, 1, 0, 1105, 1, 99999, 1106, 0, 300, 1105, 1,
            99999, 1, 225, 225, 225, 1101, 314, 0, 0, 106, 0, 0, 1105, 1, 99999, 1008, 226, 226,
            224, 1002, 223, 2, 223, 1005, 224, 329, 1001, 223, 1, 223, 8, 226, 677, 224, 102, 2,
            223, 223, 1006, 224, 344, 1001, 223, 1, 223, 1007, 226, 677, 224, 1002, 223, 2, 223,
            1005, 224, 359, 101, 1, 223, 223, 1008, 677, 226, 224, 102, 2, 223, 223, 1006, 224,
            374, 1001, 223, 1, 223, 1107, 226, 677, 224, 1002, 223, 2, 223, 1006, 224, 389, 1001,
            223, 1, 223, 107, 677, 677, 224, 1002, 223, 2, 223, 1006, 224, 404, 101, 1, 223, 223,
            1007, 226, 226, 224, 1002, 223, 2, 223, 1006, 224, 419, 101, 1, 223, 223, 7, 677, 226,
            224, 1002, 223, 2, 223, 1005, 224, 434, 1001, 223, 1, 223, 1007, 677, 677, 224, 1002,
            223, 2, 223, 1006, 224, 449, 101, 1, 223, 223, 107, 226, 226, 224, 1002, 223, 2, 223,
            1006, 224, 464, 1001, 223, 1, 223, 1108, 677, 677, 224, 1002, 223, 2, 223, 1005, 224,
            479, 101, 1, 223, 223, 8, 677, 226, 224, 1002, 223, 2, 223, 1006, 224, 494, 101, 1,
            223, 223, 7, 226, 677, 224, 102, 2, 223, 223, 1005, 224, 509, 1001, 223, 1, 223, 1107,
            677, 226, 224, 102, 2, 223, 223, 1005, 224, 524, 1001, 223, 1, 223, 1108, 677, 226,
            224, 1002, 223, 2, 223, 1005, 224, 539, 1001, 223, 1, 223, 1108, 226, 677, 224, 102, 2,
            223, 223, 1006, 224, 554, 101, 1, 223, 223, 108, 226, 677, 224, 102, 2, 223, 223, 1005,
            224, 569, 1001, 223, 1, 223, 8, 677, 677, 224, 1002, 223, 2, 223, 1005, 224, 584, 101,
            1, 223, 223, 108, 677, 677, 224, 1002, 223, 2, 223, 1005, 224, 599, 1001, 223, 1, 223,
            108, 226, 226, 224, 102, 2, 223, 223, 1006, 224, 614, 101, 1, 223, 223, 1008, 677, 677,
            224, 102, 2, 223, 223, 1006, 224, 629, 1001, 223, 1, 223, 107, 226, 677, 224, 102, 2,
            223, 223, 1006, 224, 644, 101, 1, 223, 223, 1107, 677, 677, 224, 1002, 223, 2, 223,
            1005, 224, 659, 1001, 223, 1, 223, 7, 226, 226, 224, 1002, 223, 2, 223, 1005, 224, 674,
            101, 1, 223, 223, 4, 223, 99, 226,
        ];

        let mut vm = Vm::with_memory(intcode);

        let why = vm.run();
        assert_eq!(
            why,
            Err(VmStopReason::BlockedOnInput { ip: 0 }),
            "vm didn't stop to wait on input"
        );

        vm.add_input(1);

        let why = vm.run();
        assert_eq!(
            why,
            Ok(222),
            "vm finished executing at an unexpected address"
        );

        assert_eq!(
            vm.get_output(),
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 13346482],
            "vm output didn't match expected output"
        );
    }

    #[test]
    fn check_part2_input() {
        let intcode = vec![
            3, 225, 1, 225, 6, 6, 1100, 1, 238, 225, 104, 0, 101, 20, 183, 224, 101, -63, 224, 224,
            4, 224, 1002, 223, 8, 223, 101, 6, 224, 224, 1, 223, 224, 223, 1101, 48, 40, 225, 1101,
            15, 74, 225, 2, 191, 40, 224, 1001, 224, -5624, 224, 4, 224, 1002, 223, 8, 223, 1001,
            224, 2, 224, 1, 223, 224, 223, 1101, 62, 60, 225, 1102, 92, 15, 225, 102, 59, 70, 224,
            101, -885, 224, 224, 4, 224, 1002, 223, 8, 223, 101, 7, 224, 224, 1, 224, 223, 223, 1,
            35, 188, 224, 1001, 224, -84, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 2, 224, 1, 223,
            224, 223, 1001, 66, 5, 224, 1001, 224, -65, 224, 4, 224, 102, 8, 223, 223, 1001, 224,
            3, 224, 1, 223, 224, 223, 1002, 218, 74, 224, 101, -2960, 224, 224, 4, 224, 1002, 223,
            8, 223, 1001, 224, 2, 224, 1, 224, 223, 223, 1101, 49, 55, 224, 1001, 224, -104, 224,
            4, 224, 102, 8, 223, 223, 1001, 224, 6, 224, 1, 224, 223, 223, 1102, 43, 46, 225, 1102,
            7, 36, 225, 1102, 76, 30, 225, 1102, 24, 75, 224, 101, -1800, 224, 224, 4, 224, 102, 8,
            223, 223, 101, 2, 224, 224, 1, 224, 223, 223, 1101, 43, 40, 225, 4, 223, 99, 0, 0, 0,
            677, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1105, 0, 99999, 1105, 227, 247, 1105, 1, 99999,
            1005, 227, 99999, 1005, 0, 256, 1105, 1, 99999, 1106, 227, 99999, 1106, 0, 265, 1105,
            1, 99999, 1006, 0, 99999, 1006, 227, 274, 1105, 1, 99999, 1105, 1, 280, 1105, 1, 99999,
            1, 225, 225, 225, 1101, 294, 0, 0, 105, 1, 0, 1105, 1, 99999, 1106, 0, 300, 1105, 1,
            99999, 1, 225, 225, 225, 1101, 314, 0, 0, 106, 0, 0, 1105, 1, 99999, 1008, 226, 226,
            224, 1002, 223, 2, 223, 1005, 224, 329, 1001, 223, 1, 223, 8, 226, 677, 224, 102, 2,
            223, 223, 1006, 224, 344, 1001, 223, 1, 223, 1007, 226, 677, 224, 1002, 223, 2, 223,
            1005, 224, 359, 101, 1, 223, 223, 1008, 677, 226, 224, 102, 2, 223, 223, 1006, 224,
            374, 1001, 223, 1, 223, 1107, 226, 677, 224, 1002, 223, 2, 223, 1006, 224, 389, 1001,
            223, 1, 223, 107, 677, 677, 224, 1002, 223, 2, 223, 1006, 224, 404, 101, 1, 223, 223,
            1007, 226, 226, 224, 1002, 223, 2, 223, 1006, 224, 419, 101, 1, 223, 223, 7, 677, 226,
            224, 1002, 223, 2, 223, 1005, 224, 434, 1001, 223, 1, 223, 1007, 677, 677, 224, 1002,
            223, 2, 223, 1006, 224, 449, 101, 1, 223, 223, 107, 226, 226, 224, 1002, 223, 2, 223,
            1006, 224, 464, 1001, 223, 1, 223, 1108, 677, 677, 224, 1002, 223, 2, 223, 1005, 224,
            479, 101, 1, 223, 223, 8, 677, 226, 224, 1002, 223, 2, 223, 1006, 224, 494, 101, 1,
            223, 223, 7, 226, 677, 224, 102, 2, 223, 223, 1005, 224, 509, 1001, 223, 1, 223, 1107,
            677, 226, 224, 102, 2, 223, 223, 1005, 224, 524, 1001, 223, 1, 223, 1108, 677, 226,
            224, 1002, 223, 2, 223, 1005, 224, 539, 1001, 223, 1, 223, 1108, 226, 677, 224, 102, 2,
            223, 223, 1006, 224, 554, 101, 1, 223, 223, 108, 226, 677, 224, 102, 2, 223, 223, 1005,
            224, 569, 1001, 223, 1, 223, 8, 677, 677, 224, 1002, 223, 2, 223, 1005, 224, 584, 101,
            1, 223, 223, 108, 677, 677, 224, 1002, 223, 2, 223, 1005, 224, 599, 1001, 223, 1, 223,
            108, 226, 226, 224, 102, 2, 223, 223, 1006, 224, 614, 101, 1, 223, 223, 1008, 677, 677,
            224, 102, 2, 223, 223, 1006, 224, 629, 1001, 223, 1, 223, 107, 226, 677, 224, 102, 2,
            223, 223, 1006, 224, 644, 101, 1, 223, 223, 1107, 677, 677, 224, 1002, 223, 2, 223,
            1005, 224, 659, 1001, 223, 1, 223, 7, 226, 226, 224, 1002, 223, 2, 223, 1005, 224, 674,
            101, 1, 223, 223, 4, 223, 99, 226,
        ];

        let mut vm = Vm::with_memory(intcode);

        let why = vm.run();
        assert_eq!(
            why,
            Err(VmStopReason::BlockedOnInput { ip: 0 }),
            "vm didn't stop to wait on input"
        );

        vm.add_input(5);

        let why = vm.run();
        assert_eq!(
            why,
            Ok(222),
            "vm finished executing at an unexpected address"
        );

        assert_eq!(
            vm.get_output(),
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 12111395],
            "vm output didn't match expected output"
        );
    }
}
