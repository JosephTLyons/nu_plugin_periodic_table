use nu_plugin::{EvaluatedCall, LabeledError, Plugin};
use nu_protocol::{Signature, Value};

use crate::PeriodicTable;

impl Plugin for PeriodicTable {
    fn signature(&self) -> Vec<Signature> {
        vec![Signature::build("periodic-table")
            .desc("List the elements of the periodic table")
            .switch(
                "classic",
                "Display the elements in classical form",
                Some('c'),
            )]
    }

    fn run(&mut self, name: &str, call: &EvaluatedCall, _: &Value) -> Result<Value, LabeledError> {
        let tag = call.head;

        if name != "periodic-table" {
            return Ok(Value::Nothing { span: tag });
        }

        if call.has_flag("classic") {
            return PeriodicTable::build_classic_table(&tag)
        }

        PeriodicTable::build_detailed_table(&tag)
    }
}
