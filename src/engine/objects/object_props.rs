#[derive(Clone, Copy, Debug)]
pub struct ObjectProps {
    pub oid: usize,
    pub selectable: bool,
    pub draggable: bool,
    pub dead: bool,
    pub size: f32,
}

impl Default for ObjectProps {
    fn default() -> ObjectProps {
        ObjectProps {
            oid: 0,
            selectable: true,
            draggable: true,
            dead: false,
            size: 1.0,
        }
    }
}

impl ObjectProps {
    pub fn new(
        oid: usize,
        selectable: bool,
        draggable: bool,
        dead: bool,
        size: f32,
    ) -> ObjectProps {
        ObjectProps {
            oid,
            selectable,
            draggable,
            dead,
            size,
        }
    }
}
