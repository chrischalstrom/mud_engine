use crate::monster::Monster;

pub fn gen_monsters() -> Vec<Monster> {
    vec![
        Monster {
            hp: 16,
            atk: 8,
            def: 8,
            name: String::from("rat")
        },
        Monster {
            hp: 16,
            atk: 8,
            def: 8,
            name: String::from("squirrel")
        },
        Monster {
            hp: 16,
            atk: 8,
            def: 8,
            name: String::from("turkey")
        },
    ]
}