#[salsa::accumulator]
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Diagnostic {
    pub row: usize,
    pub col: usize,
    pub message: String,
}
