use super::{HexCoord, Object, ObjectType};

#[derive(Copy, Clone, Debug)]
pub struct Block {
    pub coord: HexCoord,
    pub object: Option<Object>,
}

impl Block {
    pub fn with(otype: ObjectType, coord: HexCoord) -> Block {
        Block {
            coord,
            object: Some(Object::new(otype)),
        }
    }
    pub fn empty(coord: HexCoord) -> Block {
        Block {
            coord,
            object: None,
        }
    }
}
