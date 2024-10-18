use super::{HexCoord, Block, Object, ObjectType};

pub struct Board {
    size: usize,
    pub blocks: Vec<Block>,
}

impl Board {
    pub fn new(size: usize) -> Board {
        let blocks: Vec<Block> = (0..size)
            .map(|x| {
                let row: Vec<Block> = (0..size)
                    .map(|y| {
                        let coord = HexCoord::create(x, y, size);
                        match (23 - x + 2 * y) % 7 {
                            0 => Block::with(ObjectType::Dasher, coord),
                            1 => Block::with(ObjectType::Jumper, coord),
                            2 => Block::with(ObjectType::Wall, coord),
                            _ => Block::empty(coord),
                        }
                    })
                    .collect();
                row
            })
            .flatten()
            .collect();

        Board {size, blocks}
    }

    pub fn render(&self) -> String {
        format!("Game[size={}]", self.size)
    }
}
