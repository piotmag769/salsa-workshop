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
                move |event| {
                    // eprintln!("Event: {event:?}");
                    let mut logs = logs.lock().unwrap();
                    logs.push(format!("Event: {:?}", event.kind))
                }
            }))),
            logs,
        }
    }
}

impl Drop for SpreadsheetDatabase {
    fn drop(&mut self) {
        let logs = self.logs.lock().unwrap();
        eprintln!("{:#?}", logs)
    }
}
