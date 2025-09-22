#[salsa::input]
pub struct RawSpreadsheet {
    #[returns(ref)]
    pub cells: Vec<Vec<String>>,
}
