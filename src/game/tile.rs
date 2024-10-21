use super::{HexCoord, TileType};

#[derive(Copy, Clone, Debug)]
pub struct Tile {
    pub coord: HexCoord,
    pub ttype: TileType,
}

impl Tile {
    pub fn new(coord: HexCoord, ttype: TileType) -> Tile {
        Tile {
            coord,
            ttype,
        }
    }
    pub fn empty(coord: HexCoord) -> Tile {
        Tile {
            coord,
            ttype: TileType::Ground,
        }
    }
}
