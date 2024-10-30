use crate::engine::{moves, moves::Move, statuses::Status, Game, Object};
use crate::geometry::HexCoord;

#[derive(Debug, Clone)]
pub struct Drag {
    pub object: Object,
    pub targets: Vec<HexCoord>,
    moves: Vec<moves::Move>,
}

impl Drag {
    pub fn create(object: &Object, game: &mut Game) -> Drag {
        let obj = game.get_obj_mut(object).unwrap();
        obj.statuses.push(Status::Dragged);

        let moves = moves::legal_moves(object, &game.board);
        Drag {
            object: object.clone(),
            targets: moves.iter().map(|m| *m.target()).collect(),
            moves,
        }
    }

    pub fn get_move_to(&self, target: &HexCoord) -> Option<&Move> {
        self.moves.iter().find(|m| m.target() == target)
    }

    pub fn has_move_to(&self, target: &HexCoord) -> bool {
        self.targets.contains(target)
    }

    pub fn get_move(&self, target: &HexCoord) -> Option<&moves::Move> {
        self.moves.iter().find(|m| m.target() == target)
    }
}
