use salsa::EventKind;
use std::sync::{Arc, Mutex};

#[salsa::db]
#[derive(Clone)]
pub struct SpreadsheetDatabase {
    storage: salsa::Storage<Self>,
    logs: Arc<Mutex<Vec<String>>>,
}

impl salsa::Database for SpreadsheetDatabase {}

impl Default for SpreadsheetDatabase {
    fn default() -> Self {
        let logs = <Arc<Mutex<Vec<String>>>>::default();
        Self {
            storage: salsa::Storage::new(Some(Box::new({
                let logs = logs.clone();
                move |event| match event.kind {
                    EventKind::DidValidateMemoizedValue { .. }
                    | EventKind::WillBlockOn { .. }
                    | EventKind::WillExecute { .. }
                    | EventKind::WillIterateCycle { .. }
                    | EventKind::DidDiscard { .. }
                    | EventKind::DidSetCancellationFlag
                    | EventKind::WillCheckCancellation => {
                        eprintln!("{:?}", event.kind);
                        let mut logs = logs.lock().unwrap();
                        logs.push(format!("{:?}", event.kind))
                    }

                    EventKind::DidDiscardAccumulated { .. }
                    | EventKind::DidInternValue { .. }
                    | EventKind::DidReuseInternedValue { .. }
                    | EventKind::DidValidateInternedValue { .. } => {}

                    EventKind::WillDiscardStaleOutput { .. } => {}
                }
            }))),
            logs,
        }
    }
}

impl Drop for SpreadsheetDatabase {
    fn drop(&mut self) {
        let _logs = self.logs.lock().unwrap();
        // eprintln!("{:#?}", _logs)
    }
}
