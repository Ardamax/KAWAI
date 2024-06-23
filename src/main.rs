use crate::engine::combat_units;
use crate::engine::map::GameMap;
use crate::engine::tileset;
use std::time::Instant;

pub mod engine;

fn main() {
    let test_map = GameMap::new(8, 8, vec![0; 64]).unwrap();
    let now = Instant::now();
    {
        for _ in 0..1000000 {
            for i in 0..8 {
                for j in 0..8 {
                    test_map.get_adjacent([i, j]);
                }
            }
        }
    }
    let elapsed = now.elapsed();
    println!("get_adjacent: {:.2?}", elapsed / 64000000);
}
