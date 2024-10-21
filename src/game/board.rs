use super::{HexCoord, Tile, Object, ObjectType};

pub struct Board {
    pub size: usize,
    pub tiles: Vec<Tile>,
    pub objects: Vec<Object>,
}

impl Board {
    pub fn new(size: usize) -> Board {
        let tiles: Vec<Tile> = (0..size)
            .map(|x| {
                let row: Vec<Tile> = (0..size)
                    .map(|y| {
                        let coord = HexCoord::create(x, y, size);
                        Tile::empty(coord)
                    })
                    .collect();
                row
            })
            .flatten()
            .collect();

        let objects = tiles
            .iter()
            .filter(|t| {
                (23 - t.coord.x + 2 * t.coord.y) % 7 < 3
            })
            .enumerate()
            .map(|(i, t)| {
                match i % 3 {
                    0 => Object::new(ObjectType::Dasher, t.coord),
                    1 => Object::new(ObjectType::Jumper, t.coord),
                    _ => Object::new(ObjectType::Wall, t.coord),
                }
            })
            .collect();
        Board {size, tiles, objects}
    }

    pub fn render(&self) -> String {
        format!("Game[size={}]", self.size)
    }
}
