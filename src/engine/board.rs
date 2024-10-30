use std::collections::HashSet;

use super::{Object, ObjectType, Player};
use crate::geometry::HexCoord;

pub struct Board {
    pub size: usize,
    pub tiles: Vec<Object>,
    pub objects: Vec<Object>,
    pub current_player: Player,
}

impl Board {
    pub fn test_square(size: usize) -> Board {
        let tiles: Vec<Object> = (0..size)
            .flat_map(move |x| {
                (0..size).map(move |y| {
                    let coord = HexCoord::create(x, y, size);
                    Object::new_tile(y * size + x, coord)
                })
            })
            .collect();

        let mut oid = tiles.len();
        let pieces: Vec<Object> = tiles
            .iter()
            .filter(|t| (23 - t.coord.x + 2 * t.coord.y) % 7 < 3)
            .enumerate()
            .map(|(i, t)| {
                oid += 1;
                let player = if i % 2 == 0 { Player::A } else { Player::B };
                match i % 3 {
                    0 => Object::new(oid, ObjectType::Dasher, t.coord, player),
                    1 => Object::new(oid, ObjectType::Jumper, t.coord, player),
                    _ => Object::new(oid, ObjectType::Wall, t.coord, Player::God),
                }
            })
            .collect();
        Board::new(size, tiles, pieces)
    }

    pub fn new(size: usize, tiles: Vec<Object>, pieces: Vec<Object>) -> Board {
        Board::verify(&tiles, &pieces);
        Board {
            size,
            tiles,
            objects: pieces,
            current_player: Player::A,
        }
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }

    pub fn remove_object(&mut self, object: &Object) {
        self.objects.retain(|o| o.props.oid != object.props.oid);
    }

    pub fn is_empty(&self, coord: &HexCoord) -> bool {
        self.tile_coords().contains(coord)
            && self
                .objects
                .iter()
                .all(|o| &o.coord != coord || o.props.dead)
    }

    pub fn contents(&self, coord: &HexCoord) -> Option<&Object> {
        self.objects
            .iter()
            .find(|o| &o.coord == coord && !o.props.dead)
    }

    pub fn owner(&self, coord: &HexCoord) -> Option<Player> {
        self.contents(coord).map(|o| o.player)
    }

    pub fn next_player(&mut self) {
        self.current_player = self.current_player.opponent();
    }

    fn tile_coords(&self) -> Vec<HexCoord> {
        self.tiles.iter().map(|t| t.coord).collect()
    }

    fn verify(tiles: &[Object], objects: &[Object]) {
        let tile_coords: Vec<HexCoord> = tiles.iter().map(|t| t.coord).collect();
        let mut tcoords = HashSet::new();
        let mut ocoords = HashSet::new();
        let mut oids = HashSet::new();

        tiles.iter().for_each(|t| {
            if !tcoords.insert(t.coord) {
                panic!("Duplicate tile coord");
            }
        });
        objects.iter().for_each(|o| {
            if !ocoords.insert(o.coord) {
                panic!("Duplicate object coord");
            }
        });
        tiles.iter().chain(objects.iter()).for_each(|o| {
            if !oids.insert(o.props.oid) {
                panic!("Duplicate oid");
            }
            if !tile_coords.contains(&o.coord) {
                panic!("Object placed on non-tile");
            }
        });
    }
}
