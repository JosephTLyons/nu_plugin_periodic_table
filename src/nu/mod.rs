use nu_plugin::{EvaluatedCall, Plugin, PluginCommand, SimplePluginCommand};
use nu_protocol::{Category, Example, LabeledError, Signature, Value};

use crate::PeriodicTable;

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

    fn usage(&self) -> &str {
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

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "periodic-table".into(),
                example: "Display the periodic table in detailed form".into(),
                result: None,
            },
            Example {
                description: "Display the periodic table in classic form".into(),
                example: "periodic-table -c".into(),
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
        let tag = call.head;

        let should_display_classic_table = call.has_flag("classic")?;

        if should_display_classic_table {
            return PeriodicTable::build_classic_table(&tag);
        }

        let should_show_full_column_names = call.has_flag("full")?;

        PeriodicTable::build_detailed_table(&tag, should_show_full_column_names)
    }
}
