use crate::engine::{objects::ObjectProps, statuses::Status, ObjectType, Player};
use crate::geometry::{HexCoord, ScreenCoord};
use crate::ui::Animation;

#[derive(Clone, Debug)]
pub struct Object {
    pub otype: ObjectType,
    pub coord: HexCoord,
    pub props: ObjectProps,
    pub statuses: Vec<Status>,
    pub animation: Option<Animation>,
    pub player: Player,
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        self.props.oid == other.props.oid
    }
}

impl Object {
    pub fn new(oid: usize, otype: ObjectType, coord: HexCoord, player: Player) -> Object {
        Object {
            otype,
            coord,
            props: ObjectProps {
                oid,
                ..Default::default()
            },
            animation: None,
            statuses: vec![],
            player,
        }
    }

    pub fn new_tile(oid: usize, coord: HexCoord) -> Object {
        Object {
            otype: ObjectType::Tile,
            coord,
            props: ObjectProps {
                oid,
                selectable: false,
                draggable: false,
                ..Default::default()
            },
            animation: None,
            statuses: vec![],
            player: Player::God,
        }
    }

    pub fn owned_by(&self, player: &Player) -> bool {
        player == &self.player
    }

    pub fn get_display_pos(&self, time: f32) -> ScreenCoord {
        match &self.animation {
            Some(animation) => animation.get_pos(time),
            None => ScreenCoord::from_hexcoord(&self.coord),
        }
    }

    pub fn get_screen_coord(&self) -> ScreenCoord {
        ScreenCoord::from_hexcoord(&self.coord)
    }

    pub fn set_coord(&mut self, coord: &HexCoord) {
        self.coord = *coord;
    }

    pub fn add_status(&mut self, status: Status) {
        self.statuses.push(status);
    }

    pub fn remove_status(&mut self, status: Status) {
        self.statuses.retain(|&s| s != status);
    }
}