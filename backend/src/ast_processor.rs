use bdl_frontend::ast::{
    Program, Expr, 
    IntegerLiteral, FloatLiteral, StringLiteral, Identifier,
    AssignmentExpr, MethodCallExpr, PrintExpr, IfExpr,
    RepExpr, ListExpr, BinOpExpr, UnOpExpr, FunctionDef, ReturnExpr
};

pub struct AstProcessor {
    program: Program,
}

impl AstProcessor {
    pub fn new(program: Program) -> Self {
        AstProcessor { program }
    }

    pub fn process(&self) {
        for expr in &self.program.expressions {
            self.process_expression(expr);
        }
    }

    fn process_expression(&self, expr: &Expr) {
        match expr {
            Expr::Integer(int) => self.process_integer(int),
            Expr::Float(float) => self.process_float(float),
            Expr::String(string) => self.process_string(string),
            Expr::Identifier(id) => self.process_identifier(id),
            Expr::AssignmentExpr(assign) => self.process_assignment(assign),
            Expr::MethodCallExpr(method) => self.process_method_call(method),
            Expr::PrintExpr(print) => self.process_print(print),
            Expr::IfExpr(if_expr) => self.process_if(if_expr),
            Expr::RepExpr(rep) => self.process_rep(rep),
            Expr::ListExpr(list) => self.process_list(list),
            Expr::BinOp(binop) => self.process_binop(binop),
            Expr::UnOp(unop) => self.process_unop(unop),
            Expr::FunctionDef(func) => self.process_function(func),
            Expr::NoneExpr(_) => self.process_none(),
            Expr::ReturnExpr(ret) => self.process_return(ret),
        }
    }

    fn process_integer(&self, int: &IntegerLiteral) {
        println!("Processing integer: {}", int.value);
    }

    fn process_float(&self, float: &FloatLiteral) {
        println!("Processing float: {}", float.value);
    }

    fn process_string(&self, string: &StringLiteral) {
        println!("Processing string: {}", string.value);
    }

    fn process_identifier(&self, id: &Identifier) {
        println!("Processing identifier: {}", id.value);
    }

    fn process_assignment(&self, assign: &AssignmentExpr) {
        println!("Processing assignment to: {}", assign.target.value.value);
        self.process_expression(&assign.value);
    }

    fn process_method_call(&self, method: &MethodCallExpr) {
        println!("Processing method call: {}", method.method_name.value);
        for arg in &method.args {
            self.process_expression(arg);
        }
    }

    fn process_print(&self, print: &PrintExpr) {
        println!("Processing print expression");
        self.process_expression(&print.arg);
    }

    fn process_if(&self, if_expr: &IfExpr) {
        println!("Processing if expression");
        self.process_expression(&if_expr.condition);
        for expr in &if_expr.then_block {
            self.process_expression(expr);
        }
        if let Some(else_block) = &if_expr.else_block {
            for expr in else_block {
                self.process_expression(expr);
            }
        }
    }

    fn process_rep(&self, rep: &RepExpr) {
        println!("Processing rep expression");
        self.process_expression(&rep.num_iterations);
        self.process_expression(&rep.body);
    }

    fn process_list(&self, list: &ListExpr) {
        println!("Processing list expression");
        for elem in &list.elems {
            self.process_expression(elem);
        }
    }

    fn process_binop(&self, binop: &BinOpExpr) {
        println!("Processing binary operation: {}", binop.op);
        self.process_expression(&binop.left);
        self.process_expression(&binop.right);
    }

    fn process_unop(&self, unop: &UnOpExpr) {
        println!("Processing unary operation: {}", unop.op);
        self.process_expression(&unop.arg);
    }

    fn process_function(&self, func: &FunctionDef) {
        println!("Processing function definition: {}", func.name.value);
        for arg in &func.args {
            println!("  Parameter: {} : {:?}", arg.value.value, arg.associated_type);
        }
        for expr in &func.body {
            self.process_expression(expr);
        }
    }

    fn process_none(&self) {
        println!("Processing none expression");
    }

    fn process_return(&self, ret: &ReturnExpr) {
        println!("Processing return expression");
        self.process_expression(&ret.value);
    }
}