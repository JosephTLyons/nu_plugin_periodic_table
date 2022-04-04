use crate::periodic_table_grid::PERIODIC_TABLE_GRID;
use periodic_table_on_an_enum::Element;

pub struct PeriodicTable;

impl PeriodicTable {
    pub fn get_periodic_table_grid() -> [[Option<Element>; 18]; 9] {
        PERIODIC_TABLE_GRID
    }

    pub fn get_element_from_symbol_name(symbol_name: &str) -> Option<Element> {
        Element::from_symbol(symbol_name)
    }
}
