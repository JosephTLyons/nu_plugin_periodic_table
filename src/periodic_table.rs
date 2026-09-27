use crate::extensions::{GroupBlockExt, StateOfMatterExt};
use periodic_table_on_an_enum::Element;

pub enum Value {
    String(&'static str),
    Int(i64),
    Float(f64),
    Rgb([u8; 3]),
}

pub struct Column {
    pub full_name: &'static str,
    pub short_name: &'static str,
    pub value: Value,
}

impl Column {
    fn new(full_name: &'static str, short_name: &'static str, value: Value) -> Self {
        Self {
            full_name,
            short_name,
            value,
        }
    }
}

pub fn columns(element: &Element) -> [Column; 16] {
    [
        Column::new("name", "name", Value::String(element.get_name())),
        Column::new("symbol", "sym", Value::String(element.get_symbol())),
        Column::new(
            "atomic number",
            "a-num",
            Value::Int(element.get_atomic_number() as i64),
        ),
        Column::new(
            "atomic mass",
            "a-mass",
            Value::Float(element.get_atomic_mass().into()),
        ),
        Column::new(
            "atomic radius",
            "a-rad",
            Value::Int(element.get_atomic_radius().into()),
        ),
        Column::new("cpk color", "cpk-col", Value::Rgb(element.get_cpk())),
        Column::new(
            "electron configuration",
            "elec-config",
            Value::String(element.get_electronic_configuration_str()),
        ),
        Column::new(
            "electronegativity",
            "electroneg",
            Value::Float(element.get_electronegativity().into()),
        ),
        Column::new(
            "ionization energy",
            "ioniz-energ",
            Value::Float(element.get_ionization_energy().into()),
        ),
        Column::new(
            "electron affinity",
            "elec-affin",
            Value::Float(element.get_electron_affinity().into()),
        ),
        Column::new(
            "standard state",
            "stand-state",
            Value::String(element.get_standard_state().name()),
        ),
        Column::new(
            "melting point",
            "m-point",
            Value::Float(element.get_melting_point().into()),
        ),
        Column::new(
            "boiling point",
            "b-point",
            Value::Float(element.get_boiling_point().into()),
        ),
        Column::new(
            "density",
            "density",
            Value::Float(element.get_density().into()),
        ),
        Column::new(
            "group block",
            "g-block",
            Value::String(element.get_group().name()),
        ),
        Column::new(
            "year discovered",
            "year",
            Value::Int(element.get_year_discovered().into()),
        ),
    ]
}
