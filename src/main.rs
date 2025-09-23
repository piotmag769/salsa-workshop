mod db;
mod input;
mod ir;
mod lexer;
mod parser;
mod solver;

use salsa::Setter;
use std::path::PathBuf;

use crate::db::SpreadsheetDatabase;
use crate::input::file::{FileInput, FilePath};
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
            row! {"3" | "5"  | "$0:0 + $0:1"},
            // row! {"7" | "12" | "$1:0 + $1:1"},
            // row! {"1" | "13" | "$2:0 + $2:1"},
        ],
        // User changed cell 0:1.
        [
            row! {"3" | "5 - 3"  | "$0:0 + $0:1"},
            // row! {"7" | "12" | "$1:0 + $1:1"},
            // row! {"1" | "13" | "$2:0 + $2:1"},
        ],
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

    let file_path = FilePath::new(&db, PathBuf::from(""));
    let mut queue = ["data/input.txt", "data/different_input.txt"].to_vec();

    // let raw_spreadsheet = RawSpreadsheet::new(&db, Default::default());

    queue.reverse();
    while let Some(path) = queue.pop() {
        // Set new input.
        file_path.set_path_buf(&mut db).to(PathBuf::from(path));
        // raw_spreadsheet.set_cells(&mut db).to(cells.to_vec());

        // Run queries.
        let raw_spreadsheet = db.spreadsheet_from_file(file_path).unwrap();
        let parsed_spreadsheet = db.parse_spreadsheet(raw_spreadsheet);
        let solved_spreadsheet = db.solve_spreadsheet(parsed_spreadsheet);

        print_spreadsheet(solved_spreadsheet);
    }
}

fn print_spreadsheet(solved_spreadsheet: Vec<Vec<Option<u32>>>) {
    eprintln!("[");
    for row in solved_spreadsheet {
        eprintln!("  {row:?}");
    }
    eprintln!("]");
}
