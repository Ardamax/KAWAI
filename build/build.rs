use std::path::Path;

mod process_tiles;
mod process_units;

fn main() {
    // Compares tileset module with tileset csv and if csv is newer rebuild tileset
    let tileset_file = Path::new("resources/data/tileset.csv");
    let tileset_module = Path::new("src/engine/tileset.rs");

    let movetypes = process_tiles::process_tileset_file(tileset_file, tileset_module);

    let units_file = Path::new("resources/data/units.csv");
    let units_module = Path::new("src/engine/unitset.rs");

    process_units::process_units_file(units_file, units_module, movetypes)
}
