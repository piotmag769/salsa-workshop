use salsa::Database;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::input::raw::RawSpreadsheet;

pub trait FileInput: Database {
    fn spreadsheet_from_file<'db>(
        &'db self,
        file: FilePath<'db>,
        files_overrides: FilesOverrides,
    ) -> Option<RawSpreadsheet> {
        spreadsheet_from_file(self.as_dyn_database(), file, files_overrides)
    }
}

impl<T: Database + ?Sized> FileInput for T {}

#[salsa::input]
pub struct FilesOverrides {
    #[returns(ref)]
    pub overrides: HashMap<PathBuf, String>,
}

#[salsa::interned]
pub struct FilePath {
    #[returns(ref)]
    pub path_buf: PathBuf,
}

#[salsa::tracked]
fn spreadsheet_from_file<'db>(
    db: &'db dyn Database,
    file: FilePath<'db>,
    files_overrides: FilesOverrides,
) -> Option<RawSpreadsheet> {
    let content = if let Some(file_override) = files_overrides.overrides(db).get(file.path_buf(db))
    {
        file_override.clone()
    } else {
        file_content(db, file)?
    };

    let mut cells = Vec::new();
    for row in content.split("\n") {
        // Don't handle bad inputs bcs we don't care about it in this task.
        let row: Vec<_> = row.split("|").map(ToString::to_string).collect();
        cells.push(row);
    }

    Some(RawSpreadsheet::new(db, cells))
}

#[salsa::tracked]
fn file_content<'db>(db: &'db dyn Database, file: FilePath<'db>) -> Option<String> {
    // Report that the query depends on external input.
    // Each time a revision changes, this query will be reexecuted.
    // TODO: try commenting it out after completing the TODO task in main.rs.
    db.report_untracked_read();

    std::fs::read_to_string(file.path_buf(db)).ok()
}
