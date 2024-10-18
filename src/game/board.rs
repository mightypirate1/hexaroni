use super::{HexCoord, Block, Object, ObjectType};

pub struct Board {
    pub size: usize,
    pub blocks: Vec<Block>,
    pub objects: Vec<Object>,
}

impl Board {
    pub fn new(size: usize) -> Board {
        let blocks: Vec<Block> = (0..size)
            .map(|x| {
                let row: Vec<Block> = (0..size)
                    .map(|y| {
                        let coord = HexCoord::create(x, y, size);
                        Block::empty(coord)
                    })
                    .collect();
                row
            })
            .flatten()
            .collect();

        let objects = blocks
            .iter()
            .filter(|b| {
                (23 - b.coord.x + 2 * b.coord.y) % 7 < 3
            })
            .enumerate()
            .map(|(i, b)| {
                match i % 3 {
                    0 => Object::new(ObjectType::Dasher, b.coord),
                    1 => Object::new(ObjectType::Jumper, b.coord),
                    _ => Object::new(ObjectType::Wall, b.coord),
                }
            })
            .collect();
        Board {size, blocks, objects}
    }

    pub fn render(&self) -> String {
        format!("Game[size={}]", self.size)
    }
}
