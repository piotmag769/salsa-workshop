mod db;
mod input;
mod ir;
mod lexer;
mod parser;
mod solver;

use salsa::Setter;

use crate::db::SpreadsheetDatabase;
use crate::parser::ParserGroup;
use crate::solver::SolverGroup;

macro_rules! row {
    ($($cell_str:literal) | *) => {
        vec![$($cell_str.to_string(),)*]
    };
}

fn main() {
    let mut db: SpreadsheetDatabase = Default::default();
    // let cells = vec![
    //     row! {"0" | "5"  | "$0:0 + $0:1"},
    //     row! {"7" | "12" | "$1:0 + $1:1"},
    //     row! {"1" | "13" | "$2:0 + $2:1"},
    //     row! {"$2:0 + $2:1" | "0"  | "$0:2 + $1:2 + $2:2"},
    // ];
    let cells = vec![row! {"5" | "$0:0 + $0:0"}, row! {"7" | "$0:1 + $1:0"}];

    // Set input.
    db.spreadsheet_input().set_cells(&mut db).to(cells);

    // Run queries.
    let parsed_spreadsheet = db.parse_spreadsheet();
    let solved_spreadsheet = db.solve_spreadsheet(parsed_spreadsheet);

    eprintln!("{solved_spreadsheet:?}");
}
