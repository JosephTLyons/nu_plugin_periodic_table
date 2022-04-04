mod periodic_table_grid;

use nu_plugin::{serve_plugin, JsonSerializer};
use periodic_table::PeriodicTable;

fn main() {
    serve_plugin(&mut PeriodicTable {}, JsonSerializer {})
}
