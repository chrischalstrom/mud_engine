use crate::combat::Attackable;

pub struct Monster {
    pub hp: i32,
    pub atk: i32,
    pub def: i32,
    pub name: String,
}

impl Attackable for Monster {
    fn attack(&self, other: &mut Attackable) -> i32 {
        other.take_damage(self.atk);
        self.atk
    }

    fn take_damage(&mut self, damage: i32) {
        self.hp -= damage;
    }
}