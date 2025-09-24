use salsa::EventKind;

#[salsa::db]
#[derive(Clone)]
pub struct SpreadsheetDatabase {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for SpreadsheetDatabase {}

impl Default for SpreadsheetDatabase {
    fn default() -> Self {
        Self {
            storage: salsa::Storage::new(Some(Box::new({
                move |event| match event.kind {
                    EventKind::DidValidateMemoizedValue { .. }
                    | EventKind::WillBlockOn { .. }
                    | EventKind::WillExecute { .. }
                    | EventKind::WillIterateCycle { .. }
                    | EventKind::DidDiscard { .. }
                    | EventKind::DidSetCancellationFlag => {
                        eprintln!("{:?}", event.kind);
                    }

                    EventKind::DidDiscardAccumulated { .. }
                    | EventKind::WillCheckCancellation
                    | EventKind::DidInternValue { .. }
                    | EventKind::DidReuseInternedValue { .. }
                    | EventKind::DidValidateInternedValue { .. }
                    | EventKind::WillDiscardStaleOutput { .. } => {}
                }
            }))),
        }
    }
}
