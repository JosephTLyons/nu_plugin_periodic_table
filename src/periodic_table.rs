use std::collections::VecDeque;

use nu_ansi_term::Color;
use nu_plugin::LabeledError;
use nu_protocol::Value;

use indexmap::IndexMap;
use periodic_table_on_an_enum::{periodic_table, Element, GroupBlock, StateOfMatter};

use crate::periodic_table_grid::PERIODIC_TABLE_GRID;

pub struct PeriodicTable;

impl PeriodicTable {
    pub fn build_classic_table(tag: &nu_protocol::Span) -> Result<Value, LabeledError> {
        let mut vec_deque = VecDeque::new();
        for row in PERIODIC_TABLE_GRID.iter() {
            let mut row_indexmap = IndexMap::new();

            for (i, element_option) in row.iter().enumerate() {
                let value = match element_option {
                    Some(element) => Value::String {
                        val: {
                            let symbol = element.get_symbol();
                            let [r, g, b] =
                                PeriodicTable::get_group_block_color(&element.get_group());
                            Color::Rgb(r, g, b).paint(symbol).to_string()
                        },
                        span: *tag,
                    },
                    None => Value::Nothing { span: *tag },
                };

                row_indexmap.insert(i.to_string(), value);
            }

            // Clean this logic up, there is probably a more direct way of doing this
            let cols: Vec<String> = row_indexmap.keys().map(|f| f.to_string()).collect();
            let mut vals: Vec<Value> = Vec::new();

            for c in &cols {
                if let Some(x) = row_indexmap.get(c) {
                    vals.push(x.to_owned())
                }
            }

            vec_deque.push_back(Value::Record {
                cols,
                vals,
                span: *tag,
            })
        }
        Ok(Value::List {
            vals: vec_deque.into_iter().collect(),
            span: *tag,
        })
    }

    pub fn build_detailed_table(
        tag: &nu_protocol::Span,
        should_show_should_show_full_column_names: bool,
    ) -> Result<Value, LabeledError> {
        let mut vec_deque = VecDeque::new();

        for element in periodic_table() {
            let row_indexmap = PeriodicTable::get_row_indexmap(
                &element,
                tag,
                should_show_should_show_full_column_names,
            );

            // Clean this logic up, there is probably a more direct way of doing this
            let cols: Vec<String> = row_indexmap.keys().map(|f| f.to_string()).collect();
            let mut vals: Vec<Value> = Vec::new();

            for c in &cols {
                if let Some(x) = row_indexmap.get(c) {
                    vals.push(x.to_owned())
                }
            }

            vec_deque.push_back(Value::Record {
                cols,
                vals,
                span: *tag,
            })
        }

        Ok(Value::List {
            vals: vec_deque.into_iter().collect(),
            span: *tag,
        })
    }

