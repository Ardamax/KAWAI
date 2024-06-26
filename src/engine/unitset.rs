use entities::entity_data::UnitData;
use crate::tileset::MoveType;

struct Unit {
    name: &'static str,
    unit_data: UnitData,
    move_type: MoveType
}

const ANTI_AIR: Unit = Unit {
    name: "Anti-Air",
    move_type: MoveType::Treads,
    unit_data: UnitData::define_unit( 6, 9, 60, 0, 2, 0, 1, 8000,)
};
const APC: Unit = Unit {
    name: "APC",
    move_type: MoveType::Treads,
    unit_data: UnitData::define_unit( 6, 0, 60, 0, 1, 0, 0, 5000,)
};
const ARTILLERY: Unit = Unit {
    name: "Artillery",
    move_type: MoveType::Treads,
    unit_data: UnitData::define_unit( 5, 9, 50, 0, 1, 2, 3, 6000,)
};
const B_COPTER: Unit = Unit {
    name: "B-Copter",
    move_type: MoveType::Air,
    unit_data: UnitData::define_unit( 6, 6, 99, 2, 3, 0, 1, 9000,)
};
const BATTLESHIP: Unit = Unit {
    name: "Battleship",
    move_type: MoveType::Sea,
    unit_data: UnitData::define_unit( 5, 9, 99, 1, 2, 2, 6, 28000,)
};
const BLACK_BOAT: Unit = Unit {
    name: "Black Boat",
    move_type: MoveType::Lander,
    unit_data: UnitData::define_unit( 7, 0, 50, 1, 1, 0, 0, 7500,)
};
const BLACK_BOMB: Unit = Unit {
    name: "Black Bomb",
    move_type: MoveType::Air,
    unit_data: UnitData::define_unit( 9, 0, 45, 5, 1, 0, 0, 25000,)
};
const BOMBER: Unit = Unit {
    name: "Bomber",
    move_type: MoveType::Air,
    unit_data: UnitData::define_unit( 7, 9, 99, 5, 2, 0, 1, 22000,)
};
const CARRIER: Unit = Unit {
    name: "Carrier",
    move_type: MoveType::Sea,
    unit_data: UnitData::define_unit( 5, 9, 99, 1, 4, 3, 8, 30000,)
};
const CRUISER: Unit = Unit {
    name: "Cruiser",
    move_type: MoveType::Sea,
    unit_data: UnitData::define_unit( 6, 9, 99, 1, 3, 0, 1, 18000,)
};
const FIGHTER: Unit = Unit {
    name: "Fighter",
    move_type: MoveType::Air,
    unit_data: UnitData::define_unit( 9, 9, 99, 5, 2, 0, 1, 20000,)
};
const INFANTRY: Unit = Unit {
    name: "Infantry",
    move_type: MoveType::Foot,
    unit_data: UnitData::define_unit( 3, 0, 99, 0, 2, 0, 1, 1000,)
};
const LANDER: Unit = Unit {
    name: "Lander",
    move_type: MoveType::Lander,
    unit_data: UnitData::define_unit( 6, 0, 99, 1, 1, 0, 0, 12000,)
};
const MD_TANK: Unit = Unit {
    name: "Md. Tank",
    move_type: MoveType::Treads,
    unit_data: UnitData::define_unit( 5, 8, 50, 0, 1, 0, 1, 16000,)
};
const MECH: Unit = Unit {
    name: "Mech",
    move_type: MoveType::Boot,
    unit_data: UnitData::define_unit( 2, 3, 70, 0, 2, 0, 1, 3000,)
};
const MEGA_TANK: Unit = Unit {
    name: "Mega Tank",
    move_type: MoveType::Treads,
    unit_data: UnitData::define_unit( 4, 3, 50, 0, 1, 0, 1, 28000,)
};
const MISSILE: Unit = Unit {
    name: "Missile",
    move_type: MoveType::Tires,
    unit_data: UnitData::define_unit( 4, 6, 50, 0, 5, 3, 5, 12000,)
};
const NEOTANK: Unit = Unit {
    name: "Neotank",
    move_type: MoveType::Treads,
    unit_data: UnitData::define_unit( 6, 9, 99, 0, 1, 0, 1, 22000,)
};
const PIPERUNNER: Unit = Unit {
    name: "Piperunner",
    move_type: MoveType::Pipe,
    unit_data: UnitData::define_unit( 9, 9, 99, 0, 4, 2, 5, 20000,)
};
const RECON: Unit = Unit {
    name: "Recon",
    move_type: MoveType::Tires,
    unit_data: UnitData::define_unit( 8, 0, 80, 0, 5, 0, 1, 4000,)
};
const ROCKET: Unit = Unit {
    name: "Rocket",
    move_type: MoveType::Tires,
    unit_data: UnitData::define_unit( 5, 6, 50, 0, 1, 3, 5, 15000,)
};
const STEALTH: Unit = Unit {
    name: "Stealth",
    move_type: MoveType::Air,
    unit_data: UnitData::define_unit( 6, 6, 60, 5, 4, 0, 1, 24000,)
};
const SUB: Unit = Unit {
    name: "Sub",
    move_type: MoveType::Sea,
    unit_data: UnitData::define_unit( 5, 6, 60, 1, 5, 0, 1, 20000,)
};
const T_COPTER: Unit = Unit {
    name: "T-Copter",
    move_type: MoveType::Air,
    unit_data: UnitData::define_unit( 6, 0, 99, 2, 2, 0, 0, 5000,)
};
const TANK: Unit = Unit {
    name: "Tank",
    move_type: MoveType::Treads,
    unit_data: UnitData::define_unit( 6, 9, 70, 0, 3, 0, 1, 7000,)
};
pub enum UnitType {
    AntiAir,
    Apc,
    Artillery,
    BCopter,
    Battleship,
    BlackBoat,
    BlackBomb,
    Bomber,
    Carrier,
    Cruiser,
    Fighter,
    Infantry,
    Lander,
    MdTank,
    Mech,
    MegaTank,
    Missile,
    Neotank,
    Piperunner,
    Recon,
    Rocket,
    Stealth,
    Sub,
    TCopter,
    Tank,
}

