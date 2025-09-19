mod compile;
mod db;
mod input;
mod ir;
pub mod lexer;

use crate::compile::parse_cell_contents;
use crate::db::SpreadsheetDatabase;
use crate::input::spreadsheet_input;
use salsa::Setter;

macro_rules! row {
    ($($cell_str:literal) | *) => {
        vec![$($cell_str.to_string(),)*]
    };
}

fn main() {
    let mut db: SpreadsheetDatabase = Default::default();
    let cells = vec![
        row! {"0" | "5"  | "$0:0 + $0:1"},
        row! {"7" | "12" | "$1:0 + $1:1"},
        row! {"1" | "13" | "$2:0 + $2:1"},
        row! {"$2:0 + $2:1" | "0"  | "$0:2 + $1:2 + $2:2"},
    ];
    spreadsheet_input(&db).set_cells(&mut db).to(cells);

    parse_cell_contents(&db);
}
