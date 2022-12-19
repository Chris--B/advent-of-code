use ultraviolet::*;

pub trait VecExt: Sized + Copy {
    type Neighbors;

    fn neighbors(&self) -> Self::Neighbors;
}

impl VecExt for IVec2 {
    type Neighbors = [Self; 4];

    fn neighbors(&self) -> Self::Neighbors {
        let a = *self;
        [
            // Positive
            a + Self::unit_x(),
            a + Self::unit_y(),
            // Negative
            a - Self::unit_x(),
            a - Self::unit_y(),
        ]
    }
}

impl VecExt for Vec2 {
    type Neighbors = [Self; 4];

    fn neighbors(&self) -> Self::Neighbors {
        let a = *self;
        [
            // Positive
            a + Self::unit_x(),
            a + Self::unit_y(),
            // Negative
            a - Self::unit_x(),
            a - Self::unit_y(),
        ]
    }
}

impl VecExt for IVec3 {
    type Neighbors = [Self; 6];

    fn neighbors(&self) -> Self::Neighbors {
        let a = *self;
        [
            // Positive
            a + Self::unit_x(),
            a + Self::unit_y(),
            a + Self::unit_z(),
            // Negative
            a - Self::unit_x(),
            a - Self::unit_y(),
            a - Self::unit_z(),
        ]
    }
}

impl VecExt for Vec3 {
    type Neighbors = [Self; 6];

    fn neighbors(&self) -> Self::Neighbors {
        let a = *self;
        [
            // Positive
            a + Self::unit_x(),
            a + Self::unit_y(),
            a + Self::unit_z(),
            // Negative
            a - Self::unit_x(),
            a - Self::unit_y(),
            a - Self::unit_z(),
        ]
    }
}
