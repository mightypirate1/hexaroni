use super::{Board, Object, moves::{Move, Effect}};
use crate::ui::Animation;
use crate::geometry::{HexCoord, ScreenCoord};

pub struct Game {
    pub board: Board
}

impl Game {
    pub fn new() -> Game {
        Game {board: Board::test_square(7)}
    }

    pub fn apply_move(&mut self, r#move: &Move, time: f32, duration: f32) {
        self.move_to(&r#move.object, r#move.target(), time, duration);
        for effect in &r#move.effects {
            match effect {
                Effect::Kill {object} => {
                    if let Some(obj) = self.get_obj_mut(&object) {
                        obj.props.dead = true;
                        obj.animation = Some(
                            Animation::Kill {
                                pos: obj.get_display_pos(time),
                                start_time: time,
                                duration,
                            }
                        );
                    }
                }
            }
        }
        self.board.next_player();
    }


    pub fn move_to(&mut self, object: &Object, to: &HexCoord, time: f32, duration: f32) {
        if let Some(obj) = self.get_obj_mut(&object) {
            obj.animation = Some(
                Animation::Move {
                    from: obj.get_display_pos(time),
                    to: ScreenCoord::from_hexcoord(&to),
                    start_time: time,
                    duration,
                }
            );
            obj.set_coord(to);
        }
    }

    pub fn get_obj_mut(&mut self, object: &Object) -> Option<&mut Object> {
        self.board.objects
            .iter_mut()
            .find(|o| o.props.oid == object.props.oid)
    }

    pub fn get_object_at_pos(&self, pos: ScreenCoord) -> Option<Object> {
        self.board.objects
            .iter()
            .find(|o| {
                pos.is_close(o.get_screen_coord()) && !o.props.dead
            })
            .cloned()
    }

    pub fn get_tile_at_pos(&self, pos: ScreenCoord) -> Option<Object> {   
        self.board.tiles
            .iter()
            .find(|o| {
                pos.is_close(o.get_screen_coord()) && !o.props.dead
            })
            .cloned()
    }

    /**
    run on start of game loop tick
    - cleans up dead objects
    - resets expired animations to none
     */
    pub fn on_tick_start(&mut self, time: f32) {
        let mut kills = vec![];
        for obj in self.board.tiles.iter_mut().chain(self.board.objects.iter_mut()) {
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
            let x = self.board.objects.len();
            self.board.remove_object(&obj);
        }
    }
}
