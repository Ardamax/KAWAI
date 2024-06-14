use entities::entity_data::TileData;
use entities::entity_data::UnitData;
use struct_field_names_as_array::FieldNamesAsArray;

fn main() {
    let woods = TileData::define_tile(0, 0, 0, 1, 0, 0, 1);
    assert!(!woods.building());
    assert!(!woods.funds());
    assert!(!woods.destructible());
    assert!(woods.fogs());
    assert!(!woods.sight());
    assert_eq!(woods.is_teleport(), 0);
    assert_eq!(woods.def(), 1);

    println!("{}", woods.is_teleport());
    println!("{:?}", TileData::FIELD_NAMES_AS_ARRAY);

    println!("{}", woods.is_teleport());
    println!("{:?}", UnitData::FIELD_NAMES_AS_ARRAY);
}
