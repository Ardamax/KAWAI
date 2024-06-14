use convert_case::{Case, Casing};
use entities::entity_data::UnitData;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};
use struct_field_names_as_array::FieldNamesAsArray;

type Header = (String, String, Vec<String>);
type Record = (String, String, Vec<u16>);

type MoveTypes = HashSet<String>;
type StaticDataLoc = HashMap<String, usize>;
type MoveTypeString = String;
type UnitName = String;

type StaticData = HashMap<String, u16>;

pub fn process_units_file(units_file: &Path, units_module: &Path, movetypes: MoveTypes) {
    // Compares units module with units csv and if csv is newer rebuild units
    let mut units_module = BufWriter::new(File::create(units_module).unwrap());

    let file = match File::open(units_file) {
        Ok(file) => file,
        Err(e) => panic!("Problems with opening file: {}", e),
    };

    match define_unit_struct(&mut units_module) {
        Ok(_) => (),
        Err(e) => panic!("Failed to write movetype enum, {}", e),
    }

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    let static_data_loc = match extract_units_header(&mut rdr) {
        Ok(static_data_loc) => static_data_loc,
        Err(e) => panic!("Problems reading CSV header: {}", e),
    };

    let all_unit_data: Vec<(String, String, StaticData)>;
    all_unit_data = match extract_unit_data(&mut rdr, static_data_loc, movetypes) {
        Ok(result) => result,
        Err(e) => panic!("Problems with extracting unit data: {}", e),
    };

    let mut const_units: Vec<String> = vec![];
    for (name, movetype, static_data) in all_unit_data {
        match define_unit_constant(&mut units_module, &name, &movetype, &static_data) {
            Ok(const_name) => const_units.push(const_name),
            Err(e) => panic!("Problems with defining unit constant for {}: {}", name, e),
        };
    }

    match write_unit_lookup(&mut units_module, &const_units) {
        Ok(_) => (),
        Err(e) => panic!("Problems with writing unit enum and lookup, {}", e),
    }
}

/// A function to read in the CSV header and determine where static data,
/// the ID bounds, and the movetype entries are.
fn extract_units_header(rdr: &mut csv::Reader<File>) -> Result<StaticDataLoc, Box<dyn Error>> {
    let mut static_data_loc: StaticDataLoc = HashMap::new();
    let header_vec: Header = rdr.headers()?.deserialize(None)?;
    for (i, header) in header_vec.2.iter().enumerate() {
        if UnitData::FIELD_NAMES_AS_ARRAY.contains(&header.as_str()) {
            static_data_loc.insert(header.to_string(), i);
        } else if header == "Movetype" {
            ()
        } else {
            Err(format!("Undefined unit data. \"{}\"", header.to_string()))?
        }
    }
    if static_data_loc.len() != UnitData::FIELD_NAMES_AS_ARRAY.len() {
        Err(format!(
            "Not all static data provided. Expected {} got {}",
            UnitData::FIELD_NAMES_AS_ARRAY.len(),
            static_data_loc.len()
        ))?
    }

    Ok(static_data_loc)
}

fn define_unit_struct(file: &mut BufWriter<File>) -> Result<(), std::io::Error> {
    writeln!(
        file,
        "use entities::entity_data::UnitData;
use crate::tileset::MoveType;

pub struct Unit {{
    pub name: &'static str,
    pub unit_data: UnitData,
    move_type: MoveType
}}\n"
    )?;
    Ok(())
}

fn define_unit_constant(
    file: &mut BufWriter<File>,
    name: &str,
    movetype: &str,
    static_data: &StaticData,
) -> Result<String, std::io::Error> {
    writeln!(
        file,
        "const {}: Unit = Unit {{",
        name.to_uppercase()
            .replace(" ", "_")
            .replace(".", "")
            .replace("-", "_")
    )?;
    writeln!(file, "    name: \"{}\",", name)?;
    writeln!(file, "    move_type: MoveType::{},", movetype)?;
    write!(file, "    unit_data: UnitData::define_unit(")?;
    for field in UnitData::FIELD_NAMES_AS_ARRAY {
        write!(file, " {},", static_data[field])?;
    }
    writeln!(file, ")\n}};")?;
    Ok(name
        .to_uppercase()
        .replace(" ", "_")
        .replace(".", "")
        .replace("-", "_"))
}

fn write_unit_lookup(
    file: &mut BufWriter<File>,
    const_units: &Vec<String>,
) -> Result<(), std::io::Error> {
    writeln!(file, "pub enum UnitType {{")?;
    let mut type_const_key: Vec<(String, String)> = vec![];
    for const_name in const_units {
        type_const_key.push((
            const_name.to_case(Case::Pascal),
            const_name.to_case(Case::UpperSnake),
        ));
    }
    for (type_name, _) in &type_const_key {
        writeln!(file, "    {},", type_name)?;
    }
    writeln!(file, "}}\n")?;
    writeln!(file, "impl UnitType {{")?;
    writeln!(file, "    pub fn movetype(&self) -> MoveType {{")?;
    writeln!(file, "        match self {{")?;
    for (type_name, const_name) in &type_const_key {
        writeln!(
            file,
            "           UnitType::{} => {}.move_type,",
            type_name, const_name
        )?;
    }
    writeln!(file, "        }}\n    }}\n")?;

    writeln!(file, "    pub fn data(&self) -> UnitData {{")?;
    writeln!(file, "        match self {{")?;
    for (type_name, const_name) in &type_const_key {
        writeln!(
            file,
            "           UnitType::{} => {}.unit_data,",
            type_name, const_name
        )?;
    }
    writeln!(file, "        }}\n    }}\n")?;
    writeln!(file, "    pub fn unit_name(&self) -> &str {{")?;
    writeln!(file, "        match self {{")?;
    for (type_name, const_name) in &type_const_key {
        writeln!(
            file,
            "           UnitType::{} => {}.name,",
            type_name, const_name
        )?;
    }
    writeln!(file, "        }}\n    }}\n}}")?;
    Ok(())
}

/// A function to build the tileset module from the tileset CSV file. This will
/// generate 3 lookup tables, one to match tile id to move data, one to match
/// tile data to tile names, and one to match tile data to attributes. It
/// returns a vector containing valid tile keys and found move types
fn extract_unit_data(
    rdr: &mut csv::Reader<File>,
    static_data_loc: StaticDataLoc,
    movetypes: MoveTypes,
) -> Result<Vec<(UnitName, MoveTypeString, StaticData)>, Box<dyn Error>> {
    let mut all_unit_data: Vec<(UnitName, MoveTypeString, StaticData)> = vec![];
    for result in rdr.deserialize() {
        let record: Record = result?;
        let (name, movetype, data) = record;
        if !movetypes.contains(&movetype) {
            Err(format!(
                "Invalid movetype in {}. Found movetype of {}",
                name, movetype
            ))?
        }
        let mut static_data = StaticData::new();
        for key in UnitData::FIELD_NAMES_AS_ARRAY {
            static_data.insert(key.to_string(), data[static_data_loc[key]]);
        }
        all_unit_data.push((name, movetype, static_data))
    }
    Ok(all_unit_data)
}
