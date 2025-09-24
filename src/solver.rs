use super::parser::ParsedSpreadsheet;
use crate::ir::{Expr, ExprId, Op};

use salsa::Database;

pub trait SolverGroup: Database {
    fn solve_spreadsheet<'db>(
        &'db self,
        parsed_spreadsheet: ParsedSpreadsheet<'db>,
    ) -> Vec<Vec<Option<u32>>> {
        let db = self.as_dyn_database();
        parsed_spreadsheet
            .cells(db)
            .iter()
            .map(|row| {
                row.iter()
                    .map(|expr_id| solve_expr(db, (*expr_id)?, parsed_spreadsheet))
                    .collect()
            })
            .collect()
    }
}

impl<T: Database + ?Sized> SolverGroup for T {}

#[salsa::tracked(cycle_result=solve_expr_handle_cycle)]
fn solve_expr<'db>(
    db: &'db dyn Database,
    expr_id: ExprId<'db>,
    parsed_spreadsheet: ParsedSpreadsheet<'db>,
) -> Option<u32> {
    match expr_id.long(db) {
        Expr::Number(num) => Some(*num),
        Expr::CellCords { row, col } => {
            let cell_content = parsed_spreadsheet.cells(db)[*row as usize][*col as usize]?;
            solve_expr(db, cell_content, parsed_spreadsheet)
        }
        Expr::Op(lhs, op, rhs) => {
            let lhs_val = solve_expr(db, *lhs, parsed_spreadsheet)?;
            let rhs_val = solve_expr(db, *rhs, parsed_spreadsheet)?;
            Some(match op {
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
) -> Option<u32> {
    todo!("Diagnostic");
    None
}
