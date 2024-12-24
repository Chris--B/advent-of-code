use ultraviolet::*;

pub trait VecExt: Sized + Copy {
    type Neighbors;
    type FullNeighbors;

    fn neighbors(&self) -> Self::Neighbors;
    fn full_neighbors(&self) -> Self::FullNeighbors {
        todo!()
    }
}

impl VecExt for IVec2 {
    type Neighbors = [Self; 4];
    type FullNeighbors = [Self; 8];

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

    fn full_neighbors(&self) -> Self::FullNeighbors {
        let a = *self;
        [
            // Positive
            a + Self::unit_x(),
            a + Self::unit_y(),
            // Negative
            a - Self::unit_x(),
            a - Self::unit_y(),
            // Positive Diagonal
            a + (-1, -1).into(),
            a + (1, -1).into(),
            a + (-1, 1).into(),
            a + (1, 1).into(),
        ]
    }
}

impl VecExt for Vec2 {
    type Neighbors = [Self; 4];
    type FullNeighbors = ();

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
    type FullNeighbors = ();

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
    type FullNeighbors = ();

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
