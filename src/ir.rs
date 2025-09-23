#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub enum Expr<'db> {
    Op(ExprId<'db>, Op, ExprId<'db>),
    CellCords { row: u32, col: u32 },
    Number(u32),
}

#[salsa::interned(debug)]
pub struct ExprId<'db> {
    #[returns(ref)]
    pub long: Expr<'db>,
}

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub enum Op {
    Add,
    Subtract,
}

#[salsa::interned]
pub struct StrId<'db> {
    #[returns(ref)]
    pub long: String,
}
