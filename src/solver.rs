use super::parser::ParsedSpreadsheet;
use crate::ir::{Expr, ExprId, Op};

use crate::diagnostic::Diagnostic;
use salsa::{Accumulator, Database};

pub trait SolverGroup: Database {
    fn solve_spreadsheet<'db>(
        &'db self,
        parsed_spreadsheet: ParsedSpreadsheet<'db>,
    ) -> (Vec<Vec<Option<u32>>>, Vec<Diagnostic>) {
        let solved = solve_spreadsheet(self.as_dyn_database(), parsed_spreadsheet);
        let diags = solve_spreadsheet::accumulated::<Diagnostic>(
            self.as_dyn_database(),
            parsed_spreadsheet,
        );
        (solved, diags.into_iter().cloned().collect())
    }
}

impl<T: Database + ?Sized> SolverGroup for T {}

#[salsa::tracked]
fn solve_spreadsheet<'db>(
    db: &'db dyn Database,
    parsed_spreadsheet: ParsedSpreadsheet<'db>,
) -> Vec<Vec<Option<u32>>> {
    parsed_spreadsheet
        .cells(db)
        .iter()
        .enumerate()
        .map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .map(|(col_index, expr_id)| {
                    match solve_expr(db, (*expr_id)?, parsed_spreadsheet) {
                        Ok(val) => Some(val),
                        Err(msg) => {
                            // Do not pass row_index and col_index to
                            // `solve_expr` for efficiency.
                            // Accumulate a diagnostic here instead.
                            Diagnostic {
                                row: row_index,
                                col: col_index,
                                message: msg,
                            }
                            .accumulate(db);
                            None
                        }
                    }
                })
                .collect()
        })
        .collect()
}

#[salsa::tracked(cycle_result=solve_expr_handle_cycle)]
fn solve_expr<'db>(
    db: &'db dyn Database,
    expr_id: ExprId<'db>,
    parsed_spreadsheet: ParsedSpreadsheet<'db>,
) -> Result<u32, String> {
    match expr_id.long(db) {
        Expr::Number(num) => Ok(*num),
        Expr::CellCords { row, col } => {
            let cell_content = parsed_spreadsheet.cells(db)[*row as usize][*col as usize]
                .ok_or_else(|| "Cell depending on unparsed cells".to_string())?;
            solve_expr(db, cell_content, parsed_spreadsheet)
        }
        Expr::Op(lhs, op, rhs) => {
            let lhs_val = solve_expr(db, *lhs, parsed_spreadsheet)?;
            let rhs_val = solve_expr(db, *rhs, parsed_spreadsheet)?;
            Ok(match op {
                Op::Add => lhs_val + rhs_val,
                Op::Subtract => lhs_val - rhs_val,
            })
        }
    }
}

/// Return `None` since we cannot solve in the case of a cycle.
fn solve_expr_handle_cycle<'db>(
    _db: &'db dyn Database,
    _expr_id: ExprId<'db>,
    _parsed_spreadsheet: ParsedSpreadsheet<'db>,
) -> Result<u32, String> {
    Err("Cycle detected".to_string())
}
