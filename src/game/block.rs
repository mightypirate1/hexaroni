use super::{HexCoord, BlockType};

#[derive(Copy, Clone, Debug)]
pub struct Block {
    pub coord: HexCoord,
    pub btype: BlockType,
}

impl Block {
    pub fn new(coord: HexCoord, btype: BlockType) -> Block {
        Block {
            coord,
            btype,
        }
    }
    pub fn empty(coord: HexCoord) -> Block {
        Block {
            coord,
            btype: BlockType::Ground,
        }
    }
}
