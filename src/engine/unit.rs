use entities::entity_data::UnitData;
use tileset::MoveType;

pub struct Unit {
    pub name: &'static str,
    pub unit_data: UnitData,
    move_type: MoveType
}

const ANTI_AIR: Unit = Unit {
    name: "Anti-Air",
    move_type: MoveType::Treads,
    unit_data: UnitData::define_Unit( 6, 9, 60, 0, 2, 0, 1, 8000,)
};
const APC: Unit = Unit {
    name: "APC",
    move_type: MoveType::Treads,
    unit_data: UnitData::define_Unit( 6, 0, 60, 0, 1, 0, 0, 5000,)
};
const ARTILLERY: Unit = Unit {
    name: "Artillery",
    move_type: MoveType::Treads,
    unit_data: UnitData::define_Unit( 5, 9, 50, 0, 1, 2, 3, 6000,)
};
const B_COPTER: Unit = Unit {
    name: "B-Copter",
    move_type: MoveType::Air,
    unit_data: UnitData::define_Unit( 6, 6, 99, 2, 3, 0, 1, 9000,)
};
const BATTLESHIP: Unit = Unit {
    name: "Battleship",
    move_type: MoveType::Sea,
    unit_data: UnitData::define_Unit( 5, 9, 99, 1, 2, 2, 6, 28000,)
};
const BLACK_BOAT: Unit = Unit {
    name: "Black Boat",
    move_type: MoveType::Lander,
    unit_data: UnitData::define_Unit( 7, 0, 50, 1, 1, 0, 0, 7500,)
};
const BLACK_BOMB: Unit = Unit {
    name: "Black Bomb",
    move_type: MoveType::Air,
    unit_data: UnitData::define_Unit( 9, 0, 45, 5, 1, 0, 0, 25000,)
};
const BOMBER: Unit = Unit {
    name: "Bomber",
    move_type: MoveType::Air,
    unit_data: UnitData::define_Unit( 7, 9, 99, 5, 2, 0, 1, 22000,)
};
const CARRIER: Unit = Unit {
    name: "Carrier",
    move_type: MoveType::Sea,
    unit_data: UnitData::define_Unit( 5, 9, 99, 1, 4, 3, 8, 30000,)
};
const CRUISER: Unit = Unit {
    name: "Cruiser",
    move_type: MoveType::Sea,
    unit_data: UnitData::define_Unit( 6, 9, 99, 1, 3, 0, 1, 18000,)
};
const FIGHTER: Unit = Unit {
    name: "Fighter",
    move_type: MoveType::Air,
    unit_data: UnitData::define_Unit( 9, 9, 99, 5, 2, 0, 1, 20000,)
};
const INFANTRY: Unit = Unit {
    name: "Infantry",
    move_type: MoveType::Foot,
    unit_data: UnitData::define_Unit( 3, 0, 99, 0, 2, 0, 1, 1000,)
};
const LANDER: Unit = Unit {
    name: "Lander",
    move_type: MoveType::Lander,
    unit_data: UnitData::define_Unit( 6, 0, 99, 1, 1, 0, 0, 12000,)
};
const MD_TANK: Unit = Unit {
    name: "Md. Tank",
    move_type: MoveType::Treads,
    unit_data: UnitData::define_Unit( 5, 8, 50, 0, 1, 0, 1, 16000,)
};
const MECH: Unit = Unit {
    name: "Mech",
    move_type: MoveType::Boot,
    unit_data: UnitData::define_Unit( 2, 3, 70, 0, 2, 0, 1, 3000,)
};
const MEGA_TANK: Unit = Unit {
    name: "Mega Tank",
    move_type: MoveType::Treads,
    unit_data: UnitData::define_Unit( 4, 3, 50, 0, 1, 0, 1, 28000,)
};
const MISSILE: Unit = Unit {
    name: "Missile",
    move_type: MoveType::Tires,
    unit_data: UnitData::define_Unit( 4, 6, 50, 0, 5, 3, 5, 12000,)
};
const NEOTANK: Unit = Unit {
    name: "Neotank",
    move_type: MoveType::Treads,
    unit_data: UnitData::define_Unit( 6, 9, 99, 0, 1, 0, 1, 22000,)
};
const PIPERUNNER: Unit = Unit {
    name: "Piperunner",
    move_type: MoveType::Pipe,
    unit_data: UnitData::define_Unit( 9, 9, 99, 0, 4, 2, 5, 20000,)
};
const RECON: Unit = Unit {
    name: "Recon",
    move_type: MoveType::Tires,
    unit_data: UnitData::define_Unit( 8, 0, 80, 0, 5, 0, 1, 4000,)
};
const ROCKET: Unit = Unit {
    name: "Rocket",
    move_type: MoveType::Tires,
    unit_data: UnitData::define_Unit( 5, 6, 50, 0, 1, 3, 5, 15000,)
};
const STEALTH: Unit = Unit {
    name: "Stealth",
    move_type: MoveType::Air,
    unit_data: UnitData::define_Unit( 6, 6, 60, 5, 4, 0, 1, 24000,)
};
const SUB: Unit = Unit {
    name: "Sub",
    move_type: MoveType::Sea,
    unit_data: UnitData::define_Unit( 5, 6, 60, 1, 5, 0, 1, 20000,)
};
const T_COPTER: Unit = Unit {
    name: "T-Copter",
    move_type: MoveType::Air,
    unit_data: UnitData::define_Unit( 6, 0, 99, 2, 2, 0, 0, 5000,)
};
const TANK: Unit = Unit {
    name: "Tank",
    move_type: MoveType::Treads,
    unit_data: UnitData::define_Unit( 6, 9, 70, 0, 3, 0, 1, 7000,)
};
pub enum UnitType {
    anti_air,
    apc,
    artillery,
    b_copter,
    battleship,
    black_boat,
    black_bomb,
    bomber,
    carrier,
    cruiser,
    fighter,
    infantry,
    lander,
    md_tank,
    mech,
    mega_tank,
    missile,
    neotank,
    piperunner,
    recon,
    rocket,
    stealth,
    sub,
    t_copter,
    tank,
}
use::std::error::Error;
pub fn get_unit(unit_type: UnitType) -> Unit {
    let result = match unit_type {
    anti_air => ANTI_AIR,
    apc => APC,
    artillery => ARTILLERY,
    b_copter => B_COPTER,
    battleship => BATTLESHIP,
    black_boat => BLACK_BOAT,
    black_bomb => BLACK_BOMB,
    bomber => BOMBER,
    carrier => CARRIER,
    cruiser => CRUISER,
    fighter => FIGHTER,
    infantry => INFANTRY,
    lander => LANDER,
    md_tank => MD_TANK,
    mech => MECH,
    mega_tank => MEGA_TANK,
    missile => MISSILE,
    neotank => NEOTANK,
    piperunner => PIPERUNNER,
    recon => RECON,
    rocket => ROCKET,
    stealth => STEALTH,
    sub => SUB,
    t_copter => T_COPTER,
    tank => TANK,
    };
Ok(result)
}
