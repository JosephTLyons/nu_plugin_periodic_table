use crate::extensions::{GroupBlockExt, StateOfMatterExt};
use crate::periodic_table_grid::PERIODIC_TABLE_GRID;
use nu_ansi_term::Color;
use nu_plugin::LabeledError;
use nu_protocol::{Record, Value};
use periodic_table_on_an_enum::{periodic_table, Element};

pub struct PeriodicTable;

impl PeriodicTable {
    pub fn build_classic_table(tag: &nu_protocol::Span) -> Result<Value, LabeledError> {
        let vec: Vec<Value> = PERIODIC_TABLE_GRID
            .into_iter()
            .map(|element_row| {
                let record: Record = element_row
                    .iter()
                    .enumerate()
                    .map(|(i, element_option)| {
                        let value = match element_option {
                            Some(element) => Value::string(
                                {
                                    let symbol = element.get_symbol();
                                    let [r, g, b] = element.get_group().color();
                                    Color::Rgb(r, g, b).paint(symbol).to_string()
                                },
                                *tag,
                            ),
                            None => Value::nothing(*tag),
                        };

                        let group_number = i + 1;
                        (group_number.to_string(), value)
                    })
                    .collect();

                Value::record(record, *tag)
            })
            .collect();

        Ok(Value::list(vec, *tag))
    }

    pub fn build_detailed_table(
        tag: &nu_protocol::Span,
        should_show_full_column_names: bool,
    ) -> Result<Value, LabeledError> {
        let vec: Vec<Value> = periodic_table()
            .into_iter()
            .map(|element| {
                let row = PeriodicTable::row(&element, tag, should_show_full_column_names);
                let record = row
                    .into_iter()
                    .map(|(name, value)| (name.to_owned(), value))
                    .collect::<Record>();
                Value::record(record, *tag)
            })
            .collect();

        Ok(Value::list(vec, *tag))
    }

    fn row<'a>(
        element: &Element,
        tag: &nu_protocol::Span,
        should_show_full_column_names: bool,
    ) -> [(&'a str, Value); 16] {
        [
            ("name", Value::string(element.get_name().to_string(), *tag)),
            (
                if should_show_full_column_names {
                    "symbol"
                } else {
                    "sym"
                },
                Value::string(element.get_symbol().to_string(), *tag),
            ),
            (
                if should_show_full_column_names {
                    "atomic number"
                } else {
                    "a-num"
                },
                Value::int(element.get_atomic_number() as i64, *tag),
            ),
            (
                if should_show_full_column_names {
                    "atomic mass"
                } else {
                    "a-mass"
                },
                Value::float(element.get_atomic_mass() as f64, *tag),
            ),
            (
                if should_show_full_column_names {
                    "atomic radius"
                } else {
                    "a-rad"
                },
                Value::int(element.get_atomic_radius() as i64, *tag),
            ),
            (
                if should_show_full_column_names {
                    "cpk color"
                } else {
                    "cpk-col"
                },
                Value::binary(element.get_cpk().to_vec(), *tag),
            ),
            (
                if should_show_full_column_names {
                    "electron configuration"
                } else {
                    "elec-config"
                },
                Value::string(element.get_electronic_configuration_str().to_string(), *tag),
            ),
            (
                if should_show_full_column_names {
                    "electronegativity"
                } else {
                    "electroneg"
                },
                Value::float(element.get_electronegativity() as f64, *tag),
            ),
            (
                if should_show_full_column_names {
                    "ionization energy"
                } else {
                    "ioniz-energ"
                },
                Value::float(element.get_ionization_energy() as f64, *tag),
            ),
            (
                if should_show_full_column_names {
                    "electron affinity"
                } else {
                    "elec-affin"
                },
                Value::float(element.get_electron_affinity() as f64, *tag),
            ),
            (
                if should_show_full_column_names {
                    "standard state"
                } else {
                    "stand-state"
                },
                Value::string(element.get_standard_state().name().to_string(), *tag),
            ),
            (
                if should_show_full_column_names {
                    "melting point"
                } else {
                    "m-point"
                },
                Value::float(element.get_melting_point() as f64, *tag),
            ),
            (
                if should_show_full_column_names {
                    "boiling point"
                } else {
                    "b-point"
                },
                Value::float(element.get_boiling_point() as f64, *tag),
            ),
            ("density", Value::float(element.get_density() as f64, *tag)),
            (
                if should_show_full_column_names {
                    "group block"
                } else {
                    "g-block"
                },
                Value::string(element.get_group().name().to_string(), *tag),
            ),
            (
                if should_show_full_column_names {
                    "year discovered"
                } else {
                    "year"
                },
                Value::int(element.get_year_discovered() as i64, *tag),
            ),
        ]
    }
}
