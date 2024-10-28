use super::{Object, ObjectType};
use crate::geometry::HexCoord;


pub struct Board {
    pub size: usize,
    pub objects: Vec<Object>,
}

impl Board {
    pub fn new(size: usize) -> Board {
        /*
          Remember that everythin breaks if object ids are non-unique
         */

        let tiles: Vec<Object> = (0..size)
            .map(move |x| {
                (0..size)
                .map(move |y| {
                        let coord = HexCoord::create(x, y, size);
                        Object::new_tile(y * size + x, coord)
                    }
                )
            })
            .flatten()
            .collect();
        
        let mut oid = tiles.len();
        let pieces: Vec<Object> = tiles
            .iter()
            .filter(|t| {
                (23 - t.coord.x + 2 * t.coord.y) % 7 < 3
            })
            .enumerate()
            .map(|(i, t)| {
                oid += 1;
                match i % 3 {
                    0 => Object::new(oid, ObjectType::Dasher, t.coord),
                    1 => Object::new(oid, ObjectType::Jumper, t.coord),
                    _ => Object::new(oid, ObjectType::Wall, t.coord),
                }
            })
            .collect();
        Board {size, objects: tiles.into_iter().chain(pieces).collect()}
    }
}
