use crate::engine::{
    objects::ObjectProps,
    statuses::{Effect, Status, StatusType},
    ObjectType, Player,
};
use crate::geometry::{HexCoord, ScreenCoord};

#[derive(Clone, Debug)]
pub struct Object {
    pub otype: ObjectType,
    pub coord: HexCoord,
    pub props: ObjectProps,
    pub statuses: Vec<Status>,
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
            statuses: vec![],
            player,
        }
    }

    pub fn new_tile(oid: usize, coord: HexCoord, lifespan: usize) -> Object {
        let mut tile = Object {
            otype: ObjectType::Tile,
            coord,
            props: ObjectProps {
                oid,
                size: 1.1,
                selectable: false,
                draggable: false,
                ..Default::default()
            },
            statuses: vec![],
            player: Player::God,
        };
        tile.add_status(&Status::new_delayed_effect_with_indicator(
            lifespan,
            Effect::KillAallOn {
                coord,
                apply: Some(Box::new(StatusType::Falling)),
                duration: Some(2.0),
            },
            lifespan - 2,
            Effect::SetStatus {
                object: tile.clone(),
                stype: Box::new(StatusType::Wobble {
                    amplitude: 0.2,
                    speed: 37.1,
                }),
                duration: None,
            },
        ));
        tile
    }

    pub fn new_wall(oid: usize, coord: HexCoord) -> Object {
        Object {
            otype: ObjectType::Wall,
            coord,
            props: ObjectProps {
                oid,
                size: 1.1,
                selectable: false,
                draggable: false,
                ..Default::default()
            },
            statuses: vec![],
            player: Player::God,
        }
    }

    /**
    run any updates the objects need to do on move apply, and applies any
    effects that may result in.

    currently:
    - look for `StatusType::DelayedEffect` and trigger them and their indicators.
    */
    pub fn tick(&mut self, curr_move_nr: usize, _time: f32) -> Vec<Effect> {
        let mut effects = vec![];
        // TODO: can we make this lookup prettier?
        if let Some(delayed) = self.get_status(&Status::new_delayed_effect(0, Effect::NoOp).stype) {
            if let StatusType::DelayedEffect {
                move_nr,
                effect,
                indicator_move_nr,
                indicator,
            } = delayed.stype.clone()
            {
                if indicator_move_nr.unwrap_or(usize::MAX) == curr_move_nr {
                    effects.push(indicator.unwrap_or_else(|| {
                        panic!(
                            "no indicator for delayed effect with indicator_move_nr set: {:?}",
                            delayed
                        )
                    }));
                }

                if move_nr == curr_move_nr {
                    self.remove_status(&delayed.stype.clone());
                    effects.push(effect);
                }
            }
        }
        effects
    }

    pub fn is_tile(&self) -> bool {
        self.otype == ObjectType::Tile
    }

    pub fn owned_by(&self, player: &Player) -> bool {
        player == &self.player
    }

    pub fn get_screen_coord(&self) -> ScreenCoord {
        ScreenCoord::from_hexcoord(&self.coord)
    }

    pub fn set_coord(&mut self, coord: &HexCoord) {
        self.coord = *coord;
    }

    pub fn add_status(&mut self, status: &Status) {
        if let Some(s) = self.statuses.iter().find(|s| s.stype == status.stype) {
            panic!(
                "dev warnng: same status added twice! (existing={:?} new={:?})",
                s, status
            );
        }
        self.statuses.push(status.clone());
    }
    pub fn add_statuses(&mut self, statuses: Vec<&Status>) {
        statuses.iter().for_each(|s| self.add_status(s))
    }

    pub fn remove_status(&mut self, _stype: &StatusType) {
        self.statuses.retain(|s| !matches!(&s.stype, _stype));
    }

    pub fn set_killed(&mut self, status: Option<&Status>) {
        self.props.dead = true;
        if let Some(s) = status {
            self.add_status(s);
        }
    }

    /**
    gets the actual status (with correct params)
    from a status with any (e.g. default) params
    */
    fn get_status(&self, _stype: &StatusType) -> Option<&Status> {
        self.statuses.iter().find(|s| matches!(&s.stype, _stype))
    }
}
