use std::collections::HashSet;

use super::{Object, ObjectType, Player};
use crate::geometry::HexCoord;

pub struct Board {
    pub size: usize,
    pub tiles: Vec<Object>,
    pub objects: Vec<Object>,
}

impl Board {
    pub fn test_square() -> Board {
        fn quick_wall(oid: usize, x: usize, y: usize, board_size: usize) -> Object {
            Object::new(
                oid,
                ObjectType::Wall,
                HexCoord::new(x, y, board_size),
                Player::God,
            )
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
                (0..board_size).map(move |y| {
                    let coord = HexCoord::new(x, y, board_size);
                    Object::new_tile(y * board_size + x, coord)
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
        Board::new(board_size, tiles, pieces)
    }

    pub fn new(size: usize, tiles: Vec<Object>, pieces: Vec<Object>) -> Board {
        Board::verify(&tiles, &pieces);
        Board {
            size,
            tiles,
            objects: pieces,
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
                panic!("Duplicate tile coord: {:?}", t.coord);
            }
        });
        objects.iter().for_each(|o| {
            if !ocoords.insert(o.coord) {
                panic!("Duplicate object coord: {:?}", o.coord);
            }
        });
        tiles.iter().chain(objects.iter()).for_each(|o| {
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
