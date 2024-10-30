#[derive(Clone, Copy, Debug)]
pub struct ObjectProps {
    pub oid: usize,
    pub selectable: bool,
    pub draggable: bool,
    pub dead: bool,
}


impl Default for ObjectProps {
    fn default() -> ObjectProps {
        ObjectProps {
            oid: 0,
            selectable: true,
            draggable: true,
            dead: false,
        }
    }
}


impl ObjectProps {
    pub fn new(oid: usize, selectable: bool, draggable: bool, dead: bool) -> ObjectProps {
        ObjectProps {
            oid,
            selectable,
            draggable,
            dead,
        }
    }
}