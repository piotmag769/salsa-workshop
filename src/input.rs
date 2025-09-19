use salsa::Database;

#[salsa::input]
pub struct Spreadsheet {
    #[returns(ref)]
    pub cells: Vec<Vec<String>>,
}

#[salsa::tracked(returns(ref))]
pub fn spreadsheet_input(db: &dyn Database) -> Spreadsheet {
    Spreadsheet::new(db, Default::default())
}
