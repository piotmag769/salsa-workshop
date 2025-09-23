use crate::input::raw::RawSpreadsheet;
use salsa::Database;
use std::path::PathBuf;

pub trait FileInput: Database {
    fn spreadsheet_from_file(&self, file: FilePath) -> Option<RawSpreadsheet> {
        spreadsheet_from_file(self.as_dyn_database(), file)
    }
}

impl<T: Database + ?Sized> FileInput for T {}

#[salsa::tracked]
fn spreadsheet_from_file<'db>(db: &'db dyn Database, file: FilePath) -> Option<RawSpreadsheet> {
    // Report that the query depends on external input.
    // Each time an input changes, this query will be reexecuted.
    db.report_untracked_read();

    let content = std::fs::read_to_string(file.path_buf(db)).ok()?;
    let mut cells = Vec::new();
    for row in content.split("\n") {
        // Don't handle bad inputs bcs we don't care.
        let row: Vec<_> = row.split("|").map(ToString::to_string).collect();
        cells.push(row);
    }

    Some(RawSpreadsheet::new(db, cells))
}

#[salsa::input]
pub struct FilePath {
    #[returns(ref)]
    pub path_buf: PathBuf,
}
