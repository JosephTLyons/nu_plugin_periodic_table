use nu_ansi_term::Color;
use nu_plugin::{EvaluatedCall, Plugin, PluginCommand, SimplePluginCommand};
use nu_protocol::{Category, Example, LabeledError, Record, Signature, Span, Value};
use periodic_table_on_an_enum::periodic_table;

use crate::extensions::GroupBlockExt;
use crate::periodic_table::{columns, Field};
use crate::periodic_table_grid::PERIODIC_TABLE_GRID;

pub struct PeriodicTable;

impl Plugin for PeriodicTable {
    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![Box::new(PeriodicTable)]
    }

    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }
}

impl SimplePluginCommand for PeriodicTable {
    type Plugin = PeriodicTable;

    fn name(&self) -> &str {
        "periodic-table"
    }

    fn description(&self) -> &str {
        "List the elements of the periodic table"
    }

    fn signature(&self) -> Signature {
        Signature::build(PluginCommand::name(self))
            .switch(
                "classic",
                "Display the elements in classical form",
                Some('c'),
            )
            .switch("full", "Display the full names of the columns", Some('f'))
            .category(Category::Generators)
    }

    fn examples(&self) -> Vec<Example<'_>> {
        vec![
            Example {
                description: "Display the periodic table in detailed form",
                example: "periodic-table",
                result: None,
            },
            Example {
                description: "Display the periodic table in classic form",
                example: "periodic-table -c",
                result: None,
            },
        ]
    }

    fn run(
        &self,
        _: &Self::Plugin,
        _: &nu_plugin::EngineInterface,
        call: &EvaluatedCall,
        _: &Value,
    ) -> Result<Value, LabeledError> {
        let span = call.head;

        let should_display_classic_table = call.has_flag("classic")?;

        if should_display_classic_table {
            return Ok(classic_table(span));
        }

        let should_show_full_column_names = call.has_flag("full")?;

        Ok(detailed_table(span, should_show_full_column_names))
    }
}

fn classic_table(span: Span) -> Value {
    let rows: Vec<Value> = PERIODIC_TABLE_GRID
        .into_iter()
        .map(|element_row| {
            let record: Record = element_row
                .iter()
                .enumerate()
                .map(|(i, element_option)| {
                    let value = match element_option {
                        Some(element) => {
                            let [r, g, b] = element.get_group().color();
                            let symbol = Color::Rgb(r, g, b).paint(element.get_symbol());
                            Value::string(symbol.to_string(), span)
                        }
                        None => Value::nothing(span),
                    };

                    let group_number = i + 1;
                    (group_number.to_string(), value)
                })
                .collect();

            Value::record(record, span)
        })
        .collect();

    Value::list(rows, span)
}

fn detailed_table(span: Span, should_show_full_column_names: bool) -> Value {
    let rows: Vec<Value> = periodic_table()
        .map(|element| {
            let record: Record = columns(&element)
                .into_iter()
                .map(|column| {
                    let name = if should_show_full_column_names {
                        column.full_name
                    } else {
                        column.short_name
                    };
                    (name.to_owned(), to_value(column.field, span))
                })
                .collect();

            Value::record(record, span)
        })
        .collect();

    Value::list(rows, span)
}

fn to_value(field: Field, span: Span) -> Value {
    match field {
        Field::String(string) => Value::string(string, span),
        Field::Int(int) => Value::int(int, span),
        Field::Float(float) => Value::float(float, span),
        Field::Bytes(bytes) => Value::binary(bytes.to_vec(), span),
    }
}
