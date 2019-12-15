//! Intcode Virtual Machine
//!
//! This module implements a virtual machine to execute and manage intcode
//! programs

use err_derive;
use num::Integer;
use smallvec::SmallVec;

use std::convert::TryFrom;
use std::fmt;
use std::fmt::Write;

/// Individual entries in the VM's memory are represented as "atom" types
/// This can be a signed or unsigned integer of unspecified size.
/// We pick i64 to make sure have enough values for anything we want.
pub type Atom = i64;

pub struct OpcodeInfo {
    name: &'static str,
    n_args: i64,
}

// TODO: enum
pub mod op {
    use super::{Atom, OpcodeInfo};

    pub const ADD: Atom = 1;
    pub const MUL: Atom = 2;
    pub const IN: Atom = 3;
    pub const OUT: Atom = 4;
    pub const JN: Atom = 5;
    pub const JZ: Atom = 6;
    pub const LT: Atom = 7;
    pub const EQ: Atom = 8;
    pub const ARB: Atom = 9;
    pub const HLT: Atom = 99;

    pub fn metadata(opcode: Atom) -> OpcodeInfo {
        match opcode {
            ADD => OpcodeInfo {
                name: "Add",
                n_args: 3,
            },
            MUL => OpcodeInfo {
                name: "Mul",
                n_args: 3,
            },
            IN => OpcodeInfo {
                name: "In",
                n_args: 1,
            },
            OUT => OpcodeInfo {
                name: "Out",
                n_args: 1,
            },
            JN => OpcodeInfo {
                name: "Jn",
                n_args: 2,
            },
            JZ => OpcodeInfo {
                name: "Jz",
                n_args: 2,
            },
            LT => OpcodeInfo {
                name: "Lt",
                n_args: 3,
            },
            EQ => OpcodeInfo {
                name: "Eq",
                n_args: 3,
            },
            ARB => OpcodeInfo {
                name: "Arb",
                n_args: 1,
            },
            HLT => OpcodeInfo {
                name: "Hlt",
                n_args: 0,
            },
            _ => OpcodeInfo {
                name: "???",
                n_args: 3,
            },
        }
    }
}

/// The Vm has halted (and can be continued)
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum NameMe {
    // Execution was paused, but can be resumed immediately
    Ready,

    // Execution has been stopped by a HALT instruction
    Halted,

    // Execution has been stopped by an IN instruction, and needs input before continuing
    Input,

    // Execution has been paused by an OUT instruction and can be resumed immediately
    // The output value is included
    Output(Atom),
}

/// The Vm has halted and cannot be continued
#[derive(Copy, Clone, Debug, err_derive::Error, PartialEq)]
pub enum VmError {
    #[error(
        display = "Invalid opcode or ParamMode in {:03}-{}",
        param_modes,
        opcode
    )]
    BadOpcode { opcode: Atom, param_modes: Atom },

    #[error(display = "Attempted to read invalid memory, address={}", addr)]
    BadMemoryRead { addr: Atom },

    #[error(
        display = "Attempted to write {} invalid memory, address={}",
        atom,
        addr
    )]
    BadMemoryWrite { addr: Atom, atom: Atom },
}

type VmResult<T> = Result<T, VmError>;

/// The state for a paused, running, or halted Intcode Vm
#[derive(Clone, Default)]
pub struct Vm {
    /// Instruction Pointer
    ///
    /// Points to the Atom offset in memory that the VM is about to execute
    ip: Atom,

    /// Relative Base
    ///
    /// Referenced by parameter mode 2 ("Relative") and opcode 9 ("Arb")
    rb: Atom,

    /// Tick count
    ///
    /// This tick is increased every time an instruction is executed
    ticks: Atom,

    /// Main Memory for the VM
    ///
    /// Instructions and data co-exist in this space, and instructions can
    /// modify any of this memory at any time.
    mem: Vec<Atom>,

    /// Pending input values for an `IN` instruction
    input_buffer: SmallVec<[Atom; 16]>,
}

impl Vm {
    /// Construct a new Vm with initial memory copied from a slice
    ///
    /// The vm will begin executing int code at index 0
    pub fn from_code(intcode: &[Atom]) -> Vm {
        let mem: Vec<Atom> = intcode.iter().copied().collect();
        Vm::from_buffer(mem)
    }

    /// Construct a new Vm with initial memory by taking ownership of a buffer
    ///
    /// The vm will begin executing int code at index 0
    pub fn from_buffer(mem: Vec<Atom>) -> Vm {
        Vm {
            ip: 0,
            rb: 0,
            ticks: 0,
            mem,
            input_buffer: SmallVec::new(),
        }
    }

    /// Construct a new Vm with no initial memory
    ///
    /// Running this without calling `Vm::reset()` will error.
    /// Use this if you expect to call reset() before using the vm.
    pub fn empty() -> Vm {
        Vm::default()
    }

    /// Reset the Vm to a pre-defined state
    ///
    /// Prefer this over creating and dropping instances in a loop
    pub fn reset(&mut self, new_mem: &[Atom]) {
        // Re-initialize internal states
        self.ip = 0;
        self.ticks = 0;
        self.mem.clear();
        self.input_buffer.clear();

        if self.mem.len() < new_mem.len() {
            // We need to resize `self.mem` so that it exactly matches the size of `new_mem`,
            // but `Vec::resize()` wastes cycles by inserting some value.
            // We immediately overwrite that value, and benchmarks show the compiler doesn't catch that.
            // Therefore, we reserve any additional space we need and and force the length to match
            // This is generally `unsafe`, but we know that it's safe in this instance.
            unsafe {
                let additional: usize = usize::saturating_sub(new_mem.len(), self.mem.len());
                self.mem.reserve(additional);
                self.mem.set_len(new_mem.len());
            }
        }

        self.mem.copy_from_slice(new_mem);
    }

    /// Retrieve the current instruction pointer
    pub fn ip(&self) -> Atom {
        self.ip
    }

    /// Input a value to the machine
    ///
    /// This will buffer the value until an `IN` instruction is executed, that will then use this
    /// value as its input.
    /// Multiple input values can be inserted. They will be processed in the order that this
    /// method is called
    pub fn input(&mut self, atom: Atom) {
        self.input_buffer.push(atom);
    }

