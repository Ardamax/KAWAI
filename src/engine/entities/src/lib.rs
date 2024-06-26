// Module that defines entity data storage

pub mod entity_data {
    use struct_field_names_as_array::FieldNamesAsArray;

    #[derive(FieldNamesAsArray)]
    #[field_names_as_array(rename_all = "PascalCase")]
    pub struct TileData {
        building: bool,
        funds: bool,
        destructible: bool,
        fogs: bool,
        sight: bool,
        defense: u8,
    }

    impl TileData {
        pub const fn define_tile(
            building: u8,
            funds: u8,
            destructible: u8,
            fogs: u8,
            sight: u8,
            defense: u8,
        ) -> TileData {
            TileData {
                building: building != 0,
                funds: funds != 0,
                destructible: destructible != 0,
                fogs: fogs != 0,
                sight: sight != 0,
                defense,
            }
        }

        pub fn building(&self) -> bool {
            self.building
        }

        pub fn funds(&self) -> bool {
            self.funds
        }

        pub fn destructible(&self) -> bool {
            self.destructible
        }

        pub fn fogs(&self) -> bool {
            self.fogs
        }

        pub fn sight(&self) -> bool {
            self.sight
        }

        pub fn def(&self) -> u8 {
            self.defense
        }
    }

    #[derive(FieldNamesAsArray)]
    #[field_names_as_array(rename_all = "PascalCase")]
    pub struct UnitData {
        movement: u8,
        ammo: u8,
        fuel: u8,
        consumption: u8,
        vision: u8,
        range_min: u8,
        range_max: u8,
        cost: u16,
    }

    impl UnitData {
        pub const fn define_unit(
            movement: u8,
            ammo: u8,
            fuel: u8,
            consumption: u8,
            vision: u8,
            range_min: u8,
            range_max: u8,
            cost: u16,
        ) -> UnitData {
            UnitData {
                movement,
                ammo,
                fuel,
                consumption,
                vision,
                range_min,
                range_max,
                cost,
            }
        }
        pub fn moves(&self) -> u8 {
            self.movement
        }

        pub fn max_ammo(&self) -> u8 {
            self.ammo
        }

        pub fn max_fuel(&self) -> u8 {
            self.fuel
        }

        pub fn fuel_consumption(&self) -> u8 {
            self.consumption
        }

        pub fn vision_range(&self) -> u8 {
            self.vision
        }

        pub fn attack_range(&self) -> [u8; 2] {
            [self.range_min, self.range_max]
        }

        pub fn unit_cost(&self) -> u16 {
            self.cost
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::entity_data::*;
    #[test]
    fn test_tile_bitfields() {
        let woods = TileData::define_tile(0, 0, 0, 1, 0, 1);
        assert!(!woods.building());
        assert!(!woods.funds());
        assert!(!woods.destructible());
        assert!(woods.fogs());
        assert!(!woods.sight());
        assert_eq!(woods.def(), 1);
    }

    #[test]
    fn test_unit_bitfields() {
        let anti_air = UnitData::define_unit(6, 9, 60, 0, 2, 0, 1, 8000);
        assert_eq!(anti_air.moves(), 6);
        assert_eq!(anti_air.max_ammo(), 9);
        assert_eq!(anti_air.max_fuel(), 60);
        assert_eq!(anti_air.fuel_consumption(), 0);
        assert_eq!(anti_air.vision_range(), 2);
        assert_eq!(anti_air.attack_range(), (0, 1));
        assert_eq!(anti_air.unit_cost(), 8000)
    }
}
