use crate::db::SpreadsheetDatabase;
use crate::input::raw::RawSpreadsheet;
use crate::parser::ParserGroup;
use salsa::{Cancelled, Database};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::thread::JoinHandle;

pub fn parse_cell_on_another_thread(
    db: SpreadsheetDatabase,
    raw_spreadsheet: RawSpreadsheet,
    row: usize,
    col: usize,
) -> JoinHandle<String> {
    let thread_closure = wrap_with_cancel_catcher(move || {
        todo!("parse single cell here and return formatted result")
    });

    std::thread::spawn(thread_closure)
}

fn wrap_with_cancel_catcher(func: impl FnOnce() -> String) -> impl FnOnce() -> String {
    || {
        catch_unwind(AssertUnwindSafe(|| func())).unwrap_or_else(|err| {
            let cancelled = err.downcast::<Cancelled>().unwrap();
            format!("{:?}", *cancelled)
        })
    }
}
