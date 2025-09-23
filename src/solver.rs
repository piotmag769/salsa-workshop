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

fn solve_expr<'db>(
    db: &'db dyn Database,
    expr_id: ExprId<'db>,
    parsed_spreadsheet: ParsedSpreadsheet<'db>,
) -> Option<u32> {
    todo!("Implement me");
}
