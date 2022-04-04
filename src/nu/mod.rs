use std::collections::VecDeque;

use nu_plugin::{EvaluatedCall, LabeledError, Plugin};
use nu_protocol::{Signature, SyntaxShape, Value};

use crate::PeriodicTable;
use indexmap::IndexMap;

impl Plugin for PeriodicTable {
    fn signature(&self) -> Vec<Signature> {
        vec![Signature::build("periodic-table")
            .desc("Display a periodic table of elements")
            .named(
                "element",
                SyntaxShape::String,
                "Display information about a single element",
                Some('e'),
            )]
    }

    fn run(&mut self, name: &str, call: &EvaluatedCall, _: &Value) -> Result<Value, LabeledError> {
        let tag = call.head;

        if name != "periodic-table" {
            return Ok(Value::Nothing { span: tag });
        }

        let element = call.has_flag("element");

        if element {
            todo!();
        } else {
            build_periodic_table(tag)
        }
    }
}

fn build_periodic_table(tag: nu_protocol::Span) -> Result<Value, LabeledError> {
    let mut vec_deque = VecDeque::new();
    for row in PeriodicTable::get_periodic_table_grid().iter() {
        let mut indexmap = IndexMap::new();

        for (i, element_option) in row.iter().enumerate() {
            let value = match element_option {
                Some(element) => Value::String {
                    val: element.get_symbol().into(),
                    span: tag,
                },
                None => Value::Nothing { span: tag },
            };

            indexmap.insert(i.to_string(), value);
        }

        let cols: Vec<String> = indexmap.keys().map(|f| f.to_string()).collect();
        let mut vals: Vec<Value> = Vec::new();
        for c in &cols {
            if let Some(x) = indexmap.get(c) {
                vals.push(x.to_owned())
            }
        }
        vec_deque.push_back(Value::Record {
            cols,
            vals,
            span: tag,
        })
    }
    Ok(Value::List {
        vals: vec_deque.into_iter().collect(),
        span: tag,
    })
}