    pub fn mem(&self) -> &[Atom] {
        &self.mem
    }

    /// Helper method to read an atom from an intcode address
    fn read_atom(&mut self, addr: Atom) -> VmResult<Atom> {
        if let Ok(addr) = usize::try_from(addr) {
            if addr >= self.mem.len() {
                self.mem.resize_with(addr + 1, Atom::default);
            }
            let atom = self.mem[addr];

            #[cfg(feature = "vm-logging")]
            println!("[{:4}]     read [{}] -> {}", self.ip, addr, atom);

            Ok(atom)
        } else {
            #[cfg(feature = "vm-logging")]
            println!("[{:4}]     read [{}] FAILED", self.ip, addr);

            Err(VmError::BadMemoryRead { addr })
        }
    }

    /// Helper method to write an atom to a vm address
    fn write_atom(&mut self, addr: Atom, atom: Atom) -> VmResult<()> {
        #[cfg(feature = "vm-logging")]
        println!("[{:4}]     write {} to [{}]", self.ip, atom, addr);

        if let Ok(addr) = usize::try_from(addr) {
            if addr >= self.mem.len() {
                self.mem.resize_with(addr + 1, Atom::default);
            }

            self.mem[addr] = atom;
            Ok(())
        } else {
            Err(VmError::BadMemoryWrite { addr, atom })
        }
    }

    fn log_instr(&self, param_modes: i64, opcode: i64, args: &[Atom; 3]) {
        if cfg!(feature = "vm-logging") {
            let info = op::metadata(opcode);
            let args = &args[..info.n_args as usize];

            println!(
                "[{:4}] {:03}-{:02} {} {:?}",
                self.ip, param_modes, opcode, info.name, args
            );
        }
    }

    /// Run the Vm until it stops
    ///
    /// Returns Ok(self.ip()) if the vm executes `HALT`, otherwise Err() describes what happened.
    pub fn run(&mut self) -> VmResult<NameMe> {
        loop {
            match self.tick()? {
                NameMe::Ready => continue,
                reason => return Ok(reason),
            }
        }
    }

    /// Run the Vm until it stops, collecting output instead of halting
    ///
    /// Returns Ok(self.ip()) if the vm executes `HALT`, otherwise Err() describes what happened.
    pub fn run_with_output(&mut self, output: &mut Vec<Atom>) -> VmResult<NameMe> {
        loop {
            match self.run()? {
                NameMe::Output(out) => output.push(out),
                reason => return Ok(reason),
            }
        }
    }

