use super::{map::GameMap, map::Position, unitset::UnitType};
use std::cmp::min;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::collections::VecDeque;

pub struct Unit {
    pub position: [u8; 2],
    pub ammo: u8,
    pub fuel: u8,
    pub health: u8,
    unit_type: UnitType,
}

impl Unit {
    /// Implements a BFS for figuring out what tiles are accessible for a unit
    pub fn get_moveable(&self, map: &GameMap) -> HashSet<Position> {
        let unit_type = &self.unit_type;
        let movetype = unit_type.movetype();
        let mobility = min(unit_type.data().moves(), self.fuel);
        // The queue must only be as big as the 2nd largest ring, which is 4 * (mobility -1)
        let mut queue: VecDeque<(Position, u8)> =
            VecDeque::with_capacity(min(4 * (mobility - 1), 1) as usize);
        let mut visited: HashSet<Position> = HashSet::from([self.position]);

        queue.push_back((self.position, mobility));
        while !queue.is_empty() {
            let (position, mobility) = queue.pop_front().unwrap();
            for position in map.get_adjacent(position) {
                if visited.contains(&position) {
                    continue;
                }
                visited.insert(position);
                let tile = map.terrain_at(position);
                let remaining_mobility = mobility - tile.move_cost(&movetype);
                match 0.cmp(&remaining_mobility) {
                    Ordering::Less => {}
                    Ordering::Equal => {
                        visited.insert(position);
                    }
                    Ordering::Greater => queue.push_back((position, remaining_mobility)),
                }
            }
        }
        return visited;
    }
}
