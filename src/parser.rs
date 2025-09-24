use crate::diagnostic::Diagnostic;
use crate::input::raw::RawSpreadsheet;
use crate::ir::{Expr, ExprId, Op, StrId};
use crate::lexer::Lexer;
use salsa::{Accumulator, Database};

pub trait ParserGroup: Database {
    fn parse_spreadsheet<'db>(
        &'db self,
        raw_spreadsheet: RawSpreadsheet,
    ) -> (ParsedSpreadsheet<'db>, Vec<Diagnostic>) {
        let parsed = parse_spreadsheet(self.as_dyn_database(), raw_spreadsheet);
        let diags =
            parse_spreadsheet::accumulated::<Diagnostic>(self.as_dyn_database(), raw_spreadsheet);
        (parsed, diags.into_iter().cloned().collect())
    }
}

impl<T: Database + ?Sized> ParserGroup for T {}

#[salsa::tracked]
pub struct ParsedSpreadsheet<'db> {
    #[tracked]
    #[returns(ref)]
    pub cells: Vec<Vec<Option<ExprId<'db>>>>,
}

#[salsa::tracked]
fn parse_spreadsheet<'db>(
    db: &'db dyn Database,
    raw_spreadsheet: RawSpreadsheet,
) -> ParsedSpreadsheet<'db> {
    let raw_cells = raw_spreadsheet.cells(db);

    let parsed_cells: Vec<Vec<_>> = raw_cells
        .iter()
        .enumerate()
        .map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .map(|(col_index, cell)| {
                    let str_id = StrId::new(db, cell);
                    match parse_cell_content(db, str_id) {
                        Ok(expr) => Some(expr),
                        Err(msg) => {
                            // Do not pass row_index and col_index to
                            // `parse_cell_content` for efficiency.
                            // Accumulate a diagnostic here instead.
                            Diagnostic {
                                row: row_index,
                                col: col_index,
                                message: format!("{msg}: [{cell}]"),
                            }
                            .accumulate(db);
                            None
                        }
                    }
                })
                .collect()
        })
        .collect();

    ParsedSpreadsheet::new(db, parsed_cells)
}

#[salsa::tracked]
fn parse_cell_content<'db>(
    db: &'db dyn Database,
    cell_content: StrId<'db>,
) -> Result<ExprId<'db>, String> {
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
                return Err("Expr cannot start with op".to_string());
            }

            if pending_op.is_none() {
                pending_op = Some(op);
            } else {
                // Two consecutive ops.
                return Err("Two consecutive ops are disallowed".to_string());
            }
            continue;
        }

        break;
    }

    // Cell was not fully parsed.
    if lexer.can_consume() {
        Err("Cell could not be fully parsed".to_string())
    }
    // Expr cannot end with op.
    else if pending_op.is_some() {
        Err(" Expr cannot end with op".to_string())
    } else {
        already_parsed_expr
            .map(|expr| ExprId::new(db, expr))
            .ok_or_else(|| "An empty cell.".to_string())
    }
}

fn merge_expressions<'db>(
    maybe_lhs: Option<Expr<'db>>,
    maybe_op: Option<Op>,
    expr_to_append: Expr<'db>,
    db: &'db dyn Database,
) -> Result<Expr<'db>, String> {
    match maybe_lhs {
        None => Ok(expr_to_append),
        Some(expr) => match maybe_op {
            Some(op) => {
                let lhs = ExprId::new(db, expr);
                let rhs = ExprId::new(db, expr_to_append);
                Ok(Expr::Op(lhs, op, rhs))
            }
            // Two consecutive expressions without an operand between them.
            None => Err(
                "Two consecutive expressions without an operand between them are disallowed"
                    .to_string(),
            ),
        },
    }
}
