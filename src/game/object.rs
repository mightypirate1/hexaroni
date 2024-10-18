use super::{HexCoord, ObjectType};


#[derive(Copy, Clone, Debug)]
pub struct Object {
    pub otype: ObjectType,
    pub coord: HexCoord,
}

impl Object {
    pub fn new(otype: ObjectType, coord: HexCoord) -> Object {
        Object {otype, coord}
    }
}