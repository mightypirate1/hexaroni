use super::{
    moves::{Effect, Move},
    Board, Object, Player,
};
use crate::geometry::{HexCoord, ScreenCoord};
use crate::ui::Animation;
use itertools::Itertools;
use macroquad::prelude::*;

pub struct Game {
    pub board: Board,
}

impl Game {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Game {
        Game {
            board: Board::test_square(),
        }
    }

    pub fn apply_move(&mut self, r#move: &Move, time: f32, duration: f32) {
        self.move_to(&r#move.object, r#move.target(), time, duration);
        for effect in &r#move.effects {
            match effect {
                Effect::Kill { object } => {
                    if let Some(obj) = self.get_obj_mut(object) {
                        obj.props.dead = true;
                        obj.animation = Some(Animation::Kill {
                            pos: obj.get_display_pos(time),
                            start_time: time,
                            duration,
                        });
                    }
                }
            }
        }
        self.board.next_player();
    }

    pub fn winner(&self) -> Option<Player> {
        fn is_alive(board: &Board, player: Player) -> bool {
            board.objects.iter().any(|o| o.player == player)
        }
        if !is_alive(&self.board, Player::A) {
            return Some(Player::B);
        }
        if !is_alive(&self.board, Player::B) {
            return Some(Player::A);
        }
        None
    }

    pub fn current_player(&self) -> Player {
        match self.winner() {
            None => self.board.current_player,
            Some(player) => player,
        }
    }

    pub fn move_to(&mut self, object: &Object, to: &HexCoord, time: f32, duration: f32) {
        if let Some(obj) = self.get_obj_mut(object) {
            obj.animation = Some(Animation::Move {
                from: obj.get_display_pos(time),
                to: ScreenCoord::from_hexcoord(to),
                start_time: time,
                duration,
            });
            obj.set_coord(to);
        }
    }

    pub fn get_obj_mut(&mut self, object: &Object) -> Option<&mut Object> {
        self.board
            .objects
            .iter_mut()
            .find(|o| o.props.oid == object.props.oid)
    }

    pub fn get_object_at_pos(&self, pos: &ScreenCoord) -> Option<Object> {
        self.get_close_from_vec(pos, &self.board.objects)
    }

    pub fn get_tile_at_pos(&self, pos: &ScreenCoord) -> Option<Object> {
        self.get_close_from_vec(pos, &self.board.tiles)
    }

    /**
    run on start of game loop tick
    - cleans up dead objects
    - resets expired animations to none
     */
    pub fn on_tick_start(&mut self, time: f32) {
        let mut kills = vec![];
        for obj in self
            .board
            .tiles
            .iter_mut()
            .chain(self.board.objects.iter_mut())
        {
            if let Some(animation) = &obj.animation {
                if animation.is_expired(time) {
                    obj.animation = None;
                    if obj.props.dead {
                        kills.push(obj.clone());
                    }
                }
            }
        }
        for obj in kills {
            self.board.remove_object(&obj);
        }
    }

    pub fn screen_size(&self) -> f32 {
        f32::min(
            0.33 * screen_width() / self.board.size as f32,
            0.58 * screen_height() / (1 + self.board.size) as f32,
        )
    }

    /**
     * Gets the closest object out of the ones that are closer than the size of the object
     */
    fn get_close_from_vec(&self, pos: &ScreenCoord, objects: &[Object]) -> Option<Object> {
        let screen_size = self.screen_size();
        let with_distances: Vec<(&Object, f32)> = objects
            .iter()
            .map(|o| (o, pos.dist_from(&o.get_screen_coord())))
            .collect();
        let detection = with_distances
            .iter()
            .filter(|(o, d)| *d < screen_size * o.props.size)
            .sorted_by(|(_, d1), (_, d2)| f32::total_cmp(d1, d2))
            .map(|(o, _)| o)
            .next();

        detection.map(|o| o.to_owned().to_owned())
    }
}
