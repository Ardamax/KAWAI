use entities::entity_data::TileData;

pub struct Tile {
    pub name: &'static str,
    pub tile_data: TileData,
    move_data: MoveCost
}

impl Tile {
    pub fn move_cost(&self, movetype: MoveType) -> u8 {
        self.move_data.cost(movetype)
    }
}

pub enum MoveType {
    Air,
    Boot,
    Foot,
    Lander,
    Pipe,
    Sea,
    Tires,
    Treads,
}

struct MoveCost {
    air_cost: u8,
    boot_cost: u8,
    foot_cost: u8,
    lander_cost: u8,
    pipe_cost: u8,
    sea_cost: u8,
    tires_cost: u8,
    treads_cost: u8,
}

impl MoveCost {
    pub fn cost(&self, movetype: MoveType) -> u8 {
        match movetype {
            MoveType::Air => self.air_cost,
            MoveType::Boot => self.boot_cost,
            MoveType::Foot => self.foot_cost,
            MoveType::Lander => self.lander_cost,
            MoveType::Pipe => self.pipe_cost,
            MoveType::Sea => self.sea_cost,
            MoveType::Tires => self.tires_cost,
            MoveType::Treads => self.treads_cost,
       }
    }
}
const PLAINS: Tile = Tile {
    name: "Plains",
    move_data: MoveCost {
        air_cost : 1,
        boot_cost : 1,
        foot_cost : 1,
        lander_cost : 0,
        pipe_cost : 0,
        sea_cost : 0,
        tires_cost : 2,
        treads_cost : 1,
    },
    tile_data: TileData::define_tile( 0, 0, 0, 0, 0, 0, 1,)
};
const WOODS: Tile = Tile {
    name: "Woods",
    move_data: MoveCost {
        air_cost : 1,
        boot_cost : 1,
        foot_cost : 1,
        lander_cost : 0,
        pipe_cost : 0,
        sea_cost : 0,
        tires_cost : 3,
        treads_cost : 2,
    },
    tile_data: TileData::define_tile( 0, 0, 0, 1, 0, 0, 2,)
};
const MOUNTAINS: Tile = Tile {
    name: "Mountains",
    move_data: MoveCost {
        air_cost : 1,
        boot_cost : 1,
        foot_cost : 2,
        lander_cost : 0,
        pipe_cost : 0,
        sea_cost : 0,
        tires_cost : 0,
        treads_cost : 0,
    },
    tile_data: TileData::define_tile( 0, 0, 0, 0, 1, 0, 4,)
};
const ROADS: Tile = Tile {
    name: "Roads",
    move_data: MoveCost {
        air_cost : 1,
        boot_cost : 1,
        foot_cost : 1,
        lander_cost : 0,
        pipe_cost : 0,
        sea_cost : 0,
        tires_cost : 1,
        treads_cost : 1,
    },
    tile_data: TileData::define_tile( 0, 0, 0, 0, 0, 0, 0,)
};
const RIVERS: Tile = Tile {
    name: "Rivers",
    move_data: MoveCost {
        air_cost : 1,
        boot_cost : 1,
        foot_cost : 2,
        lander_cost : 0,
        pipe_cost : 0,
        sea_cost : 0,
        tires_cost : 0,
        treads_cost : 0,
    },
    tile_data: TileData::define_tile( 0, 0, 0, 0, 0, 0, 0,)
};
const PIPES: Tile = Tile {
    name: "Pipes",
    move_data: MoveCost {
        air_cost : 0,
        boot_cost : 0,
        foot_cost : 0,
        lander_cost : 0,
        pipe_cost : 1,
        sea_cost : 0,
        tires_cost : 0,
        treads_cost : 0,
    },
    tile_data: TileData::define_tile( 0, 0, 0, 0, 0, 0, 0,)
};
const PIPE_SEAM: Tile = Tile {
    name: "Pipe Seam",
    move_data: MoveCost {
        air_cost : 0,
        boot_cost : 0,
        foot_cost : 0,
        lander_cost : 0,
        pipe_cost : 1,
        sea_cost : 0,
        tires_cost : 0,
        treads_cost : 0,
    },
    tile_data: TileData::define_tile( 0, 0, 1, 0, 0, 0, 0,)
};
const BROKEN_PIPES: Tile = Tile {
    name: "Broken Pipes",
    move_data: MoveCost {
        air_cost : 1,
        boot_cost : 1,
        foot_cost : 1,
        lander_cost : 0,
        pipe_cost : 0,
        sea_cost : 0,
        tires_cost : 2,
        treads_cost : 1,
    },
    tile_data: TileData::define_tile( 0, 0, 0, 0, 0, 0, 1,)
};
const LOADED_MISSILE_SILO: Tile = Tile {
    name: "Loaded Missile Silo",
    move_data: MoveCost {
        air_cost : 1,
        boot_cost : 1,
        foot_cost : 1,
        lander_cost : 0,
        pipe_cost : 0,
        sea_cost : 0,
        tires_cost : 1,
        treads_cost : 1,
    },
    tile_data: TileData::define_tile( 0, 0, 0, 0, 0, 0, 3,)
};
const UNLOADED_MISSILE_SILO: Tile = Tile {
    name: "Unloaded Missile Silo",
    move_data: MoveCost {
        air_cost : 1,
        boot_cost : 1,
        foot_cost : 1,
        lander_cost : 0,
        pipe_cost : 0,
        sea_cost : 0,
        tires_cost : 1,
        treads_cost : 1,
    },
    tile_data: TileData::define_tile( 0, 0, 0, 0, 0, 0, 3,)
};
const SEA: Tile = Tile {
    name: "Sea",
    move_data: MoveCost {
        air_cost : 1,
        boot_cost : 0,
        foot_cost : 0,
        lander_cost : 1,
        pipe_cost : 0,
        sea_cost : 1,
        tires_cost : 0,
        treads_cost : 0,
    },
    tile_data: TileData::define_tile( 0, 0, 0, 0, 0, 0, 0,)
};
const REEFS: Tile = Tile {
    name: "Reefs",
    move_data: MoveCost {
        air_cost : 1,
        boot_cost : 0,
        foot_cost : 0,
        lander_cost : 2,
        pipe_cost : 0,
        sea_cost : 2,
        tires_cost : 0,
        treads_cost : 0,
    },
    tile_data: TileData::define_tile( 0, 0, 0, 1, 0, 0, 1,)
};
const SHOALS: Tile = Tile {
    name: "Shoals",
    move_data: MoveCost {
        air_cost : 1,
        boot_cost : 1,
        foot_cost : 1,
        lander_cost : 1,
        pipe_cost : 0,
        sea_cost : 0,
        tires_cost : 1,
        treads_cost : 1,
    },
    tile_data: TileData::define_tile( 0, 0, 0, 0, 0, 0, 0,)
};
const HEADQUARTERS: Tile = Tile {
    name: "Headquarters",
    move_data: MoveCost {
        air_cost : 1,
        boot_cost : 1,
        foot_cost : 1,
        lander_cost : 0,
        pipe_cost : 0,
        sea_cost : 0,
        tires_cost : 1,
        treads_cost : 1,
    },
    tile_data: TileData::define_tile( 1, 1, 0, 0, 0, 0, 4,)
};
const CITY: Tile = Tile {
    name: "City",
    move_data: MoveCost {
        air_cost : 1,
        boot_cost : 1,
        foot_cost : 1,
        lander_cost : 0,
        pipe_cost : 0,
        sea_cost : 0,
        tires_cost : 1,
        treads_cost : 1,
    },
    tile_data: TileData::define_tile( 1, 1, 0, 0, 0, 0, 3,)
};
const BASE: Tile = Tile {
    name: "Base",
    move_data: MoveCost {
        air_cost : 1,
        boot_cost : 1,
        foot_cost : 1,
        lander_cost : 0,
        pipe_cost : 1,
        sea_cost : 0,
        tires_cost : 1,
        treads_cost : 1,
    },
    tile_data: TileData::define_tile( 1, 1, 0, 0, 0, 0, 3,)
};
const AIRPORT: Tile = Tile {
    name: "Airport",
    move_data: MoveCost {
        air_cost : 1,
        boot_cost : 1,
        foot_cost : 1,
        lander_cost : 0,
        pipe_cost : 0,
        sea_cost : 0,
        tires_cost : 1,
        treads_cost : 1,
    },
    tile_data: TileData::define_tile( 1, 1, 0, 0, 0, 0, 3,)
};
const PORT: Tile = Tile {
    name: "Port",
    move_data: MoveCost {
        air_cost : 1,
        boot_cost : 1,
        foot_cost : 1,
        lander_cost : 1,
        pipe_cost : 0,
        sea_cost : 1,
        tires_cost : 1,
        treads_cost : 1,
    },
    tile_data: TileData::define_tile( 1, 1, 0, 0, 0, 0, 3,)
};
const LAB: Tile = Tile {
    name: "Lab",
    move_data: MoveCost {
        air_cost : 1,
        boot_cost : 1,
        foot_cost : 1,
        lander_cost : 0,
        pipe_cost : 0,
        sea_cost : 0,
        tires_cost : 1,
        treads_cost : 1,
    },
    tile_data: TileData::define_tile( 1, 0, 0, 0, 0, 0, 3,)
};
const COMM_TOWER: Tile = Tile {
    name: "Comm Tower",
    move_data: MoveCost {
        air_cost : 1,
        boot_cost : 1,
        foot_cost : 1,
        lander_cost : 0,
        pipe_cost : 0,
        sea_cost : 0,
        tires_cost : 1,
        treads_cost : 1,
    },
    tile_data: TileData::define_tile( 1, 0, 0, 0, 0, 0, 3,)
};
const TELEPORT: Tile = Tile {
    name: "Teleport",
    move_data: MoveCost {
        air_cost : 0,
        boot_cost : 0,
        foot_cost : 0,
        lander_cost : 0,
        pipe_cost : 0,
        sea_cost : 0,
        tires_cost : 0,
        treads_cost : 0,
    },
    tile_data: TileData::define_tile( 0, 0, 0, 0, 0, 1, 0,)
};
use::std::error::Error;
pub fn get_tile(id:u8) -> Result<Tile, Box<dyn Error>> {
    let result = match id {
        1 => PLAINS,
        3 => WOODS,
        2 => MOUNTAINS,
        15..=27 => ROADS,
        4..=14 => RIVERS,
        101..=110 => PIPES,
        113..=114 => PIPE_SEAM,
        115..=116 => BROKEN_PIPES,
        111 => LOADED_MISSILE_SILO,
        112 => UNLOADED_MISSILE_SILO,
        28 => SEA,
        33 => REEFS,
        29..=32 => SHOALS,
        0 => HEADQUARTERS,
        34 => CITY,
        35 => BASE,
        36 => AIRPORT,
        37 => PORT,
        145 => LAB,
        133 => COMM_TOWER,
        195 => TELEPORT,
        _ => return Err(format!("Invalid Tile ID {}", id))?
    };
Ok(result)
}
