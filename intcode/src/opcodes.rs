#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Opcode {
    Add = 1,
    Mul = 2,

    Hlt = 99,
}

impl Opcode {
    pub fn from_atom(atom: crate::vm::Atom) -> Option<Opcode> {
        match atom {
            1 => Some(Opcode::Add),
            2 => Some(Opcode::Mul),

            99 => Some(Opcode::Hlt),
            _ => None,
        }
    }
}
