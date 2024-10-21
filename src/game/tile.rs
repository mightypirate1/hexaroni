use super::{HexCoord, TileType};
use crate::renderer::traits::Animation;

#[derive(Copy, Clone, Debug)]
pub struct Tile {
    pub coord: HexCoord,
    pub ttype: TileType,
    pub animation: Animation,
}

impl Tile {
    pub fn new(coord: HexCoord, ttype: TileType) -> Tile {
        Tile {
            coord,
            ttype,
            animation: Animation::Idle{
                coord,
            },
        }
    }
    pub fn empty(coord: HexCoord) -> Tile {
        Tile::new(coord, TileType::Ground)
    }
}
