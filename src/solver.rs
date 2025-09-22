use super::parser::{ParsedSpreadsheet, ParserGroup};
use crate::ir::{Expr, ExprId, Op, StrId};

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
            .map(|x| {
                x.iter()
                    .map(|expr_id| solve_expr(db, (*expr_id)?))
                    .collect()
            })
            .collect()
    }
}

impl<T: Database + ?Sized> SolverGroup for T {}

#[salsa::tracked]
fn solve_expr<'db>(db: &'db dyn Database, expr_id: ExprId<'db>) -> Option<u32> {
    match expr_id.long(db) {
        Expr::Number(num) => Some(*num),
        Expr::CellCords { row, col } => {
            let cell_content = &db.spreadsheet_input().cells(db)[*row as usize][*col as usize];
            let str_id = StrId::new(db, cell_content);
            let cell_content = db.parse_cell_content(str_id)?;
            solve_expr(db, cell_content)
        }
        Expr::Op(lhs, op, rhs) => {
            let lhs_val = solve_expr(db, *lhs)?;
            let rhs_val = solve_expr(db, *rhs)?;
            Some(match op {
                Op::Add => lhs_val + rhs_val,
                Op::Subtract => lhs_val - rhs_val,
            })
        }
    }
}
