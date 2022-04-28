use nu_plugin::{EvaluatedCall, LabeledError, Plugin};
use nu_protocol::{Signature, Value};

use crate::PeriodicTable;

impl Plugin for PeriodicTable {
    fn signature(&self) -> Vec<Signature> {
        vec![Signature::build("periodic-table")
            .switch(
                "classic",
                "Display the elements in classical form",
                Some('c'),
            )
            .switch("full", "Display the full names of the columns", Some('f'))]
    }

    fn run(&mut self, name: &str, call: &EvaluatedCall, _: &Value) -> Result<Value, LabeledError> {
        let tag = call.head;

        if name != "periodic-table" {
            return Ok(Value::Nothing { span: tag });
        }

        let should_display_classic_table = call.has_flag("classic");

        if should_display_classic_table {
            return PeriodicTable::build_classic_table(&tag);
        }

        let should_show_full_column_names = call.has_flag("full");

        PeriodicTable::build_detailed_table(&tag, should_show_full_column_names)
    }
}
