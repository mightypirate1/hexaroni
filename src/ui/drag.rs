use crate::engine::{moves, moves::Move, statuses::Status, Object};
use crate::game::GameController;
use crate::geometry::HexCoord;

#[derive(Debug, Clone)]
pub struct Drag {
    pub object: Object,
    pub targets: Vec<HexCoord>,
    moves: Vec<moves::Move>,
}

impl Drag {
    pub fn create(object: &Object, game: &mut GameController) -> Drag {
        if !game.game_state.allows_moves() {
            return Drag {
                object: object.clone(),
                targets: vec![],
                moves: vec![],
            };
        }

        game.board
            .get_as_mut(object)
            .unwrap()
            .add_status(&Status::new_dragged());
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
