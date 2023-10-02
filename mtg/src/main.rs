use env_logger::{Builder, Env, Target};
use std::fmt;
use std::io;

use log;

fn main() {
    Builder::from_env(Env::default().default_filter_or("debug"))
        .target(Target::Stdout)
        .init();
    let creature_1 = Creature::new("Rat", 2, 2);
    let creature_2 = Creature::new("Small Rat", 2, 1);
    let user = Player::<Card>::new("Carlos");
    let cpu = Player::new("CPU");
    log::info!("Init game");
    //loop {
    //    let player = user.clone();
    //    log::info!("Turn of the player {}", player.name);
    //    log::info!("Init combat");
    //    log::info!("Do you want to attack? [y/N]");
    //    let mut answer = String::new();
    //    io::stdin()
    //        .read_line(&mut answer)
    //        .expect("Failed to read line");
    //    answer = answer.trim().to_string();
    //    let is_player_attacking = answer.to_lowercase() == "y".to_string();
    //    if !is_player_attacking {
    //        continue;
    //    }
    //    let attacker = user.creatures[0].clone();
    //    let blocker = cpu.creatures[0].clone();
    //    log::info!("{} vs {}", attacker, blocker);
    //    let new_attacker = get_creature_after_combat(&blocker, &attacker);
    //    let new_blocker = get_creature_after_combat(&attacker, &blocker);
    //    log::debug!("Result: {} and {}", new_attacker, new_blocker);
    //}
}


#[derive(Clone, Debug)]
struct Creature {
    name: &'static str,
    power: i8,
    toughness: i8,
}

trait Card {
    fn name(&self) -> &'static str;
}

impl Creature {
    fn new(name: &'static str, power: i8, toughness: i8) -> Self {
        Creature { name, power, toughness}
    }
}

impl Card for Creature {
    fn name(&self) -> &'static str {
        self.name
    }
}

impl fmt::Display for Creature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}({}, {})", self.name, self.power, self.toughness)
    }
}

#[derive(Clone)]
struct Player<T: Card + Sized>{
    name: &'static str,
    creatures: Vec<Creature>,
    graveyard: Vec<T>,
}

impl<T: Card + Sized> Player<T> {
    fn new(name: &'static str) -> Player<T> {
        Player { name, creatures: Vec::new(), graveyard: Vec::new() }
    }

    fn add_creature(&self, creature: Creature) {
        self.creatures.push(creature);
    }

    fn add_graveyard(&self, card: T) {
        self.graveyard.push(card);
    }

}

fn get_creature_after_combat(attacker: &Creature, blocker: &Creature) -> Creature {
    let new_toughness = blocker.toughness - attacker.power;
    Creature {
        name: blocker.name.clone(),
        power: blocker.power,
        toughness: new_toughness,
    }
}
