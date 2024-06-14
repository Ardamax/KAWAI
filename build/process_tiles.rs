use convert_case::{Case, Casing};
use entities::entity_data::TileData;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};
use struct_field_names_as_array::FieldNamesAsArray;

type Header = (String, Vec<String>);
type Record = (String, Vec<u8>);

type TileIDLoc = [usize; 2];
type StaticDataLoc = HashMap<String, usize>;
type MoveTypesLoc = HashMap<String, usize>;

type StaticData = HashMap<String, u8>;
type MoveCost = HashMap<String, u8>;

pub fn process_tileset_file(tileset_file: &Path, tileset_module: &Path) -> HashSet<String> {
    let mut tileset_module = BufWriter::new(File::create(tileset_module).unwrap());

    let tile_ids_loc: TileIDLoc;
    let static_data_loc: StaticDataLoc;
    let movetypes_loc: MoveTypesLoc;

    let file = match File::open(tileset_file) {
        Ok(file) => file,
        Err(e) => panic!("Problems with opening file: {}", e),
    };

    match define_tile_struct(&mut tileset_module) {
        Ok(_) => (),
        Err(e) => panic!("Failed to write movetype enume, {}", e),
    }

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    match extract_tileset_header(&mut rdr) {
        Ok(result) => (tile_ids_loc, static_data_loc, movetypes_loc) = result,
        Err(e) => panic!("Problems reading CSV header: {}", e),
    }

    match define_movetype(&mut tileset_module, &movetypes_loc) {
        Ok(_) => (),
        Err(e) => panic!("Failed to write movetype enum, {}", e),
    }

    let all_tile_data: Vec<([u8; 2], String, StaticData, MoveCost)>;
    match extract_tileset_data(tile_ids_loc, static_data_loc, &movetypes_loc, &mut rdr) {
        Ok(result) => all_tile_data = result,
        Err(e) => panic!("Problems with processing tile data: {}", e),
    }

    let mut const_tiles: Vec<([u8; 2], String)> = vec![];
    for data in all_tile_data {
        match define_tile_constant(&mut tileset_module, &data.1, &data.2, &data.3) {
            Ok(const_name) => const_tiles.push((data.0, const_name)),
            Err(e) => panic!("Failed to define tile constants, {}", e),
        }
    }

    match write_tile_lookup(&mut tileset_module, &const_tiles) {
        Ok(_) => (),
        Err(e) => panic!("Failed to write tile lookup, {}", e),
    };

    movetypes_loc.keys().cloned().collect()
}

/// A function to build the tileset module from the tileset CSV file. This will
/// generate 3 lookup tables, one to match tile id to move data, one to match
/// tile data to tile names, and one to match tile data to attributes. It
/// returns a vector containing valid tile keys and found move types
fn extract_tileset_data(
    tile_ids_loc: TileIDLoc,
    static_data_loc: StaticDataLoc,
    movetypesloc: &MoveTypesLoc,
    rdr: &mut csv::Reader<File>,
) -> Result<Vec<([u8; 2], String, StaticData, MoveCost)>, Box<dyn Error>> {
    let mut all_tile_data: Vec<([u8; 2], String, StaticData, MoveCost)> = vec![];
    let [id_min_idx, id_max_idx] = tile_ids_loc;
    let mut filled = [false; 256];
    for result in rdr.deserialize() {
        let record: Record = result?;
        let (name, data) = record;
        let id_lims = [data[id_min_idx], data[id_max_idx]];

        for valid_id in id_lims[0]..id_lims[1] {
            if filled[valid_id as usize] {
                return Err(format!("tile id overlap for {}", name))?;
            } else {
                filled[valid_id as usize] = true;
            }
        }
        let mut move_costs = MoveCost::new();
        let mut static_data = StaticData::new();
        for key in TileData::FIELD_NAMES_AS_ARRAY {
            static_data.insert(key.to_string(), data[static_data_loc[key]]);
        }
        for (key, value) in movetypesloc {
            move_costs.insert(key.to_string(), data[*value]);
        }
        all_tile_data.push((id_lims, name, static_data, move_costs))
    }
    Ok(all_tile_data)
}

