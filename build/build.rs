use std::path::Path;

mod process_tiles;
mod process_units;

fn main() {
    // Compares tileset module with tileset csv and if csv is newer rebuild tileset
    let tileset_file = Path::new("src/engine/data/tileset.csv");
    let tileset_module = Path::new("src/engine/tileset.rs");

    let movetypes = process_tiles::process_tileset_file(tileset_file, tileset_module);

    let units_file = Path::new("src/engine/data/units.csv");
    let units_module = Path::new("src/engine/combat_units.rs");

    process_units::process_units_file(units_file, units_module, movetypes)
}
