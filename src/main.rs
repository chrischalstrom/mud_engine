use std::sync::mpsc;
use std::thread;
use std::time;

use mud_host::combat::Attackable;
use mud_host::command_processor;
use mud_host::monster::*;
use mud_host::player::*;
use mud_host::stdin_processor;
use mud_host::world;

type GameUpdates = Vec<String>;

fn main() {
    let (input_rx, mut monsters, mut player) = init();

    loop {
        process_input(&input_rx);
        let game_updates = tick(&mut player, &mut monsters);
        output_state(game_updates);
    }
}

fn init() -> (mpsc::Receiver<String>, Vec<Monster>, Player) {
    // set up player input channel
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let processor = stdin_processor::StdinProcessor {};
        command_processor::listen(&processor, tx);
    });

    // set up world
    let monsters = world::gen_monsters();
    let player = Player {
        hp: 130,
        atk: 21,
        def: 19,
        speed: 22,
        name: String::from("Dude"),
    };

    (rx, monsters, player)
}

fn process_input(rx: &mpsc::Receiver<String>) {
    for msg in rx.try_iter() {
        println!("recv'd msg: {}", msg);
    }
}

// This probably returns a queue of updates
fn tick(player: &mut Player, monsters: &mut Vec<Monster>) -> GameUpdates {
    let mut game_updates = Vec::new();
    game_updates.push(format!("> {}: hp={}", player.name, player.hp));

    match monsters.get_mut(0) {
        Some(monster) => {
            let dmg = player.attack(monster);
            game_updates.push(format!(
                "> {} hit {} for {}.",
                player.name, monster.name, dmg
            ));
        }
        None => println!("all monsters gone"),
    };

    for monster in monsters.iter() {
        let dmg = monster.attack(player);
        game_updates.push(format!(
            "> {} hit {} for {}.",
            monster.name, player.name, dmg
        ));
    }

    thread::sleep(time::Duration::from_millis(1000));

    game_updates
}

fn output_state(game_updates: GameUpdates) {
    for update in game_updates {
        println!("{}", update);
    }
}