/// A function to read in the CSV header and determine where static data,
/// the ID bounds, and the movetype entries are.
fn extract_tileset_header(
    rdr: &mut csv::Reader<File>,
) -> Result<(TileIDLoc, StaticDataLoc, MoveTypesLoc), Box<dyn Error>> {
    let mut tile_ids_loc: TileIDLoc = [0, 0];
    let mut static_data_loc: StaticDataLoc = HashMap::new();
    let mut movetypesloc: MoveTypesLoc = HashMap::new();

    // Makes an assumption that header is the name, followed by data that is all u8s
    let header_vec: Header = rdr.headers()?.deserialize(None)?;
    for (i, header) in header_vec.1.iter().enumerate() {
        match header.as_str() {
            "ID_min" => tile_ids_loc[0] = i,
            "ID_max" => tile_ids_loc[1] = i,
            _ => {
                if TileData::FIELD_NAMES_AS_ARRAY.contains(&header.as_str()) {
                    static_data_loc.insert(header.to_string(), i);
                } else {
                    movetypesloc.insert(header.to_string(), i);
                }
            }
        }
    }
    if static_data_loc.len() != TileData::FIELD_NAMES_AS_ARRAY.len() {
        Err(format!(
            "Not all static data provided. Expected {} got {}",
            TileData::FIELD_NAMES_AS_ARRAY.len(),
            static_data_loc.len()
        ))?
    }

    Ok((tile_ids_loc, static_data_loc, movetypesloc))
}

fn define_movetype(
    file: &mut BufWriter<File>,
    movetypes: &MoveTypesLoc,
) -> Result<(), std::io::Error> {
    // Write a movetypes enum
    writeln!(file, "pub enum MoveType {{")?;
    for movetype in movetypes.keys() {
        writeln!(file, "    {},", movetype)?;
    }
    writeln!(file, "}}\n")?;

    writeln!(file, "struct MoveCost {{")?;
    for movetype in movetypes.keys() {
        writeln!(file, "    {}_cost: u8,", movetype.to_case(Case::Snake))?;
    }
    writeln!(file, "}}\n")?;

    writeln!(
        file,
        "impl MoveCost {{
    pub fn cost(&self, movetype: MoveType) -> u8 {{
        match movetype {{"
    )?;
    for movetype in movetypes.keys() {
        writeln!(
            file,
            "            MoveType::{} => self.{}_cost,",
            movetype,
            movetype.to_case(Case::Snake)
        )?;
    }
    writeln!(
        file,
        "       }}
    }}
}}"
    )?;
    Ok(())
}

fn define_tile_struct(file: &mut BufWriter<File>) -> Result<(), std::io::Error> {
    writeln!(
        file,
        "use entities::entity_data::TileData;

pub struct Tile {{
    pub name: &'static str,
    pub tile_data: TileData,
    move_data: MoveCost
}}\n"
    )?;
    writeln!(file, "impl Tile {{")?;
    /*     for static_data_field in TileData::FIELD_NAMES_AS_ARRAY {
        writeln!(file, "    pub fn {}(&self) {{", static_data_field)?;
        writeln!(
            file,
            "        self.tile_data.{}()",
            static_data_field.to_case(Case::Snake)
        )?;
        writeln!(file, "    }}")?;
    } */
    writeln!(
        file,
        "    pub fn move_cost(&self, movetype: MoveType) -> u8 {{"
    )?;
    writeln!(file, "        self.move_data.cost(movetype)")?;
    writeln!(file, "    }}")?;
    writeln!(file, "}}\n")?;
    Ok(())
}

fn define_tile_constant(
    file: &mut BufWriter<File>,
    name: &str,
    static_data: &StaticData,
    move_costs: &MoveCost,
) -> Result<String, std::io::Error> {
    writeln!(
        file,
        "const {}: Tile = Tile {{",
        name.to_case(Case::UpperSnake)
    )?;
    writeln!(file, "    name: \"{}\",", name)?;
    writeln!(file, "    move_data: MoveCost {{")?;
    for (movetype, cost) in move_costs {
        writeln!(
            file,
            "        {}_cost : {},",
            movetype.to_case(Case::Snake),
            cost
        )?;
    }
    writeln!(file, "    }},")?;
    write!(file, "    tile_data: TileData::define_tile(")?;
    for field in TileData::FIELD_NAMES_AS_ARRAY {
        write!(file, " {},", static_data[field])?;
    }
    writeln!(file, ")\n}};")?;
    Ok(name.to_string().to_case(Case::UpperSnake))
}

fn write_tile_lookup(
    file: &mut BufWriter<File>,
    const_tiles: &Vec<([u8; 2], String)>,
) -> Result<(), std::io::Error> {
    writeln!(file, "use::std::error::Error;")?;
    writeln!(
        file,
        "pub fn get_tile(id:u8) -> Result<Tile, Box<dyn Error>> {{"
    )?;
    writeln!(file, "    let result = match id {{")?;
    for (limits, const_name) in const_tiles {
        if limits[1] - limits[0] <= 1 {
            writeln!(file, "        {} => {},", limits[0], const_name)?;
        } else {
            writeln!(
                file,
                "        {}..={} => {},",
                limits[0],
                limits[1] - 1,
                const_name
            )?;
        }
    }
    writeln!(
        file,
        "        _ => return Err(format!(\"Invalid Tile ID {{}}\", id))?"
    )?;
    writeln!(file, "    }};\nOk(result)\n}}")?;
    Ok(())
}
