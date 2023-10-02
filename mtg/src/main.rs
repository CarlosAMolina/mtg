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
    let mut graveyard_user: Vec<Box<dyn Card>> = vec![];
    let mut graveyard_cpu: Vec<Box<dyn Card>> = vec![];
    let creatures_user = vec![creature_1];
    let creatures_cpu = vec![creature_2];
    let user = Player::new("Carlos");
    let cpu = Player::new("CPU");
    log::info!("Init game. {} vs {}", user.name, cpu.name);
    loop {
        let player = user.clone();
        log::info!("Turn of the player {}", player.name);
        log::info!("Init combat");
        log::info!("Do you want to attack? [y/N]");
        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");
        answer = answer.trim().to_string();
        let is_player_attacking = answer.to_lowercase() == "y".to_string();
        // TODO if !is_player_attacking {
        // TODO rm
        if is_player_attacking {
            continue;
        }
        let attacker = creatures_user[0].clone();
        let blocker = creatures_cpu[0].clone();
        log::info!("{} vs {}", attacker, blocker);
        let new_attacker = get_creature_after_combat(&blocker, &attacker);
        let new_blocker = get_creature_after_combat(&attacker, &blocker);
        log::debug!("Result: {} and {}", new_attacker, new_blocker);
        if new_attacker.toughness <= 0 {
            graveyard_user.push(Box::new(attacker));
        }
        if new_blocker.toughness <= 0 {
            graveyard_cpu.push(Box::new(blocker));
        }
        log::debug!(
            "Graveyard {} ({}): {:?}",
            player.name,
            graveyard_user.len(),
            graveyard_user
        );
        log::debug!(
            "Graveyard {} ({}): {:?}",
            cpu.name,
            graveyard_cpu.len(),
            graveyard_cpu
        );
    }
}

use core::fmt::Debug;
impl Debug for dyn Card {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.name())
    }
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
        Creature {
            name,
            power,
            toughness,
        }
    }
}

#[derive(Clone)]
struct Player {
    name: &'static str,
}

impl Player {
    fn new(name: &'static str) -> Self {
        Player { name }
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

fn get_creature_after_combat(attacker: &Creature, blocker: &Creature) -> Creature {
    let new_toughness = blocker.toughness - attacker.power;
    Creature {
        name: blocker.name.clone(),
        power: blocker.power,
        toughness: new_toughness,
    }
}
