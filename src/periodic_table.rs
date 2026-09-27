use crate::extensions::{GroupBlockExt, StateOfMatterExt};
use periodic_table_on_an_enum::Element;

/// A single piece of element data, independent of how it is displayed.
pub enum Field {
    String(&'static str),
    Int(i64),
    Float(f64),
    Bytes([u8; 3]),
}

pub struct Column {
    pub full_name: &'static str,
    pub short_name: &'static str,
    pub field: Field,
}

/// The detailed-table columns for `element`, in display order.
pub fn columns(element: &Element) -> [Column; 16] {
    let column = |full_name, short_name, field| Column {
        full_name,
        short_name,
        field,
    };

    [
        column("name", "name", Field::String(element.get_name())),
        column("symbol", "sym", Field::String(element.get_symbol())),
        column(
            "atomic number",
            "a-num",
            Field::Int(element.get_atomic_number() as i64),
        ),
        column(
            "atomic mass",
            "a-mass",
            Field::Float(element.get_atomic_mass().into()),
        ),
        column(
            "atomic radius",
            "a-rad",
            Field::Int(element.get_atomic_radius().into()),
        ),
        column("cpk color", "cpk-col", Field::Bytes(element.get_cpk())),
        column(
            "electron configuration",
            "elec-config",
            Field::String(element.get_electronic_configuration_str()),
        ),
        column(
            "electronegativity",
            "electroneg",
            Field::Float(element.get_electronegativity().into()),
        ),
        column(
            "ionization energy",
            "ioniz-energ",
            Field::Float(element.get_ionization_energy().into()),
        ),
        column(
            "electron affinity",
            "elec-affin",
            Field::Float(element.get_electron_affinity().into()),
        ),
        column(
            "standard state",
            "stand-state",
            Field::String(element.get_standard_state().name()),
        ),
        column(
            "melting point",
            "m-point",
            Field::Float(element.get_melting_point().into()),
        ),
        column(
            "boiling point",
            "b-point",
            Field::Float(element.get_boiling_point().into()),
        ),
        column(
            "density",
            "density",
            Field::Float(element.get_density().into()),
        ),
        column(
            "group block",
            "g-block",
            Field::String(element.get_group().name()),
        ),
        column(
            "year discovered",
            "year",
            Field::Int(element.get_year_discovered().into()),
        ),
    ]
}
