use crate::prelude::*;

#[bitmask(u8)]
#[bitmask_config(vec_debug)]
pub enum Cardinal {
    Norð,
    Souð,
    East,
    West,
}

impl Cardinal {
    pub const ALL_NO_DIAG: [Cardinal; 4] = [Norð, Souð, East, West];

    pub fn rev(&self) -> Self {
        let mut r = Cardinal::none();

        if self.contains(Norð) {
            r |= Souð;
        }
        if self.contains(Souð) {
            r |= Norð;
        }
        if self.contains(East) {
            r |= West;
        }
        if self.contains(West) {
            r |= East;
        }

        r
    }

    pub fn turn_right(&self) -> Self {
        let mut r = Cardinal::none();

        if self.contains(Norð) {
            r |= East;
        }
        if self.contains(Souð) {
            r |= West;
        }
        if self.contains(East) {
            r |= Souð;
        }
        if self.contains(West) {
            r |= Norð;
        }

        r
    }
}
impl From<Cardinal> for IVec2 {
    fn from(val: Cardinal) -> Self {
        let mut r = IVec2::zero();

        if val.contains(Norð) {
            r += IVec2::new(0, 1);
        }
        if val.contains(Souð) {
            r += IVec2::new(0, -1);
        }
        if val.contains(East) {
            r += IVec2::new(1, 0);
        }
        if val.contains(West) {
            r += IVec2::new(-1, 0);
        }

        r
    }
}
