use crate::tileset;
use std::error::Error;
use std::fmt;

use super::tileset::Tile;

pub type Position = [u8; 2];

/// Represents an Advance Wars map of maximum size 256x256 tiles
#[derive(Debug)]
pub struct GameMap {
    pub width: u8,
    pub height: u8,
    terrain: Box<[u8]>,
}

pub enum Weather {
    None,
    Rain,
    Snow,
    Sandstorm,
}

impl GameMap {
    pub fn new(width: u8, height: u8, terrain: Vec<u8>) -> Result<GameMap, MapGenError> {
        if terrain.len() != (width * height).into() {
            return Err(MapGenError {
                width,
                height,
                vec_size: terrain.len(),
            });
        }
        Ok(GameMap {
            width,
            height,
            terrain: terrain.into_boxed_slice(),
        })
    }

    pub fn terrain_at(&self, position: Position) -> Tile {
        let (x, y) = (position[0] as usize, position[1] as usize);
        if self.width as usize * y + x >= self.terrain.len() {
            panic!("Attempting to access nonexistant terrain")
        }
        tileset::get_tile(self.terrain[self.width as usize * y + x]).unwrap()
    }

    pub fn get_adjacent(&self, position: Position) -> Vec<[u8; 2]> {
        let [x, y] = position;
        let (width, height) = (self.width, self.height);
        let check = |x: u8, y: u8| -> Option<[u8; 2]> {
            if (0..=width).contains(&x) && (0..=height).contains(&y) {
                return Some([x, y]);
            }
            return None;
        };

        vec![
            check(x + 1, y),
            check(x - 1, y),
            check(x, y + 1),
            check(x, y - 1),
        ]
        .iter()
        .filter_map(|pos| *pos)
        .collect()
    }
}

#[derive(Debug, Clone)]
pub struct MapGenError {
    width: u8,
    height: u8,
    vec_size: usize,
}

impl Error for MapGenError {}

impl fmt::Display for MapGenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Received {} x {} map expecting {} tiles but only received {} tiles",
            self.width,
            self.height,
            self.width * self.height,
            self.vec_size
        )
    }
}
