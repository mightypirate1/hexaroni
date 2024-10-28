use crate::game::{
    ObjectType,
    objects::ObjectProps,
    statuses::Status,
};
use crate::geometry::{HexCoord, ScreenCoord};
use crate::ui::Animation;


#[derive(Clone, Debug)]
pub struct Object {
    pub otype: ObjectType,
    pub coord: HexCoord,
    pub props: ObjectProps,
    pub pos: ScreenCoord,
    pub statuses: Vec<Status>,
    pub animation: Option<Animation>,
}


impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        self.props.oid == other.props.oid
    }
}


impl Object {
    pub fn new(oid: usize, otype: ObjectType, coord: HexCoord) -> Object {
        Object {
            otype,
            coord,
            props: ObjectProps{oid, ..Default::default()},
            pos: ScreenCoord::from_hexcoord(&coord),
            animation: None,
            statuses: vec![],
        }
    }

    pub fn new_tile(oid: usize, coord: HexCoord) -> Object {
        Object {
            otype: ObjectType::Tile,
            coord,
            props: ObjectProps{
                oid,
                selectable: false,
                draggable: false,
                ..Default::default()
            },
            pos: ScreenCoord::from_hexcoord(&coord),
            animation: None,
            statuses: vec![],
        }
    }

    pub fn get_screen_coord(&self, time: f32) -> ScreenCoord {
        match &self.animation {
            Some(animation) => animation.get_pos(time),
            None => self.pos,
        }
    }

    pub fn move_to(&mut self, coord: HexCoord) {
        self.set_coord(coord);
        self.set_pos_to_coord();
    }

    pub fn set_pos(&mut self, pos: ScreenCoord) {
        self.pos = pos;
    }

    pub fn set_coord(&mut self, coord: HexCoord) {
        self.coord = coord;
    }

    pub fn set_pos_to_coord(&mut self) {
        self.pos = ScreenCoord::from_hexcoord(&self.coord);
    }

    pub fn add_status(&mut self, status: Status) {
        self.statuses.push(status);
    }

    pub fn remove_status(&mut self, status: Status) {
        self.statuses.retain(|s| *s != status);
    }
}
