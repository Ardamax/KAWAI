use crate::engine::{game::Game, map::Position, unitset::UnitType};
use std::cmp::min;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::hash::{Hash, Hasher};

pub struct Unit {
    pub position: [u8; 2],
    pub ammo: u8,
    pub fuel: u8,
    pub health: u8,
    pub team: u8,
    unit_type: UnitType,
}

impl Hash for Unit {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.position[0] * std::u8::MAX + self.position[1]).hash(state)
    }
}

impl Unit {
    /// Implements a BFS for figuring out what tiles are accessible for a unit
    pub fn get_moveable(&self, game: &Game) -> HashSet<Position> {
        let movetype = &self.unit_type.movetype();
        let mobility = min(self.unit_type.data().moves(), self.fuel);
        // The queue must only be as big as the 2nd largest ring, which is 4 * (mobility -1)
        let mut queue: VecDeque<(Position, u8)> =
            VecDeque::with_capacity(min(4 * (mobility - 1), 1) as usize);
        let mut moveable: HashSet<Position> = HashSet::from([self.position]);
        let mut not_moveable: HashSet<Position> = HashSet::new();

        let map = &game.map;
        let occupying_team = &game.occupying_team;
        let team = occupying_team.get(&self.position).unwrap();
        queue.push_back((self.position, mobility));
        while !queue.is_empty() {
            let (position, mobility) = queue.pop_front().unwrap();
            for position in map.get_adjacent(position) {
                // Don't recheck tiles
                if not_moveable.contains(&position) || moveable.contains(&position) {
                    continue;
                }

                // Now check if you can move onto the tile
                let tile = map.terrain_at(position);
                let cost = tile.move_cost(&movetype);
                if cost == std::u8::MAX {
                    continue;
                }
                let remaining_mobility = mobility - cost;

                let occupant = occupying_team.get(&position);
                // Check if the tile is occupied by an enemy team
                match (0.cmp(&remaining_mobility), occupant) {
                    (Ordering::Equal, None) => {
                        moveable.insert(position);
                    }
                    (Ordering::Greater, None) => {
                        queue.push_back((position, remaining_mobility));
                        moveable.insert(position);
                    }
                    (Ordering::Greater, Some(occupant)) => {
                        if occupant == team {
                            queue.push_back((position, remaining_mobility));
                        }
                        not_moveable.insert(position);
                    }
                    _ => {
                        not_moveable.insert(position);
                    }
                }
            }
        }
        moveable
    }
}
