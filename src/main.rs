mod db;
mod diagnostic;
mod input;
mod ir;
mod lexer;
mod parser;
mod solver;
mod threads;

use crate::db::SpreadsheetDatabase;
use crate::input::raw::RawSpreadsheet;
use crate::parser::ParserGroup;
use crate::solver::SolverGroup;
use crate::threads::parse_cell_on_another_thread;
use salsa::Setter;

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
            row! {"$0:2" | "5 - 3"  | "$0:1 + $0:0"},
            row! {"7" | "12" | "$1:0 + $1:1"},
            row! {"1" | "13" | "$2:0 + $2:1"},
        ],
        // [
        //     row! {"3" | "5 - 3"  | "$0:0 + $0:1"},
        //     row! {"7" | "12" | "$1:0 + $1:1"},
        //     row! {"420" | "13" | "$2:0 + $2:1 - 5"},
        // ],
    ]
    .to_vec();

    let raw_spreadsheet = RawSpreadsheet::new(&db, Default::default());

    queue.reverse();
    while let Some(cells) = queue.pop() {
        // Set new input.
        raw_spreadsheet.set_cells(&mut db).to(cells.to_vec());

        let mut handles = Vec::new();
        for _ in std::iter::repeat_n((), 3) {
            let db_clone = db.clone();
            let handle = parse_cell_on_another_thread(db_clone, raw_spreadsheet, 0, 0);

            handles.push(handle);
        }

        for handle in handles {
            eprintln!("{}", handle.join().unwrap());
        }
    }
}
