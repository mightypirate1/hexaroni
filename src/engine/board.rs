use std::collections::HashSet;

use crate::engine::{statuses::Status, Object, ObjectType, Player};
use crate::geometry::HexCoord;

#[derive(Clone)]
pub struct Board {
    pub size: usize,
    objects: Vec<Object>,
}

impl Board {
    pub fn test_square() -> Board {
        fn quick_wall(oid: usize, x: usize, y: usize, board_size: usize) -> Object {
            Object::new_wall(oid, HexCoord::new(x, y, board_size))
        }
        fn quick_piece(
            oid: usize,
            x: usize,
            y: usize,
            otype: ObjectType,
            board_size: usize,
        ) -> Object {
            Object::new(
                oid,
                otype,
                HexCoord::new(x, y, board_size),
                if oid % 2 == 0 { Player::A } else { Player::B },
            )
        }

        let board_size = 7;
        let tiles: Vec<Object> = (0..board_size)
            .flat_map(move |x| {
                fn delay(x: usize) -> usize {
                    ((33.0 * x.abs_diff(3) as f32) / 7.0).round() as usize
                }
                (0..board_size).map(move |y| {
                    let coord = HexCoord::new(x, y, board_size);
                    let lifespan = 48 - delay(x) - delay(y);
                    Object::new_tile(y * board_size + x, coord, lifespan)
                })
            })
            .collect();
        let oid = tiles.len();
        let pieces = vec![
            quick_wall(oid, 3, 4, board_size),
            quick_wall(oid + 1, 2, 2, board_size),
            quick_wall(oid + 2, 3, 2, board_size),
            quick_wall(oid + 3, 4, 4, board_size),
            quick_wall(oid + 4, 0, 4, board_size),
            quick_wall(oid + 5, 6, 2, board_size),
            // cow
            quick_piece(oid + 6, 1, 1, ObjectType::Dasher, board_size),
            quick_piece(oid + 7, 5, 5, ObjectType::Dasher, board_size),
            quick_piece(oid + 8, 1, 2, ObjectType::Dasher, board_size),
            quick_piece(oid + 9, 4, 5, ObjectType::Dasher, board_size),
            quick_piece(oid + 10, 2, 1, ObjectType::Dasher, board_size),
            quick_piece(oid + 11, 5, 4, ObjectType::Dasher, board_size),
            quick_piece(oid + 12, 4, 0, ObjectType::Dasher, board_size),
            quick_piece(oid + 13, 2, 6, ObjectType::Dasher, board_size),
            // horse
            quick_piece(oid + 14, 0, 3, ObjectType::Jumper, board_size),
            quick_piece(oid + 15, 6, 3, ObjectType::Jumper, board_size),
            quick_piece(oid + 16, 6, 6, ObjectType::Jumper, board_size),
            quick_piece(oid + 17, 0, 0, ObjectType::Jumper, board_size),
            quick_piece(oid + 18, 1, 4, ObjectType::Jumper, board_size),
            quick_piece(oid + 19, 5, 2, ObjectType::Jumper, board_size),
            quick_piece(oid + 20, 0, 5, ObjectType::Jumper, board_size),
            quick_piece(oid + 21, 6, 1, ObjectType::Jumper, board_size),
        ];
        Board::new(
            board_size,
            pieces.iter().chain(tiles.iter()).cloned().collect(),
        )
    }

    pub fn new(size: usize, objects: Vec<Object>) -> Board {
        Board::verify(&objects);
        Board { size, objects }
    }

    pub fn tiles(&self) -> Vec<&Object> {
        self.objects.iter().filter(|o| o.is_tile()).collect()
    }

    pub fn tiles_mut(&mut self) -> Vec<&mut Object> {
        self.objects.iter_mut().filter(|o| o.is_tile()).collect()
    }

    pub fn pieces(&self) -> Vec<&Object> {
        self.objects.iter().filter(|o| !o.is_tile()).collect()
    }

    pub fn pieces_mut(&mut self) -> Vec<&mut Object> {
        self.objects.iter_mut().filter(|o| !o.is_tile()).collect()
    }

    pub fn objects(&self) -> Vec<&Object> {
        self.objects.iter().collect()
    }

    pub fn objects_mut(&mut self) -> Vec<&mut Object> {
        self.objects.iter_mut().collect()
    }

    pub fn tile_at(&self, coord: &HexCoord) -> Option<&Object> {
        self.tiles().iter().find(|o| &o.coord == coord).copied()
    }

    pub fn piece_at(&self, coord: &HexCoord) -> Option<&Object> {
        self.pieces().iter().find(|o| &o.coord == coord).copied()
    }

    pub fn get_as_mut(&mut self, object: &Object) -> Option<&mut Object> {
        self.objects
            .iter_mut()
            .find(|o| o.props.oid == object.props.oid)
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }

    pub fn remove_object(&mut self, object: &Object) {
        self.objects.retain(|o| o.props.oid != object.props.oid);
    }

    pub fn kill_piece_at(&mut self, coord: &HexCoord, status: Option<Status>) {
        self.pieces_mut()
            .iter_mut()
            .filter(|p| &p.coord == coord)
            .for_each(|p| p.set_killed(status.as_ref()))
    }

    pub fn kill_all_at(&mut self, coord: &HexCoord, status: Option<Status>) {
        self.objects_mut()
            .iter_mut()
            .filter(|o| &o.coord == coord)
            .for_each(|o| o.set_killed(status.as_ref()))
    }

    pub fn is_empty(&self, coord: &HexCoord) -> bool {
        match self.piece_at(coord) {
            Some(p) => p.props.dead,
            None => true,
        }
    }

    pub fn contents(&self, coord: &HexCoord) -> Option<&Object> {
        self.pieces()
            .iter()
            .find(|p| &p.coord == coord && !p.props.dead)
            .cloned()
    }

    pub fn owner(&self, coord: &HexCoord) -> Option<Player> {
        self.contents(coord).map(|o| o.player)
    }

    fn verify(objects: &[Object]) {
        let tiles: Vec<&Object> = objects.iter().filter(|o| o.is_tile()).collect();
        let non_tiles: Vec<&Object> = objects.iter().filter(|o| !o.is_tile()).collect();
        let tile_coords: Vec<HexCoord> = tiles.iter().map(|t| t.coord).collect();
        let mut tcoords = HashSet::new();
        let mut ocoords = HashSet::new();
        let mut oids = HashSet::new();

        tiles.iter().for_each(|t| {
            if !tcoords.insert(t.coord) {
                panic!("Duplicate tile coord: {:?}", t.coord);
            }
        });
        non_tiles.iter().for_each(|o| {
            if !ocoords.insert(o.coord) {
                panic!("Duplicate object coord: {:?}", o.coord);
            }
        });
        tiles.iter().chain(non_tiles.iter()).for_each(|o| {
            if !oids.insert(o.props.oid) {
                panic!("Duplicate oid: oid={:?}", o.props.oid);
            }
            if !tile_coords.contains(&o.coord) {
                panic!("Object placed on non-tile: oid={:?}", o.props.oid);
            }
        });
        for player in [Player::A, Player::B] {
            if !objects.iter().any(|o| o.owned_by(&player)) {
                panic!("No pieces for {:?}", player);
            }
        }
    }
}
