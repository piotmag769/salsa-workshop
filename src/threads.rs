use crate::db::SpreadsheetDatabase;
use crate::input::raw::RawSpreadsheet;
use crate::parser::ParserGroup;
use salsa::Cancelled;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::thread::JoinHandle;

pub fn parse_cell_on_another_thread(
    db: SpreadsheetDatabase,
    raw_spreadsheet: RawSpreadsheet,
    row: usize,
    col: usize,
) -> JoinHandle<String> {
    let thread_closure = wrap_with_cancel_catcher(move || {
        let parse_result = db
            .parse_single_cell(raw_spreadsheet, row, col)
            .map(|x| x.long(&db));
        format!("{parse_result:?}")
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
