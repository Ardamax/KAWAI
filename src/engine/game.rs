use crate::engine::map::GameMap;
use crate::engine::combat_units::Unit;
use std::collections::HashMap;
use crate::engine::map::Position;

/// A struct to model a current game-state
pub struct Game {
    pub map: GameMap,
    pub unit_list: Vec<Unit>,
    pub occupying_team: HashMap<Position, u8>,
}

impl Game {
}