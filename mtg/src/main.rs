use core::fmt::Debug;
use env_logger::{Builder, Env, Target};
use std::fmt;
use std::io;

use log;

fn main() {
    Builder::from_env(Env::default().default_filter_or("debug"))
        .target(Target::Stdout)
        .init();
    let user = Player::new("Carlos");
    let cpu = Player::new("CPU");
    let mut cards_user: Vec<&dyn Card> = vec![];
    let mut cards_cpu: Vec<&dyn Card> = vec![];
    let mut creatures_hand_user: Vec<&Creature> = vec![];
    let mut creatures_hand_cpu: Vec<&Creature> = vec![];
    let mut creatures_battlefield_user: Vec<&Creature> = vec![];
    let mut creatures_battlefield_cpu: Vec<&Creature> = vec![];
    let mut graveyard_user: Vec<&dyn Card> = vec![];
    let mut graveyard_cpu: Vec<&dyn Card> = vec![];
    let creature_1 = Creature::new(CardId(1), "Rat", 2, 2);
    let creature_2 = Creature::new(CardId(2), "Small Rat", 2, 1);
    cards_user.push(&creature_1);
    cards_cpu.push(&creature_2);
    creatures_hand_user.push(&creature_1);
    creatures_hand_cpu.push(&creature_2);
    creatures_battlefield_user.push(&creature_1);
    creatures_battlefield_cpu.push(&creature_2);
    log::info!("Init game. {} vs {}", user.name, cpu.name);
    let mut turn_count = 0;
    loop {
        log::debug!("Init turn {}", turn_count);
        let player = user.clone();
        log::info!("Turn of the player {}", player.name);
        log::info!("Init combat");
        log::info!("Do you want to attack? [y/N]");
        // TODO let mut answer = String::new();
        // TODO io::stdin()
        // TODO     .read_line(&mut answer)
        // TODO     .expect("Failed to read line");
        // TODO answer = answer.trim().to_string();
        // TODO let is_player_attacking = answer.to_lowercase() == "y".to_string();
        // TODO if !is_player_attacking {
        // TODO    continue;
        // TODO}
        let attacker = creatures_battlefield_user[0];
        let blocker = creatures_battlefield_cpu[0];
        log::info!("{} vs {}", attacker, blocker);
        let new_attacker = get_creature_after_combat(blocker, attacker);
        let new_blocker = get_creature_after_combat(attacker, blocker);
        log::debug!("Result attacker: {} -> {}", attacker, new_attacker);
        log::debug!("Result blocker: {} -> {}", blocker, new_blocker);
        if new_attacker.toughness <= 0 {
            let index = creatures_battlefield_user
                .iter()
                .position(|x| x.id == attacker.id)
                .unwrap();
            creatures_battlefield_user.remove(index);
            graveyard_user.push(attacker);
        }
        if new_blocker.toughness <= 0 {
            let index = creatures_battlefield_cpu
                .iter()
                .position(|x| x.id == blocker.id)
                .unwrap();
            creatures_battlefield_cpu.remove(index);
            graveyard_cpu.push(blocker);
        }
        log::debug!(
            "Battlefield {} ({}): {:?}",
            player.name,
            creatures_battlefield_user.len(),
            creatures_battlefield_user
        );
        log::debug!(
            "Battlefield {} ({}): {:?}",
            cpu.name,
            creatures_battlefield_cpu.len(),
            creatures_battlefield_cpu
        );
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
        turn_count += 1;
        if user.life <= 0 {
            log::info!("User {} wins!", user.name);
            break;
        }
        if cpu.life <= 0 {
            log::info!("User {} wins!", cpu.name);
            break;
        }
    }
}

#[derive(Clone)]
struct Player {
    life: u8,
    name: &'static str,
}

impl Player {
    fn new(name: &'static str) -> Self {
        Player { life: 20, name }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct CardId(i32);

trait Card {
    fn id(&self) -> CardId;
    fn name(&self) -> &'static str;
}

impl Debug for dyn Card {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}, name={}", self.id(), self.name())
    }
}

#[derive(Clone, Debug)]
struct Creature {
    id: CardId,
    name: &'static str,
    power: i8,
    toughness: i8,
}

impl Creature {
    fn new(id: CardId, name: &'static str, power: i8, toughness: i8) -> Self {
        Creature {
            id,
            name,
            power,
            toughness,
        }
    }
}

impl Card for Creature {
    fn id(&self) -> CardId {
        self.id
    }

    fn name(&self) -> &'static str {
        self.name
    }
}

impl fmt::Display for Creature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?} {}({}, {})",
            self.id, self.name, self.power, self.toughness
        )
    }
}

fn get_creature_after_combat(attacker: &Creature, blocker: &Creature) -> Creature {
    let new_toughness = blocker.toughness - attacker.power;
    Creature {
        id: blocker.id.clone(),
        name: blocker.name.clone(),
        power: blocker.power,
        toughness: new_toughness,
    }
}