    fn get_row_indexmap(
        element: &Element,
        tag: &nu_protocol::Span,
        should_show_full_column_names: bool,
    ) -> IndexMap<String, Value> {
        let mut row_indexmap = IndexMap::new();

        row_indexmap.insert(
            "name".to_string(),
            Value::String {
                val: element.get_name().to_string(),
                span: *tag,
            },
        );
        row_indexmap.insert(
            if should_show_full_column_names {
                "symbol"
            } else {
                "sym"
            }
            .to_string(),
            Value::String {
                val: element.get_symbol().to_string(),
                span: *tag,
            },
        );
        row_indexmap.insert(
            if should_show_full_column_names {
                "atomic number"
            } else {
                "a-num"
            }
            .to_string(),
            Value::Int {
                val: element.get_atomic_number() as i64,
                span: *tag,
            },
        );
        row_indexmap.insert(
            if should_show_full_column_names {
                "atomic mass"
            } else {
                "a-mass"
            }
            .to_string(),
            Value::Float {
                val: element.get_atomic_mass() as f64,
                span: *tag,
            },
        );
        row_indexmap.insert(
            if should_show_full_column_names {
                "atomic radius"
            } else {
                "a-rad"
            }
            .to_string(),
            Value::Int {
                val: element.get_atomic_radius() as i64,
                span: *tag,
            },
        );
        row_indexmap.insert(
            if should_show_full_column_names {
                "cpk color"
            } else {
                "cpk-col"
            }
            .to_string(),
            Value::Binary {
                val: element.get_cpk().to_vec(),
                span: *tag,
            },
        );
        row_indexmap.insert(
            if should_show_full_column_names {
                "electron configuration"
            } else {
                "elec-config"
            }
            .to_string(),
            Value::String {
                val: element.get_electronic_configuration_str().to_string(),
                span: *tag,
            },
        );
        row_indexmap.insert(
            if should_show_full_column_names {
                "electronegativity"
            } else {
                "electroneg"
            }
            .to_string(),
            Value::Float {
                val: element.get_electronegativity() as f64,
                span: *tag,
            },
        );
        row_indexmap.insert(
            if should_show_full_column_names {
                "ionization energy"
            } else {
                "ioniz-energ"
            }
            .to_string(),
            Value::Float {
                val: element.get_ionization_energy() as f64,
                span: *tag,
            },
        );
        row_indexmap.insert(
            if should_show_full_column_names {
                "electron affinity"
            } else {
                "elec-affin"
            }
            .to_string(),
            Value::Float {
                val: element.get_electron_affinity() as f64,
                span: *tag,
            },
        );
        // TODO: CustomValue?,
        // row_indexmap.insert(
        //     if should_show_full_column_names {
        //         "oxidization state"
        //     } else {
        //         "oxid-state"
        //     }
        //     .to_string(),
        //     Value::String {
        //         val: element.get_oxidation_states(),
        //         span: *tag,
        //     },
        // );
        row_indexmap.insert(
            if should_show_full_column_names {
                "standard state"
            } else {
                "stand-state"
            }
            .to_string(),
            Value::String {
                val: PeriodicTable::get_state_of_matter_name(&element.get_standard_state())
                    .to_string(),
                span: *tag,
            },
        );
        row_indexmap.insert(
            if should_show_full_column_names {
                "melting point"
            } else {
                "m-point"
            }
            .to_string(),
            Value::Float {
                val: element.get_melting_point() as f64,
                span: *tag,
            },
        );
        row_indexmap.insert(
            if should_show_full_column_names {
                "boiling point"
            } else {
                "b-point"
            }
            .to_string(),
            Value::Float {
                val: element.get_boiling_point() as f64,
                span: *tag,
            },
        );
        row_indexmap.insert(
            "density".to_string(),
            Value::Float {
                val: element.get_density() as f64,
                span: *tag,
            },
        );
        row_indexmap.insert(
            if should_show_full_column_names {
                "group block"
            } else {
                "g-block"
            }
            .to_string(),
            Value::String {
                val: PeriodicTable::get_group_block_name(&element.get_group()).to_string(),
                span: *tag,
            },
        );
        row_indexmap.insert(
            if should_show_full_column_names {
                "year discovered"
            } else {
                "year"
            }
            .to_string(),
            Value::Int {
                val: element.get_year_discovered() as i64,
                span: *tag,
            },
        );

        row_indexmap
    }

    fn get_state_of_matter_name(state_of_matter: &StateOfMatter) -> &str {
        match state_of_matter {
            StateOfMatter::Solid => "Solid",
            StateOfMatter::Liquid => "Liquid",
            StateOfMatter::Gas => "Gas",
        }
    }

    fn get_group_block_name(group_block: &GroupBlock) -> &str {
        match group_block {
            GroupBlock::AlkaliMetal => "Alkali Metal",
            GroupBlock::AlkalineEarthMetal => "Alkaline Earth Metal",
            GroupBlock::Lanthanide => "Lanthanide",
            GroupBlock::Actinide => "Actinide",
            GroupBlock::TransitionMetal => "Transition Metal",
            GroupBlock::PostTransitionMetal => "Post Transition Metal",
            GroupBlock::Metalloid => "Metalloid",
            GroupBlock::NonMetal => "Non Metal",
            GroupBlock::Halogen => "Halogen",
            GroupBlock::NobleGas => "Noble Gas",
        }
    }

    fn get_group_block_color(group_block: &GroupBlock) -> [u8; 3] {
        match group_block {
            GroupBlock::AlkaliMetal => [76, 152, 100],
            GroupBlock::AlkalineEarthMetal => [51, 104, 170],
            GroupBlock::Lanthanide => [214, 77, 148],
            GroupBlock::Actinide => [219, 196, 204],
            GroupBlock::TransitionMetal => [75, 53, 140],
            GroupBlock::PostTransitionMetal => [245, 201, 84],
            GroupBlock::Metalloid => [99, 187, 202],
            GroupBlock::NonMetal => [189, 218, 157],
            GroupBlock::Halogen => [248, 221, 189],
            GroupBlock::NobleGas => [208, 56, 83],
        }
    }
}
