mod codegen;

use bdl_frontend::ast::{Expr, IntegerLiteral, PrintExpr, Program};
use codegen::generate;

fn main() {
    // Create a simple test program that prints a number
    let test_program = Program {
        expressions: vec![Expr::PrintExpr(PrintExpr::new(Expr::Integer(
            IntegerLiteral::new(42),
        )))],
    };

    // Create and run the processor
    let program = generate(&test_program);

    println!("{}", program);
}
