use crate::combat::Attackable;

// TODO think of way to handle equipment
// - player command prompt showing player health
pub struct Player {
    pub hp: i32,
    pub atk: i32,
    pub def: i32,
    pub speed: i32,
    pub name: String,
}

impl Attackable for Player {
    fn attack(&self, other: &mut Attackable) -> i32 {
        other.take_damage(self.atk);
        self.atk
        // player should get experience if killing monster?
        // or maybe experience is distributed to all in the room if monster dies?
        // an attack has multiple effects
        // - target takes damage
        // - print to stdout
        // - other?
        // so, need a way to manage event dispatch
    }

    fn take_damage(&mut self, damage: i32) {
        let hp = self.hp - damage;
        self.hp = if hp < 0 { 0 } else { hp };
    }
}
