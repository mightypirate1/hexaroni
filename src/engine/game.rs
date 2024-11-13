use super::{
    moves::{Effect, Move},
    statuses::Status,
    Board, Object, Player,
};
use crate::config::CONF;
use crate::geometry::{HexCoord, ScreenCoord};
use itertools::Itertools;
use macroquad::prelude::*;
use std::time::Instant;

pub struct Game {
    pub board: Board,
    last_move_time: Instant,
    timed_moves: bool,
    allowed_think_time: f32,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            board: Board::test_square(),
            last_move_time: Instant::now(),
            timed_moves: false,
            allowed_think_time: 0.0,
        }
    }
}

impl Game {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Game {
        Game {
            board: Board::test_square(),
            ..Default::default()
        }
    }

    pub fn start_move_timer(&mut self, allowed_think_time: f32) {
        self.last_move_time = Instant::now();
        self.timed_moves = true;
        self.allowed_think_time = allowed_think_time;
    }

    pub fn apply_move(&mut self, r#move: &Move, time: f32, move_duration: f32) {
        if !r#move.object.owned_by(&self.current_player()) {
            return;
        }
        self.move_to(&r#move.object, r#move.target(), time, move_duration);
        for effect in &r#move.effects {
            match effect {
                Effect::Kill { object } => {
                    if let Some(obj) = self.get_obj_mut(object) {
                        let killer_coord = ScreenCoord::from_hexcoord(&r#move.object.coord);
                        let obj_coord = ScreenCoord::from_hexcoord(&object.coord);
                        obj.props.dead = true;
                        obj.statuses.push(Status::Killed {
                            knockback: obj_coord.as_vec() - killer_coord.as_vec(),
                            start_time: time,
                            duration: CONF.kill_duration,
                        });
                    }
                }
            }
        }
        self.board.next_player();
        self.last_move_time = Instant::now();
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
            obj.statuses.push(Status::Move {
                from: ScreenCoord::from_hexcoord(&obj.coord),
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
        self.board
            .tiles
            .iter_mut()
            .chain(self.board.objects.iter_mut())
            .for_each(|o| {
                if o.props.dead && !o.statuses.iter().any(|s| !s.is_expired(time)) {
                    kills.push(o.clone());
                }
                o.statuses = o
                    .statuses
                    .iter()
                    .filter(|s| !s.is_expired(time))
                    .cloned()
                    .collect();
            });
        for obj in kills {
            self.board.remove_object(&obj);
        }

        if self.timed_moves && self.last_move_time.elapsed().as_secs_f32() > self.allowed_think_time
        {
            self.last_move_time = Instant::now();
            self.board.current_player = self.board.current_player.opponent();
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
