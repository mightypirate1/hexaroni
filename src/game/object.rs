use super::ObjectType;


#[derive(Copy, Clone, Debug)]
pub struct Object {
    pub otype: ObjectType,
}

impl Object {
    pub fn new(otype: ObjectType) -> Object {
        Object {otype}
    }
}