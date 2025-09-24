mod db;
mod diagnostic;
mod input;
mod ir;
mod lexer;
mod parser;
mod solver;
mod threads;

use salsa::{Database, Setter};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::db::SpreadsheetDatabase;
use crate::input::file::{FileInput, FilePath, FilesOverrides};
use crate::parser::ParserGroup;
use crate::solver::SolverGroup;

macro_rules! row {
    ($($cell_str:literal) | *) => {
        vec![$($cell_str.to_string(),)*]
    };
}

fn main() {
    let mut db: SpreadsheetDatabase = Default::default();

    fs::copy("data/og.txt", "data/input.txt").unwrap();
    let files_overrides = FilesOverrides::new(&db, Default::default());

    // Run queries.
    run_excel(&db, file_path(&db), files_overrides);

    // Simulate a user writing to a file in an editor.
    files_overrides
        .set_overrides(&mut db)
        .to(HashMap::from_iter([(
            PathBuf::from("data/input.txt"),
            "2 | 4 | $0:0 + $0:1".to_string(),
        )]));

    // Run queries.
    run_excel(&db, file_path(&db), files_overrides);

    // Simulate a user saving a file.
    fs::write("data/input.txt", "2 | 4 | $0:0 + $0:1".to_string()).unwrap();
    files_overrides
        .set_overrides(&mut db)
        .to(Default::default());

    // Run queries.
    run_excel(&db, file_path(&db), files_overrides);
}

// To satisfy the borrow checker since FilePath holds db lifetime.
fn file_path<'db>(db: &'db dyn Database) -> FilePath<'db> {
    FilePath::new(db, PathBuf::from("data/input.txt"))
}

fn run_excel(db: &dyn Database, file_path: FilePath, files_overrides: FilesOverrides) {
    // Run queries.
    let raw_spreadsheet = db
        .spreadsheet_from_file(file_path, files_overrides)
        .unwrap();
    let (parsed_spreadsheet, diags) = db.parse_spreadsheet(raw_spreadsheet);
    if !diags.is_empty() {
        eprintln!("Parser diags: {diags:#?}");
    }
    let (solved_spreadsheet, diags) = db.solve_spreadsheet(parsed_spreadsheet);
    if !diags.is_empty() {
        eprintln!("Solver diags: {diags:#?}");
    }
    eprintln!("{solved_spreadsheet:?}");
}