    pub fn tick(&mut self) -> VmResult<NameMe> {
        self.ticks += 1;

        // Read the opcode atom
        // It comes in two parts:
        //      - 2 digits for the opcode
        //      - 3 single digits for 3 parameter modes
        //          "missing" modes default to 0 (absolute address)
        let packed_opcode = self.read_atom(self.ip)?;
        let (param_modes, opcode) = packed_opcode.div_rem(&100);

        // Pre-fetch three atoms
        // This is safe because out-of-bounds values will extend the memory buffer and return 0
        const INTERNAL_FETCH_ERROR: &str = "internal vm error - failed to prefetch instruction ops";
        let prefetch: [Atom; 3] = [
            self.read_atom(self.ip + 1).expect(INTERNAL_FETCH_ERROR),
            self.read_atom(self.ip + 2).expect(INTERNAL_FETCH_ERROR),
            self.read_atom(self.ip + 3).expect(INTERNAL_FETCH_ERROR),
        ];

        self.log_instr(param_modes, opcode, &prefetch);

        // param_modes encodes three parameter modes:
        //    mode 0: the parameter is an absolute address to read the value from
        //    mode 1: the parameter is an immediate value that should be used as-is
        //    mode 2: the parameter is a relative address (+ self.rb) to read the value from
        // It is illegal to have parameter mode 1 on writeout parameters

        // 0-prefixed literals are DECIMAL, not octal
        // We use 0-prefixed literals to better show off their leading 0 digits, so disable this
        #[allow(clippy::zero_prefixed_literal)]
        match opcode {
            op::ADD => {
                match param_modes {
                    // === Addr writeout
                    000 => {
                        let a = self.read_atom(prefetch[0])?;
                        let b = self.read_atom(prefetch[1])?;
                        let c = a + b;
                        self.write_atom(prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }
                    001 => {
                        let a = prefetch[0];
                        let b = self.read_atom(prefetch[1])?;
                        let c = a + b;
                        self.write_atom(prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }
                    002 => {
                        let a = prefetch[0] + self.rb;
                        let b = self.read_atom(prefetch[1])?;
                        let c = a + b;
                        self.write_atom(prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }

                    010 => {
                        let a = self.read_atom(prefetch[0])?;
                        let b = prefetch[1];
                        let c = a + b;
                        self.write_atom(prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }
                    011 => {
                        let a = prefetch[0];
                        let b = prefetch[1];
                        let c = a + b;
                        self.write_atom(prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }
                    012 => {
                        let a = prefetch[0] + self.rb;
                        let b = prefetch[1];
                        let c = a + b;
                        self.write_atom(prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }

                    020 => {
                        let a = self.read_atom(prefetch[0])?;
                        let b = self.read_atom(self.rb + prefetch[1])?;
                        let c = a + b;
                        self.write_atom(prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }
                    021 => {
                        let a = prefetch[0];
                        let b = self.read_atom(self.rb + prefetch[1])?;
                        let c = a + b;
                        self.write_atom(prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }
                    022 => {
                        let a = prefetch[0] + self.rb;
                        let b = self.read_atom(self.rb + prefetch[1])?;
                        let c = a + b;
                        self.write_atom(prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }

                    // === Relative writeout
                    200 => {
                        let a = self.read_atom(prefetch[0])?;
                        let b = self.read_atom(prefetch[1])?;
                        let c = a + b;
                        self.write_atom(self.rb + prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }
                    201 => {
                        let a = prefetch[0];
                        let b = self.read_atom(prefetch[1])?;
                        let c = a + b;
                        self.write_atom(self.rb + prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }
                    202 => {
                        let a = prefetch[0] + self.rb;
                        let b = self.read_atom(prefetch[1])?;
                        let c = a + b;
                        self.write_atom(self.rb + prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }

                    210 => {
                        let a = self.read_atom(prefetch[0])?;
                        let b = prefetch[1];
                        let c = a + b;
                        self.write_atom(self.rb + prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }
                    211 => {
                        let a = prefetch[0];
                        let b = prefetch[1];
                        let c = a + b;
                        self.write_atom(self.rb + prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }
                    212 => {
                        let a = prefetch[0] + self.rb;
                        let b = prefetch[1];
                        let c = a + b;
                        self.write_atom(self.rb + prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }

                    220 => {
                        let a = self.read_atom(prefetch[0])?;
                        let b = self.read_atom(self.rb + prefetch[1])?;
                        let c = a + b;
                        self.write_atom(self.rb + prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }
                    221 => {
                        let a = prefetch[0];
                        let b = self.read_atom(self.rb + prefetch[1])?;
                        let c = a + b;
                        self.write_atom(self.rb + prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }
                    222 => {
                        let a = prefetch[0] + self.rb;
                        let b = self.read_atom(self.rb + prefetch[1])?;
                        let c = a + b;
                        self.write_atom(self.rb + prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }

                    _ => Err(VmError::BadOpcode {
                        opcode,
                        param_modes,
                    }),
                }
            }

            op::MUL => {
                match param_modes {
                    // === Addr writeout
                    000 => {
                        let a = self.read_atom(prefetch[0])?;
                        let b = self.read_atom(prefetch[1])?;
                        let c = a * b;
                        self.write_atom(prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }
                    001 => {
                        let a = prefetch[0];
                        let b = self.read_atom(prefetch[1])?;
                        let c = a * b;
                        self.write_atom(prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }
                    002 => {
                        let a = prefetch[0] + self.rb;
                        let b = self.read_atom(prefetch[1])?;
                        let c = a * b;
                        self.write_atom(prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }

                    010 => {
                        let a = self.read_atom(prefetch[0])?;
                        let b = prefetch[1];
                        let c = a * b;
                        self.write_atom(prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }
                    011 => {
                        let a = prefetch[0];
                        let b = prefetch[1];
                        let c = a * b;
                        self.write_atom(prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }
                    012 => {
                        let a = prefetch[0] + self.rb;
                        let b = prefetch[1];
                        let c = a * b;
                        self.write_atom(prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }

                    020 => {
                        let a = self.read_atom(prefetch[0])?;
                        let b = self.read_atom(self.rb + prefetch[1])?;
                        let c = a * b;
                        self.write_atom(prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }
                    021 => {
                        let a = prefetch[0];
                        let b = self.read_atom(self.rb + prefetch[1])?;
                        let c = a * b;
                        self.write_atom(prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }
                    022 => {
                        let a = prefetch[0] + self.rb;
                        let b = self.read_atom(self.rb + prefetch[1])?;
                        let c = a * b;
                        self.write_atom(prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }

                    // === Relative writeout
                    200 => {
                        let a = self.read_atom(prefetch[0])?;
                        let b = self.read_atom(prefetch[1])?;
                        let c = a * b;
                        self.write_atom(self.rb + prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }
                    201 => {
                        let a = prefetch[0];
                        let b = self.read_atom(prefetch[1])?;
                        let c = a * b;
                        self.write_atom(self.rb + prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }
                    202 => {
                        let a = prefetch[0] + self.rb;
                        let b = self.read_atom(prefetch[1])?;
                        let c = a * b;
                        self.write_atom(self.rb + prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }

                    210 => {
                        let a = self.read_atom(prefetch[0])?;
                        let b = prefetch[1];
                        let c = a * b;
                        self.write_atom(self.rb + prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }
                    211 => {
                        let a = prefetch[0];
                        let b = prefetch[1];
                        let c = a * b;
                        self.write_atom(self.rb + prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }
                    212 => {
                        let a = prefetch[0] + self.rb;
                        let b = prefetch[1];
                        let c = a * b;
                        self.write_atom(self.rb + prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }

                    220 => {
                        let a = self.read_atom(prefetch[0])?;
                        let b = self.read_atom(self.rb + prefetch[1])?;
                        let c = a * b;
                        self.write_atom(self.rb + prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }
                    221 => {
                        let a = prefetch[0];
                        let b = self.read_atom(self.rb + prefetch[1])?;
                        let c = a * b;
                        self.write_atom(self.rb + prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }
                    222 => {
                        let a = prefetch[0] + self.rb;
                        let b = self.read_atom(self.rb + prefetch[1])?;
                        let c = a * b;
                        self.write_atom(self.rb + prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }

                    _ => Err(VmError::BadOpcode {
                        opcode,
                        param_modes,
                    }),
                }
            }

            op::IN => {
                // If we have no input, we cannot continue
                if self.input_buffer.is_empty() {
                    #[cfg(feature = "vm-logging")]
                    println!("[{:4}]     no input", self.ip);
                    return Ok(NameMe::Input);
                }

                let atom = self.input_buffer[0];
                self.input_buffer.remove(0);

                #[cfg(feature = "vm-logging")]
                println!("[{:4}]     input: {}", self.ip, atom);

                match param_modes {
                    000 => {
                        self.write_atom(prefetch[0], atom)?;
                        self.ip += 1 + op::metadata(op::IN).n_args;
                        Ok(NameMe::Ready)
                    }
                    002 => {
                        self.write_atom(self.rb + prefetch[0], atom)?;
                        self.ip += 1 + op::metadata(op::IN).n_args;
                        Ok(NameMe::Ready)
                    }
                    _ => Err(VmError::BadOpcode {
                        opcode,
                        param_modes,
                    }),
                }
            }

            op::OUT => match param_modes {
                000 => {
                    let atom = self.read_atom(prefetch[0])?;

                    #[cfg(feature = "vm-logging")]
                    println!("[{:4}]     output: {}", self.ip, atom);

                    self.ip += 1 + op::metadata(op::OUT).n_args;
                    Ok(NameMe::Output(atom))
                }

                001 => {
                    let atom = prefetch[0];

                    #[cfg(feature = "vm-logging")]
                    println!("[{:4}]     output: {}", self.ip, atom);

                    self.ip += 1 + op::metadata(op::OUT).n_args;
                    Ok(NameMe::Output(atom))
                }

                002 => {
                    let atom = self.read_atom(self.rb + prefetch[0])?;

                    #[cfg(feature = "vm-logging")]
                    println!("[{:4}]     output: {}", self.ip, atom);

                    self.ip += 1 + op::metadata(op::OUT).n_args;
                    Ok(NameMe::Output(atom))
                }

                _ => Err(VmError::BadOpcode {
                    opcode,
                    param_modes,
                }),
            },

            op::JN => match param_modes {
                000 => {
                    let pred = self.read_atom(prefetch[0])?;
                    let atom = self.read_atom(prefetch[1])?;

                    if pred != 0 {
                        self.ip = atom;
                    } else {
                        self.ip += 1 + op::metadata(op::JN).n_args;
                    }

                    Ok(NameMe::Ready)
                }
                001 => {
                    let pred = prefetch[0];
                    let atom = self.read_atom(prefetch[1])?;

                    if pred != 0 {
                        self.ip = atom;
                    } else {
                        self.ip += 1 + op::metadata(op::JN).n_args;
                    }

                    Ok(NameMe::Ready)
                }
                002 => {
                    let pred = self.read_atom(self.rb + prefetch[0])?;
                    let atom = self.read_atom(prefetch[1])?;

                    if pred != 0 {
                        self.ip = atom;
                    } else {
                        self.ip += 1 + op::metadata(op::JN).n_args;
                    }

                    Ok(NameMe::Ready)
                }

                010 => {
                    let pred = self.read_atom(prefetch[0])?;
                    let atom = prefetch[1];

                    if pred != 0 {
                        self.ip = atom;
                    } else {
                        self.ip += 1 + op::metadata(op::JN).n_args;
                    }

                    Ok(NameMe::Ready)
                }
                011 => {
                    let pred = prefetch[0];
                    let atom = prefetch[1];

                    if pred != 0 {
                        self.ip = atom;
                    } else {
                        self.ip += 1 + op::metadata(op::JN).n_args;
                    }

                    Ok(NameMe::Ready)
                }
                012 => {
                    let pred = self.read_atom(self.rb + prefetch[0])?;
                    let atom = prefetch[1];

                    if pred != 0 {
                        self.ip = atom;
                    } else {
                        self.ip += 1 + op::metadata(op::JN).n_args;
                    }

                    Ok(NameMe::Ready)
                }

                _ => Err(VmError::BadOpcode {
                    opcode,
                    param_modes,
                }),
            },

            op::JZ => match param_modes {
                00 => {
                    let pred = self.read_atom(prefetch[0])?;
                    let atom = self.read_atom(prefetch[1])?;

                    if pred == 0 {
                        self.ip = atom;
                    } else {
                        self.ip += 1 + op::metadata(op::JN).n_args;
                    }

                    Ok(NameMe::Ready)
                }
                01 => {
                    let pred = prefetch[0];
                    let atom = self.read_atom(prefetch[1])?;

                    if pred == 0 {
                        self.ip = atom;
                    } else {
                        self.ip += 1 + op::metadata(op::JN).n_args;
                    }

                    Ok(NameMe::Ready)
                }
                02 => {
                    let pred = self.read_atom(self.rb + prefetch[0])?;
                    let atom = self.read_atom(prefetch[1])?;

                    if pred == 0 {
                        self.ip = atom;
                    } else {
                        self.ip += 1 + op::metadata(op::JN).n_args;
                    }

                    Ok(NameMe::Ready)
                }

                10 => {
                    let pred = self.read_atom(prefetch[0])?;
                    let atom = prefetch[1];

                    if pred == 0 {
                        self.ip = atom;
                    } else {
                        self.ip += 1 + op::metadata(op::JN).n_args;
                    }

                    Ok(NameMe::Ready)
                }
                11 => {
                    let pred = prefetch[0];
                    let atom = prefetch[1];

                    if pred == 0 {
                        self.ip = atom;
                    } else {
                        self.ip += 1 + op::metadata(op::JN).n_args;
                    }

                    Ok(NameMe::Ready)
                }
                12 => {
                    let pred = self.read_atom(self.rb + prefetch[0])?;
                    let atom = prefetch[1];

                    if pred == 0 {
                        self.ip = atom;
                    } else {
                        self.ip += 1 + op::metadata(op::JN).n_args;
                    }

                    Ok(NameMe::Ready)
                }

                20 => {
                    let pred = self.read_atom(prefetch[0])?;
                    let atom = self.read_atom(self.rb + prefetch[1])?;

                    if pred == 0 {
                        self.ip = atom;
                    } else {
                        self.ip += 1 + op::metadata(op::JN).n_args;
                    }

                    Ok(NameMe::Ready)
                }
                21 => {
                    let pred = prefetch[0];
                    let atom = self.read_atom(self.rb + prefetch[1])?;

                    if pred == 0 {
                        self.ip = atom;
                    } else {
                        self.ip += 1 + op::metadata(op::JN).n_args;
                    }

                    Ok(NameMe::Ready)
                }
                22 => {
                    let pred = self.read_atom(self.rb + prefetch[0])?;
                    let atom = self.read_atom(self.rb + prefetch[1])?;

                    if pred == 0 {
                        self.ip = atom;
                    } else {
                        self.ip += 1 + op::metadata(op::JN).n_args;
                    }

                    Ok(NameMe::Ready)
                }

                _ => Err(VmError::BadOpcode {
                    opcode,
                    param_modes,
                }),
            },

            op::LT => {
                match param_modes {
                    // === Addr writeout
                    000 => {
                        let a = self.read_atom(prefetch[0])?;
                        let b = self.read_atom(prefetch[1])?;
                        let c = (a < b) as Atom;
                        self.write_atom(prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }
                    001 => {
                        let a = prefetch[0];
                        let b = self.read_atom(prefetch[1])?;
                        let c = (a < b) as Atom;
                        self.write_atom(prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }
                    002 => {
                        let a = prefetch[0] + self.rb;
                        let b = self.read_atom(prefetch[1])?;
                        let c = (a < b) as Atom;
                        self.write_atom(prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }

                    010 => {
                        let a = self.read_atom(prefetch[0])?;
                        let b = prefetch[1];
                        let c = (a < b) as Atom;
                        self.write_atom(prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }
                    011 => {
                        let a = prefetch[0];
                        let b = prefetch[1];
                        let c = (a < b) as Atom;
                        self.write_atom(prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }
                    012 => {
                        let a = prefetch[0] + self.rb;
                        let b = prefetch[1];
                        let c = (a < b) as Atom;
                        self.write_atom(prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }

                    020 => {
                        let a = self.read_atom(prefetch[0])?;
                        let b = self.read_atom(self.rb + prefetch[1])?;
                        let c = (a < b) as Atom;
                        self.write_atom(prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }
                    021 => {
                        let a = prefetch[0];
                        let b = self.read_atom(self.rb + prefetch[1])?;
                        let c = (a < b) as Atom;
                        self.write_atom(prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }
                    022 => {
                        let a = prefetch[0] + self.rb;
                        let b = self.read_atom(self.rb + prefetch[1])?;
                        let c = (a < b) as Atom;
                        self.write_atom(prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }

                    // === Relative writeout
                    200 => {
                        let a = self.read_atom(prefetch[0])?;
                        let b = self.read_atom(prefetch[1])?;
                        let c = (a < b) as Atom;
                        self.write_atom(self.rb + prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }
                    201 => {
                        let a = prefetch[0];
                        let b = self.read_atom(prefetch[1])?;
                        let c = (a < b) as Atom;
                        self.write_atom(self.rb + prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }
                    202 => {
                        let a = prefetch[0] + self.rb;
                        let b = self.read_atom(prefetch[1])?;
                        let c = (a < b) as Atom;
                        self.write_atom(self.rb + prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }

                    210 => {
                        let a = self.read_atom(prefetch[0])?;
                        let b = prefetch[1];
                        let c = (a < b) as Atom;
                        self.write_atom(self.rb + prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }
                    211 => {
                        let a = prefetch[0];
                        let b = prefetch[1];
                        let c = (a < b) as Atom;
                        self.write_atom(self.rb + prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }
                    212 => {
                        let a = prefetch[0] + self.rb;
                        let b = prefetch[1];
                        let c = (a < b) as Atom;
                        self.write_atom(self.rb + prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }

                    220 => {
                        let a = self.read_atom(prefetch[0])?;
                        let b = self.read_atom(self.rb + prefetch[1])?;
                        let c = (a < b) as Atom;
                        self.write_atom(self.rb + prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }
                    221 => {
                        let a = prefetch[0];
                        let b = self.read_atom(self.rb + prefetch[1])?;
                        let c = (a < b) as Atom;
                        self.write_atom(self.rb + prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }
                    222 => {
                        let a = prefetch[0] + self.rb;
                        let b = self.read_atom(self.rb + prefetch[1])?;
                        let c = (a < b) as Atom;
                        self.write_atom(self.rb + prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }

                    _ => Err(VmError::BadOpcode {
                        opcode,
                        param_modes,
                    }),
                }
            }

            op::EQ => {
                match param_modes {
                    // === Addr writeout
                    000 => {
                        let a = self.read_atom(prefetch[0])?;
                        let b = self.read_atom(prefetch[1])?;
                        let c = (a == b) as Atom;
                        self.write_atom(prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }
                    001 => {
                        let a = prefetch[0];
                        let b = self.read_atom(prefetch[1])?;
                        let c = (a == b) as Atom;
                        self.write_atom(prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }
                    002 => {
                        let a = prefetch[0] + self.rb;
                        let b = self.read_atom(prefetch[1])?;
                        let c = (a == b) as Atom;
                        self.write_atom(prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }

                    010 => {
                        let a = self.read_atom(prefetch[0])?;
                        let b = prefetch[1];
                        let c = (a == b) as Atom;
                        self.write_atom(prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }
                    011 => {
                        let a = prefetch[0];
                        let b = prefetch[1];
                        let c = (a == b) as Atom;
                        self.write_atom(prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }
                    012 => {
                        let a = prefetch[0] + self.rb;
                        let b = prefetch[1];
                        let c = (a == b) as Atom;
                        self.write_atom(prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }

                    020 => {
                        let a = self.read_atom(prefetch[0])?;
                        let b = self.read_atom(self.rb + prefetch[1])?;
                        let c = (a == b) as Atom;
                        self.write_atom(prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }
                    021 => {
                        let a = prefetch[0];
                        let b = self.read_atom(self.rb + prefetch[1])?;
                        let c = (a == b) as Atom;
                        self.write_atom(prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }
                    022 => {
                        let a = prefetch[0] + self.rb;
                        let b = self.read_atom(self.rb + prefetch[1])?;
                        let c = (a == b) as Atom;
                        self.write_atom(prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }

                    // === Relative writeout
                    200 => {
                        let a = self.read_atom(prefetch[0])?;
                        let b = self.read_atom(prefetch[1])?;
                        let c = (a == b) as Atom;
                        self.write_atom(self.rb + prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }
                    201 => {
                        let a = prefetch[0];
                        let b = self.read_atom(prefetch[1])?;
                        let c = (a == b) as Atom;
                        self.write_atom(self.rb + prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }
                    202 => {
                        let a = prefetch[0] + self.rb;
                        let b = self.read_atom(prefetch[1])?;
                        let c = (a == b) as Atom;
                        self.write_atom(self.rb + prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }

                    210 => {
                        let a = self.read_atom(prefetch[0])?;
                        let b = prefetch[1];
                        let c = (a == b) as Atom;
                        self.write_atom(self.rb + prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }
                    211 => {
                        let a = prefetch[0];
                        let b = prefetch[1];
                        let c = (a == b) as Atom;
                        self.write_atom(self.rb + prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }
                    212 => {
                        let a = prefetch[0] + self.rb;
                        let b = prefetch[1];
                        let c = (a == b) as Atom;
                        self.write_atom(self.rb + prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }

                    220 => {
                        let a = self.read_atom(prefetch[0])?;
                        let b = self.read_atom(self.rb + prefetch[1])?;
                        let c = (a == b) as Atom;
                        self.write_atom(self.rb + prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }
                    221 => {
                        let a = prefetch[0];
                        let b = self.read_atom(self.rb + prefetch[1])?;
                        let c = (a == b) as Atom;
                        self.write_atom(self.rb + prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }
                    222 => {
                        let a = prefetch[0] + self.rb;
                        let b = self.read_atom(self.rb + prefetch[1])?;
                        let c = (a == b) as Atom;
                        self.write_atom(self.rb + prefetch[2], c)?;
                        self.ip += 4;
                        Ok(NameMe::Ready)
                    }

                    _ => Err(VmError::BadOpcode {
                        opcode,
                        param_modes,
                    }),
                }
            }

            op::ARB => match param_modes {
                0 => {
                    let atom = self.read_atom(prefetch[0])?;

                    #[cfg(feature = "vm-logging")]
                    println!("[{:4}     arb: {} -> {}", self.ip, self.rb, self.rb + atom);

                    self.rb += atom;
                    self.ip += 2;
                    Ok(NameMe::Ready)
                }
                1 => {
                    let atom = prefetch[0];

                    #[cfg(feature = "vm-logging")]
                    println!("[{:4}     arb: {} -> {}", self.ip, self.rb, self.rb + atom);

                    self.rb += atom;
                    self.ip += 2;
                    Ok(NameMe::Ready)
                }
                2 => {
                    let atom = self.read_atom(self.rb + prefetch[0])?;

                    #[cfg(feature = "vm-logging")]
                    println!("[{:4}     arb: {} -> {}", self.ip, self.rb, self.rb + atom);

                    self.rb += atom;
                    self.ip += 2;
                    Ok(NameMe::Ready)
                }
                _ => Err(VmError::BadOpcode {
                    opcode,
                    param_modes,
                }),
            },

            // HLT does not advance the instruction pointer
            //   --> Repeated ticks at HLT stay at HLT
            op::HLT => Ok(NameMe::Halted),

            _ => Err(VmError::BadOpcode {
                opcode,
                param_modes,
            }),
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
            .field("rb", &self.rb)
            .field("ticks", &self.ticks)
            .field("input_buffer", &self.input_buffer)
            .field("mem", &pretty_fmt_memory(&self.mem)?)
            .finish()
    }
}

#[cfg(test)]
mod day_02 {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn check_example_1() {
        let intcode = vec![1, 0, 0, 0, 99];
        let mut vm = Vm::from_code(&intcode);

        assert_eq!(vm.tick(), Ok(NameMe::Ready));
        assert_eq!(vm.mem(), [2, 0, 0, 0, 99]);

        assert_eq!(vm.run(), Ok(NameMe::Halted));
    }

    #[test]
    fn check_example_2() {
        let intcode = vec![2, 3, 0, 3, 99];
        let mut vm = Vm::from_code(&intcode);

        assert_eq!(vm.tick(), Ok(NameMe::Ready));
        assert_eq!(vm.mem(), [2, 3, 0, 6, 99]);

        assert_eq!(vm.run(), Ok(NameMe::Halted));
    }

    #[test]
    fn check_example_3() {
        let intcode = vec![2, 4, 4, 5, 99, 0];
        let mut vm = Vm::from_code(&intcode);

        assert_eq!(vm.tick(), Ok(NameMe::Ready));
        assert_eq!(vm.mem(), [2, 4, 4, 5, 99, 9801]);

        assert_eq!(vm.run(), Ok(NameMe::Halted));
    }

    #[test]
    fn check_example_4() {
        let intcode = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let mut vm = Vm::from_code(&intcode);

        // This example is two ticks long
        assert_eq!(vm.tick(), Ok(NameMe::Ready), "failed on tick 1");
        assert_eq!(vm.tick(), Ok(NameMe::Ready), "failed on tick 2");
        assert_eq!(vm.mem(), [30, 1, 1, 4, 2, 5, 6, 0, 99]);

        assert_eq!(vm.run(), Ok(NameMe::Halted));
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

        // Write input values directly into the intcode
        intcode[1] = 12;
        intcode[2] = 02;

        let mut vm = Vm::from_buffer(intcode);
        assert_eq!(vm.run(), Ok(NameMe::Halted));
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

        // Write input values directly into the intcode
        let mut vm = Vm::from_buffer(intcode);
        assert_eq!(vm.run(), Ok(NameMe::Halted));
        assert_eq!(vm.mem()[0], 19690720, "Expected solution to puzzle 2 part2");
    }
}

#[cfg(test)]
mod day_05 {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn check_echo() {
        for i in 100..=110 {
            let intcode = vec![3, 0, 4, 0, 99];
            let mut vm = Vm::from_buffer(intcode);

            let reason = vm.run();
            assert_eq!(reason, Ok(NameMe::Input), "vm didn't stop to wait on input");

            vm.input(i);

            let reason = vm.run();
            assert_eq!(
                reason,
                Ok(NameMe::Output(i)),
                "vm output didn't match expected output ({})",
                i
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

        let mut vm = Vm::from_buffer(intcode);

        let reason = vm.run();
        assert_eq!(reason, Ok(NameMe::Input), "vm didn't stop to wait on input");

        vm.input(1);

        let mut output = vec![];
        assert_eq!(vm.run_with_output(&mut output), Ok(NameMe::Halted));

        assert_eq!(
            &output,
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 13346482],
            "vm output didn't match expected output"
        );
    }

    #[test]
    fn check_equal_pass_position_mode() {
        let intcode = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];

        let mut vm = Vm::from_buffer(intcode);
        vm.input(8);
        assert_eq!(vm.run(), Ok(NameMe::Output(1)));
        assert_eq!(vm.run(), Ok(NameMe::Halted));
    }

    #[test]
    fn check_less_pass_position_mode() {
        let intcode = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];

        let mut vm = Vm::from_buffer(intcode);
        vm.input(7);
        assert_eq!(vm.run(), Ok(NameMe::Output(1)));
        assert_eq!(vm.run(), Ok(NameMe::Halted));
    }

    #[test]
    fn check_equal_pass_immediate_mode() {
        let intcode = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];

        let mut vm = Vm::from_buffer(intcode);
        vm.input(8);
        assert_eq!(vm.run(), Ok(NameMe::Output(1)));
        assert_eq!(vm.run(), Ok(NameMe::Halted));
    }

    #[test]
    fn check_less_pass_immediate_mode() {
        let intcode = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];

        let mut vm = Vm::from_buffer(intcode);
        vm.input(7);
        assert_eq!(vm.run(), Ok(NameMe::Output(1)));
        assert_eq!(vm.run(), Ok(NameMe::Halted));
    }

    #[test]
    fn check_equal_fail_position_mode() {
        let intcode = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];

        let mut vm = Vm::from_buffer(intcode);
        vm.input(80);
        assert_eq!(vm.run(), Ok(NameMe::Output(0)));
        assert_eq!(vm.run(), Ok(NameMe::Halted));
    }

    #[test]
    fn check_less_fail_position_mode() {
        let intcode = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];

        let mut vm = Vm::from_buffer(intcode);
        vm.input(70);
        assert_eq!(vm.run(), Ok(NameMe::Output(0)));
        assert_eq!(vm.run(), Ok(NameMe::Halted));
    }

    #[test]
    fn check_equal_fail_immediate_mode() {
        let intcode = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];

        let mut vm = Vm::from_buffer(intcode);
        vm.input(80);
        assert_eq!(vm.run(), Ok(NameMe::Output(0)));
        assert_eq!(vm.run(), Ok(NameMe::Halted));
    }

    #[test]
    fn check_less_fail_immediate_mode() {
        let intcode = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];

        let mut vm = Vm::from_buffer(intcode);
        vm.input(70);
        assert_eq!(vm.run(), Ok(NameMe::Output(0)));
        assert_eq!(vm.run(), Ok(NameMe::Halted));
    }

    #[test]
    fn check_jump_nonzero_position_mode() {
        let intcode = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];

        let mut vm = Vm::from_buffer(intcode);
        vm.input(-1);
        assert_eq!(vm.run(), Ok(NameMe::Output(1)));
        assert_eq!(vm.run(), Ok(NameMe::Halted));
    }

    #[test]
    fn check_jump_zero_position_mode() {
        let intcode = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];

        let mut vm = Vm::from_buffer(intcode);
        vm.input(0);
        assert_eq!(vm.run(), Ok(NameMe::Output(0)));
        assert_eq!(vm.run(), Ok(NameMe::Halted));
    }

    #[test]
    fn check_jump_nonzero_immediate_mode() {
        let intcode = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];

        let mut vm = Vm::from_buffer(intcode);
        vm.input(-1);
        assert_eq!(vm.run(), Ok(NameMe::Output(1)));
        assert_eq!(vm.run(), Ok(NameMe::Halted));
    }

    #[test]
    fn check_jump_zero_immediate_mode() {
        let intcode = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];

        let mut vm = Vm::from_buffer(intcode);
        vm.input(0);
        assert_eq!(vm.run(), Ok(NameMe::Output(0)));
        assert_eq!(vm.run(), Ok(NameMe::Halted));
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

        let mut vm = Vm::from_buffer(intcode);

        let reason = vm.run();
        assert_eq!(reason, Ok(NameMe::Input), "vm didn't stop to wait on input");

        vm.input(5);
        let reason = vm.run();
        assert_eq!(reason, Ok(NameMe::Output(12111395)));
    }
}

#[cfg(test)]
mod day_09 {
    use super::*;

    #[test]
    fn check_quine() {
        let quine = &[
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];

        let mut vm = Vm::from_code(quine);
        let mut output = vec![];

        let reason = vm.run_with_output(&mut output);
        assert_eq!(reason, Ok(NameMe::Halted));

        assert_eq!(output, quine);
    }

    #[test]
    fn check_16_digit() {
        let intcode = &[1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let mut vm = Vm::from_code(intcode);

        let reason = vm.run();
        assert_eq!(reason, Ok(NameMe::Output(34915192 * 34915192)));
    }

    const INPUT: &[i64] = &[
        1102, 34463338, 34463338, 63, 1007, 63, 34463338, 63, 1005, 63, 53, 1101, 3, 0, 1000, 109,
        988, 209, 12, 9, 1000, 209, 6, 209, 3, 203, 0, 1008, 1000, 1, 63, 1005, 63, 65, 1008, 1000,
        2, 63, 1005, 63, 902, 1008, 1000, 0, 63, 1005, 63, 58, 4, 25, 104, 0, 99, 4, 0, 104, 0, 99,
        4, 17, 104, 0, 99, 0, 0, 1101, 26, 0, 1015, 1101, 29, 0, 1010, 1102, 1, 24, 1013, 1102, 1,
        33, 1008, 1102, 36, 1, 1012, 1101, 0, 572, 1023, 1101, 35, 0, 1014, 1101, 0, 38, 1019,
        1102, 1, 30, 1006, 1101, 0, 890, 1029, 1101, 34, 0, 1011, 1101, 28, 0, 1002, 1102, 1, 1,
        1021, 1101, 0, 37, 1001, 1101, 0, 197, 1026, 1101, 22, 0, 1017, 1102, 1, 895, 1028, 1101,
        0, 20, 1007, 1102, 21, 1, 1004, 1102, 1, 39, 1016, 1101, 0, 0, 1020, 1102, 1, 190, 1027,
        1101, 0, 775, 1024, 1102, 31, 1, 1018, 1101, 0, 23, 1003, 1101, 0, 25, 1009, 1101, 770, 0,
        1025, 1101, 0, 27, 1000, 1102, 1, 575, 1022, 1101, 0, 32, 1005, 109, 27, 2106, 0, 0, 1001,
        64, 1, 64, 1106, 0, 199, 4, 187, 1002, 64, 2, 64, 109, -18, 21101, 40, 0, 5, 1008, 1014,
        39, 63, 1005, 63, 219, 1106, 0, 225, 4, 205, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -6,
        1201, -1, 0, 63, 1008, 63, 28, 63, 1005, 63, 251, 4, 231, 1001, 64, 1, 64, 1105, 1, 251,
        1002, 64, 2, 64, 109, 5, 21102, 41, 1, 3, 1008, 1011, 38, 63, 1005, 63, 271, 1105, 1, 277,
        4, 257, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -7, 2102, 1, 1, 63, 1008, 63, 28, 63, 1005,
        63, 299, 4, 283, 1106, 0, 303, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -7, 1207, 10, 22, 63,
        1005, 63, 321, 4, 309, 1106, 0, 325, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 16, 2107, 31,
        -4, 63, 1005, 63, 345, 1001, 64, 1, 64, 1105, 1, 347, 4, 331, 1002, 64, 2, 64, 109, -9,
        1201, 3, 0, 63, 1008, 63, 18, 63, 1005, 63, 371, 1001, 64, 1, 64, 1106, 0, 373, 4, 353,
        1002, 64, 2, 64, 109, 7, 1202, -7, 1, 63, 1008, 63, 40, 63, 1005, 63, 393, 1106, 0, 399, 4,
        379, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -5, 1208, 5, 33, 63, 1005, 63, 417, 4, 405,
        1106, 0, 421, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 1, 1202, 2, 1, 63, 1008, 63, 30, 63,
        1005, 63, 443, 4, 427, 1105, 1, 447, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -7, 2102, 1,
        10, 63, 1008, 63, 19, 63, 1005, 63, 471, 1001, 64, 1, 64, 1105, 1, 473, 4, 453, 1002, 64,
        2, 64, 109, 6, 2108, 21, 0, 63, 1005, 63, 489, 1105, 1, 495, 4, 479, 1001, 64, 1, 64, 1002,
        64, 2, 64, 109, 9, 21108, 42, 42, 0, 1005, 1012, 513, 4, 501, 1105, 1, 517, 1001, 64, 1,
        64, 1002, 64, 2, 64, 109, 7, 21107, 43, 44, -1, 1005, 1018, 535, 4, 523, 1106, 0, 539,
        1001, 64, 1, 64, 1002, 64, 2, 64, 109, -5, 21101, 44, 0, 2, 1008, 1016, 44, 63, 1005, 63,
        561, 4, 545, 1105, 1, 565, 1001, 64, 1, 64, 1002, 64, 2, 64, 2105, 1, 9, 1106, 0, 581, 4,
        569, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 13, 21107, 45, 44, -9, 1005, 1018, 597, 1105,
        1, 603, 4, 587, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -25, 2101, 0, 3, 63, 1008, 63, 32,
        63, 1005, 63, 625, 4, 609, 1105, 1, 629, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 7, 1208,
        -7, 30, 63, 1005, 63, 645, 1105, 1, 651, 4, 635, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -2,
        21102, 46, 1, 9, 1008, 1016, 46, 63, 1005, 63, 677, 4, 657, 1001, 64, 1, 64, 1106, 0, 677,
        1002, 64, 2, 64, 109, -2, 21108, 47, 48, 9, 1005, 1014, 697, 1001, 64, 1, 64, 1105, 1, 699,
        4, 683, 1002, 64, 2, 64, 109, 14, 1205, 2, 713, 4, 705, 1105, 1, 717, 1001, 64, 1, 64,
        1002, 64, 2, 64, 109, -7, 1206, 8, 735, 4, 723, 1001, 64, 1, 64, 1106, 0, 735, 1002, 64, 2,
        64, 109, -18, 2101, 0, 6, 63, 1008, 63, 24, 63, 1005, 63, 759, 1001, 64, 1, 64, 1106, 0,
        761, 4, 741, 1002, 64, 2, 64, 109, 29, 2105, 1, 1, 4, 767, 1106, 0, 779, 1001, 64, 1, 64,
        1002, 64, 2, 64, 109, -5, 1206, 3, 791, 1106, 0, 797, 4, 785, 1001, 64, 1, 64, 1002, 64, 2,
        64, 109, -12, 2107, 31, -1, 63, 1005, 63, 819, 4, 803, 1001, 64, 1, 64, 1105, 1, 819, 1002,
        64, 2, 64, 109, 7, 1205, 7, 835, 1001, 64, 1, 64, 1105, 1, 837, 4, 825, 1002, 64, 2, 64,
        109, -11, 1207, 7, 24, 63, 1005, 63, 853, 1106, 0, 859, 4, 843, 1001, 64, 1, 64, 1002, 64,
        2, 64, 109, 4, 2108, 27, -6, 63, 1005, 63, 881, 4, 865, 1001, 64, 1, 64, 1106, 0, 881,
        1002, 64, 2, 64, 109, 24, 2106, 0, -2, 4, 887, 1106, 0, 899, 1001, 64, 1, 64, 4, 64, 99,
        21102, 27, 1, 1, 21101, 0, 913, 0, 1106, 0, 920, 21201, 1, 61934, 1, 204, 1, 99, 109, 3,
        1207, -2, 3, 63, 1005, 63, 962, 21201, -2, -1, 1, 21101, 0, 940, 0, 1106, 0, 920, 21202, 1,
        1, -1, 21201, -2, -3, 1, 21101, 0, 955, 0, 1105, 1, 920, 22201, 1, -1, -2, 1105, 1, 966,
        22102, 1, -2, -2, 109, -3, 2105, 1, 0,
    ];

    #[test]
    fn check_part1_input() {
        let mut vm = Vm::from_code(INPUT);
        vm.input(1);

        let reason = vm.run();
        assert_eq!(reason, Ok(NameMe::Output(2406950601)));
    }
}
