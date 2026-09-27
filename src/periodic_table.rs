use crate::extensions::{GroupBlockExt, StateOfMatterExt};
use crate::periodic_table_grid::PERIODIC_TABLE_GRID;
use nu_ansi_term::Color;
use nu_protocol::{LabeledError, Record, Value};
use periodic_table_on_an_enum::{periodic_table, Element};

// TODO: Rework this to not have any nushell dependencies. Return raw data and nushell mod should convert it into nushell values.

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

    fn row(
        element: &Element,
        tag: &nu_protocol::Span,
        should_show_full_column_names: bool,
    ) -> [(&'static str, Value); 16] {
        let column_name = |full_name, short_name| {
            if should_show_full_column_names {
                full_name
            } else {
                short_name
            }
        };

        [
            ("name", Value::string(element.get_name().to_string(), *tag)),
            (
                column_name("symbol", "sym"),
                Value::string(element.get_symbol().to_string(), *tag),
            ),
            (
                column_name("atomic number", "a-num"),
                Value::int(element.get_atomic_number() as i64, *tag),
            ),
            (
                column_name("atomic mass", "a-mass"),
                Value::float(element.get_atomic_mass() as f64, *tag),
            ),
            (
                column_name("atomic radius", "a-rad"),
                Value::int(element.get_atomic_radius() as i64, *tag),
            ),
            (
                column_name("cpk color", "cpk-col"),
                Value::binary(element.get_cpk().to_vec(), *tag),
            ),
            (
                column_name("electron configuration", "elec-config"),
                Value::string(element.get_electronic_configuration_str().to_string(), *tag),
            ),
            (
                column_name("electronegativity", "electroneg"),
                Value::float(element.get_electronegativity() as f64, *tag),
            ),
            (
                column_name("ionization energy", "ioniz-energ"),
                Value::float(element.get_ionization_energy() as f64, *tag),
            ),
            (
                column_name("electron affinity", "elec-affin"),
                Value::float(element.get_electron_affinity() as f64, *tag),
            ),
            (
                column_name("standard state", "stand-state"),
                Value::string(element.get_standard_state().name().to_string(), *tag),
            ),
            (
                column_name("melting point", "m-point"),
                Value::float(element.get_melting_point() as f64, *tag),
            ),
            (
                column_name("boiling point", "b-point"),
                Value::float(element.get_boiling_point() as f64, *tag),
            ),
            ("density", Value::float(element.get_density() as f64, *tag)),
            (
                column_name("group block", "g-block"),
                Value::string(element.get_group().name().to_string(), *tag),
            ),
            (
                column_name("year discovered", "year"),
                Value::int(element.get_year_discovered() as i64, *tag),
            ),
        ]
    }
}
