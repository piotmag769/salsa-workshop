mod db;
mod diagnostic;
mod input;
mod ir;
mod lexer;
mod parser;
mod solver;

use salsa::Setter;

use crate::db::SpreadsheetDatabase;
use crate::input::raw::RawSpreadsheet;
use crate::parser::ParserGroup;
use crate::solver::SolverGroup;

macro_rules! row {
    ($($cell_str:literal) | *) => {
        vec![$($cell_str.to_string(),)*]
    };
}

fn main() {
    let mut db: SpreadsheetDatabase = Default::default();

    // This symbolises changes in the state of a spreadsheet over time.
    let mut queue = [
        [
            row! {"13 14" | "5 + "     | "$0:0 + $0:1"},
            row! {"7 + -" | "12 + bro" | "$1:0 + $1:1"},
            row! {"+ 1"   | "13"       | "$2:0 + $2:1"},
        ],
        // User changed cell 0:1.
        // [
        //     row! {"3" | "5 - 3"  | "$0:0 + $0:1"},
        //     row! {"7" | "12" | "$1:0 + $1:1"},
        //     row! {"1" | "13" | "$2:0 + $2:1"},
        // ],
        // // User changed cell 2:2.
        // [
        //     row! {"3" | "5 - 3"  | "$0:0 + $0:1"},
        //     row! {"7" | "12" | "$1:0 + $1:1"},
        //     row! {"1" | "13" | "$2:0 + $2:1 - 5"},
        // ],
        // // User changed cell 1:1.
        // [
        //     row! {"3" | "5 - 3"  | "$0:0 + $0:1"},
        //     row! {"7" | "$0:0 + $2:2" | "$1:0 + $1:1"},
        //     row! {"1" | "13" | "$2:0 + $2:1 - 5"},
        // ],
    ]
    .to_vec();

    let raw_spreadsheet = RawSpreadsheet::new(&db, Default::default());

    queue.reverse();
    while let Some(cells) = queue.pop() {
        // Set new input.
        raw_spreadsheet.set_cells(&mut db).to(cells.to_vec());

        // Run queries.
        let (parsed_spreadsheet, diags) = db.parse_spreadsheet(raw_spreadsheet);
        eprintln!("Parser diags: {diags:?}");
        let solved_spreadsheet = db.solve_spreadsheet(parsed_spreadsheet);
        eprintln!("{solved_spreadsheet:?}");
    }
}
