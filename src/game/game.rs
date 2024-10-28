use super::{Board, Object, ObjectType};
use crate::ui::Animation;
use crate::geometry::{HexCoord, ScreenCoord};

pub struct Game {
    pub board: Board
}

impl Game {
    pub fn new() -> Game {
        Game {board: Board::new(7)}
    }

    pub fn move_to(&mut self, object: &Object, to: HexCoord, time: f32, duration: f32) {
        if let Some(obj) = self.get_obj_mut(&object) {
            obj.animation = Some(
                Animation::Move {
                    from: obj.pos,
                    to: ScreenCoord::from_hexcoord(&to),
                    start_time: time,
                    duration,
                }
            );
            obj.move_to(to);
        }
    }

    pub fn set_pos(&mut self, object: &Object, pos: ScreenCoord) {
        if let Some(obj) = self.get_obj_mut(&object) {
            obj.set_pos(pos);
        }
    }

    pub fn get_obj_mut(&mut self, object: &Object) -> Option<&mut Object> {
        self.board.objects.iter_mut().find(|o| o.props.oid == object.props.oid)
    }

    pub fn get_object_at_pos(&self, pos: ScreenCoord) -> Option<Object> {
        self.board.objects
            .iter()
            .find(|o| {
                match o.otype {
                    ObjectType::Tile => false,
                    _ => pos.is_close(o.pos),
                }
            })
            .cloned()
    }

    pub fn get_tile_at_pos(&self, pos: ScreenCoord) -> Option<Object> {   
        self.board.objects
            .iter()
            .find(|o| {
                match o.otype {
                    ObjectType::Tile => pos.is_close(o.pos),
                    _ => false,
                }
            })
            .cloned()
    }

    /**
    run on start of game loop tick
    - resets expired animations to none
    - resizes positions
     */
    pub fn on_tick_start(&mut self, time: f32) {
        for object in self.board.objects.iter_mut() {
            if let Some(animation) = &object.animation {
                if animation.is_expired(time) {
                    object.animation = None;
                    object.set_pos_to_coord();
                }
            }
            object.set_pos_to_coord();
        }
    }
}
