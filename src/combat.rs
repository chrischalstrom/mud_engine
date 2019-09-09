// It might make sense to have some of these stats on a field to better encapsulate it?
// at least the combat rules themselves.  As for e.g. hp, it probably doesn't make sense
// since other factors influence that and it should be independent from specifically a combat module.
// it makes sense to glob together all the logic for e.g. combat strikes though i think.

/*
pub struct Attackable {
    pub hp: i32,
    pub atk: i32,
    pub def: i32,
    pub name: String,
}

impl Attackable {
    pub fn attack(&self, other: &mut Attackable) -> bool {
        other.hp -= self.atk;
        other.hp <= 0
    }
}
*/

pub trait Attackable {
    fn attack(&self, other: &mut Attackable) -> i32;
    // fn attack(&self, other: &mut Box<dyn Attackable>);
    fn take_damage(&mut self, damage: i32);
}