impl UnitType {
    pub fn movetype(&self) -> MoveType {
        match self {
           UnitType::AntiAir => ANTI_AIR.move_type,
           UnitType::Apc => APC.move_type,
           UnitType::Artillery => ARTILLERY.move_type,
           UnitType::BCopter => B_COPTER.move_type,
           UnitType::Battleship => BATTLESHIP.move_type,
           UnitType::BlackBoat => BLACK_BOAT.move_type,
           UnitType::BlackBomb => BLACK_BOMB.move_type,
           UnitType::Bomber => BOMBER.move_type,
           UnitType::Carrier => CARRIER.move_type,
           UnitType::Cruiser => CRUISER.move_type,
           UnitType::Fighter => FIGHTER.move_type,
           UnitType::Infantry => INFANTRY.move_type,
           UnitType::Lander => LANDER.move_type,
           UnitType::MdTank => MD_TANK.move_type,
           UnitType::Mech => MECH.move_type,
           UnitType::MegaTank => MEGA_TANK.move_type,
           UnitType::Missile => MISSILE.move_type,
           UnitType::Neotank => NEOTANK.move_type,
           UnitType::Piperunner => PIPERUNNER.move_type,
           UnitType::Recon => RECON.move_type,
           UnitType::Rocket => ROCKET.move_type,
           UnitType::Stealth => STEALTH.move_type,
           UnitType::Sub => SUB.move_type,
           UnitType::TCopter => T_COPTER.move_type,
           UnitType::Tank => TANK.move_type,
        }
    }

    pub fn data(&self) -> UnitData {
        match self {
           UnitType::AntiAir => ANTI_AIR.unit_data,
           UnitType::Apc => APC.unit_data,
           UnitType::Artillery => ARTILLERY.unit_data,
           UnitType::BCopter => B_COPTER.unit_data,
           UnitType::Battleship => BATTLESHIP.unit_data,
           UnitType::BlackBoat => BLACK_BOAT.unit_data,
           UnitType::BlackBomb => BLACK_BOMB.unit_data,
           UnitType::Bomber => BOMBER.unit_data,
           UnitType::Carrier => CARRIER.unit_data,
           UnitType::Cruiser => CRUISER.unit_data,
           UnitType::Fighter => FIGHTER.unit_data,
           UnitType::Infantry => INFANTRY.unit_data,
           UnitType::Lander => LANDER.unit_data,
           UnitType::MdTank => MD_TANK.unit_data,
           UnitType::Mech => MECH.unit_data,
           UnitType::MegaTank => MEGA_TANK.unit_data,
           UnitType::Missile => MISSILE.unit_data,
           UnitType::Neotank => NEOTANK.unit_data,
           UnitType::Piperunner => PIPERUNNER.unit_data,
           UnitType::Recon => RECON.unit_data,
           UnitType::Rocket => ROCKET.unit_data,
           UnitType::Stealth => STEALTH.unit_data,
           UnitType::Sub => SUB.unit_data,
           UnitType::TCopter => T_COPTER.unit_data,
           UnitType::Tank => TANK.unit_data,
        }
    }

    pub fn unit_name(&self) -> &str {
        match self {
           UnitType::AntiAir => ANTI_AIR.name,
           UnitType::Apc => APC.name,
           UnitType::Artillery => ARTILLERY.name,
           UnitType::BCopter => B_COPTER.name,
           UnitType::Battleship => BATTLESHIP.name,
           UnitType::BlackBoat => BLACK_BOAT.name,
           UnitType::BlackBomb => BLACK_BOMB.name,
           UnitType::Bomber => BOMBER.name,
           UnitType::Carrier => CARRIER.name,
           UnitType::Cruiser => CRUISER.name,
           UnitType::Fighter => FIGHTER.name,
           UnitType::Infantry => INFANTRY.name,
           UnitType::Lander => LANDER.name,
           UnitType::MdTank => MD_TANK.name,
           UnitType::Mech => MECH.name,
           UnitType::MegaTank => MEGA_TANK.name,
           UnitType::Missile => MISSILE.name,
           UnitType::Neotank => NEOTANK.name,
           UnitType::Piperunner => PIPERUNNER.name,
           UnitType::Recon => RECON.name,
           UnitType::Rocket => ROCKET.name,
           UnitType::Stealth => STEALTH.name,
           UnitType::Sub => SUB.name,
           UnitType::TCopter => T_COPTER.name,
           UnitType::Tank => TANK.name,
        }
    }
}
