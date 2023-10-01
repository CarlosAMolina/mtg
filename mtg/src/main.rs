use env_logger::{Builder, Env, Target};
use std::fmt;

use log;

fn main() {
    Builder::from_env(Env::default().default_filter_or("debug"))
        .target(Target::Stdout)
        .init();
    let creature_1 = Creature {
        name: "Rat".to_string(),
        power: 2,
        toughness: 2,
    };
    let creature_2 = Creature {
        name: "Small Rat".to_string(),
        power: 2,
        toughness: 1,
    };
    let user = Player {
        name: "Carlos".to_string(),
        creatures: vec![creature_1],
    };
    let cpu = Player {
        name: "Antonio".to_string(),
        creatures: vec![creature_2],
    };
    log::info!("Turn of the player {}", user.name);
    log::info!("Init combat");
    let attacker = user.creatures[0].clone();
    let blocker = cpu.creatures[0].clone();
    log::info!("{} vs {}", attacker, blocker);
    let new_attacker = get_creature_after_combat(&blocker, &attacker);
    let new_blocker = get_creature_after_combat(&attacker, &blocker);
    log::debug!("Result: {} and {}", new_attacker, new_blocker);
}

#[derive(Clone, Debug)]
struct Creature {
    name: String,
    power: i8,
    toughness: i8,
}

#[derive(Debug)]
struct Player {
    name: String,
    creatures: Vec<Creature>,
}

impl fmt::Display for Creature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}({}, {})", self.name, self.power, self.toughness)
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
