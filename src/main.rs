mod db;
mod input;
mod ir;
mod lexer;
mod parser;
mod solver;

use crate::db::SpreadsheetDatabase;
use crate::input::raw::RawSpreadsheet;
use crate::parser::ParserGroup;
use crate::solver::SolverGroup;

macro_rules! row {
    ($($cell_str:literal) | *) => {
        vec![$($cell_str.to_string(),)*]
    };
}

// TODO: add #[salsa::tracked] where appropriate.
fn main() {
    let db: SpreadsheetDatabase = Default::default();
    let cells = vec![row! {"5" | "$0:0 + $1:0"}, row! {"7" | "$0:1 + $1:0"}];

    let raw_spreadsheet = todo!("create new spreadsheet input using `cells`");

    // Run queries.
    let parsed_spreadsheet = db.parse_spreadsheet(raw_spreadsheet);
    let solved_spreadsheet = db.solve_spreadsheet(parsed_spreadsheet);
    eprintln!("{solved_spreadsheet:?}");
}
