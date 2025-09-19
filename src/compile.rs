use crate::input::spreadsheet_input;
use crate::ir::{Expr, ExprId, Op, StrId};
use crate::lexer::Lexer;
use salsa::Database;

pub fn parse_cell_contents<'db>(db: &'db dyn Database) -> Option<()> {
    let cells = spreadsheet_input(db).cells(db);
    for cell in cells.iter().flatten() {
        let str_id = StrId::new(db, cell);
        let expr_id = parse_cell_content(db, str_id);
        eprintln!("{:?}", expr_id.map(|x| x.long(db)));
        // TODO: Evaluate the expression.
        // evaluate_expr(db, expr_id);
    }

    Some(())
}

// #[salsa::tracked]
// fn evaluate_expr<'db>(db: &'db dyn Database, expr: ExprId<'db>) -> Option<u32> {
//     None
// }

#[salsa::tracked]
fn parse_cell_content<'db>(db: &'db dyn Database, cell_content: StrId<'db>) -> Option<ExprId<'db>> {
    let mut lexer = Lexer::new(&cell_content.long(db));

    let mut already_parsed_expr = None;
    let mut pending_op = None;

    while lexer.can_consume() {
        let current_expr = lexer
            // Try number
            .number()
            .map(|num| Expr::Number(num))
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
