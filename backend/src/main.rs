mod ast_processor;

use bdl_frontend::ast::{Program, Expr, IntegerLiteral, PrintExpr};
use ast_processor::AstProcessor;

fn main() {
    // Create a simple test program that prints a number
    let test_program = Program {
        expressions: vec![
            Expr::PrintExpr(PrintExpr::new(
                Expr::Integer(IntegerLiteral::new(42))
            ))
        ],
    };

    // Create and run the processor
    let processor = AstProcessor::new(test_program);
    processor.process();
}
