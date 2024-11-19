use crate::config::CONF;
use crate::engine::statuses::Effect;
use crate::engine::{Board, Object, ObjectType, Player};
use crate::geometry::{HexCoord, ScreenCoord};

#[derive(Debug, Clone)]
pub struct Move {
    pub object: Object,
    pub path: Vec<HexCoord>,
    pub effects: Vec<Effect>,
}

impl Move {
    pub fn new(object: Object, path: Vec<HexCoord>, effects: Vec<Effect>) -> Move {
        if path.len() < 2 {
            panic!("Path must have at least two coordinates");
        }
        Move {
            object,
            path,
            effects,
        }
    }

    pub fn target(&self) -> &HexCoord {
        self.path.last().unwrap()
    }
}

pub fn legal_moves(object: &Object, board: &Board) -> Vec<Move> {
    match object.otype {
        ObjectType::Dasher => dasher_moves(object, board),
        ObjectType::Jumper => jumper_moves(object, board),
        _ => vec![],
    }
}

fn jumper_moves(object: &Object, board: &Board) -> Vec<Move> {
    fn create_move(obj: &Object, target: HexCoord, inter: &HexCoord, board: &Board) -> Move {
        let effects = match board.contents(&target) {
            Some(t) => {
                if t.owned_by(&obj.player.opponent()) {
                    vec![Effect::Kill {
                        victim: t.clone(),
                        killer: Some(obj.clone()),
                        animation_delay_frac: Some(0.45),
                    }]
                } else {
                    vec![]
                }
            }
            None => vec![],
        };
        Move::new(obj.clone(), vec![obj.coord, *inter, target], effects)
    }
    fn get_hook(obj: &Object, dir: usize, clockw: bool, board: &Board) -> Option<Move> {
        let intermediate = obj.coord.get_neighbor(dir, 2);
        match intermediate {
            Some(inter) => {
                let hook_dir = (dir + if clockw { 1 } else { 5 }) % 6;
                if let Some(target) = inter.get_neighbor(hook_dir, 1) {
                    if tile_available_for_step(&target, board, Some(obj.player.opponent())) {
                        return Some(create_move(obj, target, &inter, board));
                    }
                }
                None
            }
            None => None,
        }
    }
    fn get_hooks(obj: &Object, dir: usize, board: &Board) -> Vec<Option<Move>> {
        vec![
            get_hook(obj, dir, false, board),
            get_hook(obj, dir, true, board),
        ]
    }
    object
        .coord
        .get_all_directions()
        .iter()
        .flat_map(|&dir| get_hooks(object, dir, board))
        .flatten()
        .collect()
}

fn dasher_moves(object: &Object, board: &Board) -> Vec<Move> {
    fn create_move(path: Vec<HexCoord>, obj: &Object, effects: Vec<Effect>) -> Move {
        Move::new(obj.clone(), path, effects)
    }
    fn delay_frac(start: &HexCoord, end: &HexCoord, victim_coord: &HexCoord) -> f32 {
        let a = ScreenCoord::from_hexcoord(start);
        let b = ScreenCoord::from_hexcoord(victim_coord);
        let c = ScreenCoord::from_hexcoord(end);
        let full_dist = a.dist_from(&c);
        let victim_dist = a.dist_from(&b);
        0.65 * victim_dist / full_dist
    }

    fn create_path(obj: &Object, dir: &usize, board: &Board) -> (Vec<HexCoord>, Vec<Effect>) {
        let mut path = vec![];
        let mut curr = Some(obj.coord);
        let mut victims_and_coords = vec![];
        while let Some(c) = curr {
            path.push(c);
            let next_tile = c.get_neighbor(*dir, 1);
            match next_tile {
                Some(next) => {
                    if tile_available_for_step(&next, board, Some(obj.player.opponent()))
                        || CONF.dasher_can_fly && board.tile_at(&next).is_none()
                    {
                        if let Some(victim) = board.contents(&next) {
                            victims_and_coords.push((victim, next));
                        }
                    } else {
                        break;
                    }
                }
                None => break,
            }
            curr = next_tile;
        }
        let effects = victims_and_coords
            .iter()
            .map(|(v, c)| Effect::Kill {
                victim: (*v).clone(),
                killer: Some(obj.clone()),
                animation_delay_frac: Some(delay_frac(&obj.coord, path.last().unwrap(), c)),
            })
            .collect();

        (path, effects)
    }
    object
        .coord
        .get_all_directions()
        .iter()
        .map(|dir| create_path(object, dir, board))
        .filter(|(p, _)| p.len() > 1)
        .map(|(p, es)| create_move(p, object, es))
        .collect()
}

/**
tells if a tile is:
- existing
- empty (or has the tolerated player)
- not dead
*/
fn tile_available_for_step(
    tile_coord: &HexCoord,
    board: &Board,
    tolerated_player: Option<Player>,
) -> bool {
    // false if tile is dead, or non-existant
    match board.tile_at(tile_coord) {
        Some(tile) => {
            if tile.props.dead {
                return false;
            }
        }
        None => return false,
    }
    // true if the tile contains no piece, or a piece owned by the tolerated player
    match tolerated_player {
        Some(tolerated) => board
            .piece_at(tile_coord)
            .map(|p| p.owned_by(&tolerated))
            .unwrap_or_else(|| true),
        None => true,
    }
}
