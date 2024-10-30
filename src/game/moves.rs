use crate::geometry::HexCoord;
use super::{Object, ObjectType, Board, Player};

#[derive(Debug, Clone)]
pub struct Move {
    pub object: Object,
    pub path: Vec<HexCoord>,
    pub effects: Vec<Effect>,
}

#[derive(Debug, Clone)]
pub enum Effect {
    Kill {object: Object},
}

impl Move {
    pub fn new(object: Object, path: Vec<HexCoord>, effects: Vec<Effect>) -> Move {
        if path.len() < 2 {
            panic!("Path must have at least two coordinates");
        }
        Move {object, path, effects}
    }

    pub fn target(&self) -> &HexCoord {
        self.path.last().unwrap()
    }
}

pub fn legal_moves(object: &Object, board: &Board) -> Vec<Move> {
    match object.otype {
        ObjectType::Dasher => dasher_moves(object, board),
        ObjectType::Jumper => jumper_moves(object, board),
        ObjectType::Wall => wall_moves(object, board),
        _ => vec![],
    }
}

fn wall_moves(object: &Object, board: &Board) -> Vec<Move> {
    let walks: Vec<Move> = object.coord
        .get_all_neighbours(1)
        .iter()
        .filter(|c| board.is_empty(c))
        .map(|&c| Move::new(object.clone(), vec![object.coord, c], vec![]))
        .collect();
    let kills: Vec<Move> = object.coord
        .get_all_neighbours(1)
        .iter()
        .filter(|c| {
            let target = board.contents(c);
            match target {
                Some(t) => !t.owned_by(&object.player),
                None => false,
            }
        })
        .map(|&c| Move::new(
            object.clone(), 
            vec![object.coord, c],
            vec![Effect::Kill{object: board.contents(&c).unwrap().clone()}])
        )
        .collect();
    walks.into_iter().chain(kills.into_iter()).collect()
}


fn jumper_moves(object: &Object, board: &Board) -> Vec<Move> {
    fn create_move(obj: &Object, target: HexCoord, inter: &HexCoord, board: &Board) -> Move {
        let effects = match board.contents(&target) {
            Some(t) => {
                if t.owned_by(&obj.player.opponent()) {
                    vec![Effect::Kill{object: t.clone()}]
                } else {
                    vec![]
                }
            },
            None => vec![],
        };
        Move::new(
            obj.clone(), 
            vec![obj.coord, inter.clone(), target],
            effects,
    )
    }
    fn get_hook(obj: &Object, dir: usize, clockw: bool, board: &Board) -> Option<Move> {
        let intermediate = obj.coord.get_neighbor(dir, 2);
        match intermediate {
            Some(inter) => {
                let hook_dir = (dir + if clockw {1} else {5}) % 6;
                if let Some(target) = inter.get_neighbor(hook_dir, 1) {
                    if board.is_empty(&target) 
                    || board.owner(&target) == Some(obj.player.opponent()) {
                        return Some(create_move(obj, target, &inter, board));
                    }
                }
                None
            },
            None => None,
        }
    }
    fn get_hooks(obj: &Object, dir: usize, board: &Board) -> Vec<Option<Move>> {
        vec![
            get_hook(obj, dir, false, board),
            get_hook(obj, dir, true, board),
        ]
    }
    object.coord.get_all_directions()
        .iter()
        .map(|&dir| get_hooks(object, dir, board))
        .into_iter()
        .flatten()
        .flatten()
        .collect()
}


fn dasher_moves(object: &Object, board: &Board) -> Vec<Move> {
    fn create_move(path: Vec<HexCoord>, obj: &Object, effects: Vec<Effect>) -> Move {
        Move::new(obj.clone(), path, effects)
    }
    fn create_path(obj: &Object, dir: &usize, board: &Board) -> (Vec<HexCoord>, Vec<Effect>) {
        let mut path = vec![];
        let mut curr = Some(obj.coord);
        let mut effects = vec![];
        while let Some(c) = curr{
            path.push(c);
            let next = c.get_neighbor(*dir, 1);
            match next {
                Some(n) => {
                    if let Some(next_tile_owner) = board.owner(&n) {
                        if next_tile_owner != obj.player.opponent()
                        && !obj.props.dead {
                            break;
                        }
                    }
                    if let Some(target) = board.contents(&n) {
                        effects.push(Effect::Kill{object: target.clone()});
                    }
                },
                None => break,
            }
            curr = next;
        }
        (path, effects)
    }
    object.coord.get_all_directions()
        .iter()
        .map(|dir| create_path(object, dir, board))
        .filter(|(p, _)| p.len() > 1)
        .map(|(p, es)| create_move(p, object, es))
        .collect()
}
