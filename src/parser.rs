use salsa::Database;

use crate::input::RawSpreadsheet;
use crate::ir::{Expr, ExprId, Op, StrId};
use crate::lexer::Lexer;

pub trait ParserGroup: Database {
    fn parse_spreadsheet<'db>(&'db self) -> ParsedSpreadsheet<'db> {
        parse_spreadsheet(self.as_dyn_database())
    }

    fn parse_cell_content<'db>(&'db self, cell_content: StrId<'db>) -> Option<ExprId<'db>> {
        parse_cell_content(self.as_dyn_database(), cell_content)
    }

    fn spreadsheet_input(&self) -> RawSpreadsheet {
        spreadsheet_input(self.as_dyn_database())
    }
}

impl<T: Database + ?Sized> ParserGroup for T {}

// A tracked query for getting input.
#[salsa::tracked]
fn spreadsheet_input(db: &dyn Database) -> RawSpreadsheet {
    RawSpreadsheet::new(db, Default::default())
}

#[salsa::tracked]
pub struct ParsedSpreadsheet<'db> {
    #[tracked]
    #[returns(ref)]
    pub cells: Vec<Vec<Option<ExprId<'db>>>>,
}

#[salsa::tracked]
fn parse_spreadsheet<'db>(db: &'db dyn Database) -> ParsedSpreadsheet<'db> {
    let raw_cells = spreadsheet_input(db).cells(db);

    let parsed_cells = raw_cells
        .iter()
        .map(|x| {
            x.iter()
                .map(|cell| {
                    let str_id = StrId::new(db, cell);
                    parse_cell_content(db, str_id)
                })
                .collect()
        })
        .collect();

    ParsedSpreadsheet::new(db, parsed_cells)
}

#[salsa::tracked]
fn parse_cell_content<'db>(db: &'db dyn Database, cell_content: StrId<'db>) -> Option<ExprId<'db>> {
    let mut lexer = Lexer::new(cell_content.long(db));

    let mut already_parsed_expr = None;
    let mut pending_op = None;

    while lexer.can_consume() {
        let current_expr = lexer
            // Try number
            .number()
            .map(Expr::Number)
            // Try cell cords if there was no number.
            .or_else(|| {
                lexer
                    .cell_cords()
                    .map(|(row, col)| Expr::CellCords { row, col })
            });

        if let Some(current_expr) = current_expr {
            already_parsed_expr = Some(merge_expressions(
                already_parsed_expr,
                pending_op,
                current_expr,
                db,
            )?);
            // If there was op it was appended to already_parsed_expr.
            pending_op = None;
            continue;
        } else if let Some(op) = lexer.op() {
            // Expr cannot start with op.
            if already_parsed_expr.is_none() {
                // TODO: add diagnostic
                return None;
            }

            if pending_op.is_none() {
                pending_op = Some(op);
            } else {
                // Expr cannot have two consecutive ops.
                // TODO: add diagnostic
                return None;
            }
            continue;
        }

        break;
    }

    // Expr cannot end with op.
    if pending_op.is_some() {
        None
    } else {
        already_parsed_expr.map(|expr| ExprId::new(db, expr))
    }
}

fn merge_expressions<'db>(
    maybe_lhs: Option<Expr<'db>>,
    maybe_op: Option<Op>,
    expr_to_append: Expr<'db>,
    db: &'db dyn Database,
) -> Option<Expr<'db>> {
    match maybe_lhs {
        None => Some(expr_to_append),
        Some(expr) => match maybe_op {
            Some(op) => {
                let lhs = ExprId::new(db, expr);
                let rhs = ExprId::new(db, expr_to_append);
                Some(Expr::Op(lhs, op, rhs))
            }
            None => None,
        },
    }
}
