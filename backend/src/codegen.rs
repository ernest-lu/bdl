use bdl_frontend::ast::{
    AssignmentExpr, BinOpExpr, Expr, FloatLiteral, FunctionDef, Identifier, IfExpr, IntegerLiteral,
    ListExpr, MethodCallExpr, PrintExpr, Program, RepExpr, ReturnExpr, StringLiteral, Type,
    UnOpExpr,
};
use crustal::{cpp, CppProgram};

pub struct CodeGenerator {
    program: CppProgram,
}

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {
            program: CppProgram::new(),
        }
    }

    pub fn generate(&mut self, ast: &Program) -> String {
        // Add standard includes
        self.program.add_include("bits/stdc++.h");

        // Generate code for each expression
        for expr in &ast.expressions {
            self.generate_expression(expr);
        }

        // Generate the main function if it doesn't exist
        if !ast
            .expressions
            .iter()
            .any(|e| matches!(e, Expr::FunctionDef(f) if f.name.value == "main"))
        {
            self.generate_default_main();
        }

        self.program.to_string()
    }

    fn generate_default_main(&mut self) {
        self.program.add_function(
            "main",
            cpp! {
                int main() {
                    return 0;
                }
            },
        );
    }

    fn generate_expression(&mut self, expr: &Expr) {
        match expr {
            Expr::Integer(int) => self.generate_integer(int),
            Expr::Float(float) => self.generate_float(float),
            Expr::String(string) => self.generate_string(string),
            Expr::Identifier(id) => self.generate_identifier(id),
            Expr::AssignmentExpr(assign) => self.generate_assignment(assign),
            Expr::MethodCallExpr(method) => self.generate_method_call(method),
            Expr::PrintExpr(print) => self.generate_print(print),
            Expr::IfExpr(if_expr) => self.generate_if(if_expr),
            Expr::RepExpr(rep) => self.generate_rep(rep),
            Expr::ListExpr(list) => self.generate_list(list),
            Expr::BinOp(binop) => self.generate_binop(binop),
            Expr::UnOp(unop) => self.generate_unop(unop),
            Expr::FunctionDef(func) => self.generate_function(func),
            Expr::ReturnExpr(ret) => self.generate_return(ret),
            Expr::NoneExpr(_) => {} // No-op for None expressions
        }
    }

    fn type_to_cpp_type(&self, t: &Type) -> String {
        match t {
            Type::Int => "int".to_string(),
            Type::Float => "double".to_string(),
            Type::String => "std::string".to_string(),
            Type::List(inner) => format!("std::vector<{}>", self.type_to_cpp_type(inner)),
            Type::Tuple(_) => "auto".to_string(), // Using auto for tuples for now
            Type::FunctionType(_, _) => "auto".to_string(), // Using auto for function types
            Type::None => "void".to_string(),
        }
    }

    fn generate_integer(&mut self, int: &IntegerLiteral) -> String {
        int.value.to_string()
    }

    fn generate_float(&mut self, float: &FloatLiteral) -> String {
        float.value.to_string()
    }

    fn generate_string(&mut self, string: &StringLiteral) -> String {
        format!("\"{}\"", string.value.replace("\"", "\\\""))
    }

    fn generate_identifier(&mut self, id: &Identifier) -> String {
        id.value.clone()
    }

    fn generate_print(&mut self, print: &PrintExpr) {
        let expr = match &*print.arg {
            Expr::String(s) => self.generate_string(s),
            Expr::Integer(i) => self.generate_integer(i),
            Expr::Float(f) => self.generate_float(f),
            Expr::Identifier(id) => self.generate_identifier(id),
            _ => "/* unsupported print expression */".to_string(),
        };

        self.program.add_statement(cpp! {
            std::cout << ${expr} << std::endl;
        });
    }

    fn generate_assignment(&mut self, assign: &AssignmentExpr) {
        let var_type = self.type_to_cpp_type(&assign.target.associated_type);
        let var_name = &assign.target.value.value;
        let value = self.generate_expr_value(&assign.value);

        self.program.add_statement(cpp! {
            ${var_type} ${var_name} = ${value};
        });
    }

    fn generate_expr_value(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::Integer(i) => self.generate_integer(i),
            Expr::Float(f) => self.generate_float(f),
            Expr::String(s) => self.generate_string(s),
            Expr::Identifier(id) => self.generate_identifier(id),
            Expr::BinOp(binop) => self.generate_binop_expr(binop),
            _ => "/* unsupported expression */".to_string(),
        }
    }

    fn generate_binop_expr(&mut self, binop: &BinOpExpr) -> String {
        let left = self.generate_expr_value(&binop.left);
        let right = self.generate_expr_value(&binop.right);
        format!("({} {} {})", left, binop.op, right)
    }

    fn generate_binop(&mut self, binop: &BinOpExpr) {
        let expr = self.generate_binop_expr(binop);
        self.program.add_statement(cpp! {
            ${expr};
        });
    }

    fn generate_unop(&mut self, unop: &UnOpExpr) {
        let arg = self.generate_expr_value(&unop.arg);
        let expr = format!("{}{}", unop.op, arg);
        self.program.add_statement(cpp! {
            ${expr};
        });
    }

    fn generate_function(&mut self, func: &FunctionDef) {
        let name = &func.name.value;
        let mut params = Vec::new();

        for arg in &func.args {
            let param_type = self.type_to_cpp_type(&arg.associated_type);
            let param_name = &arg.value.value;
            params.push(format!("{} {}", param_type, param_name));
        }

        let params_str = params.join(", ");

        self.program.add_function(
            name,
            cpp! {
                void ${name}(${params_str}) {
                    // Function body will go here
                }
            },
        );
    }

    fn generate_return(&mut self, ret: &ReturnExpr) {
        let value = self.generate_expr_value(&ret.value);
        self.program.add_statement(cpp! {
            return ${value};
        });
    }

    fn generate_if(&mut self, if_expr: &IfExpr) {
        let condition = self.generate_expr_value(&if_expr.condition);
        self.program.add_statement(cpp! {
            if (${condition}) {
                // Then block will go here
            }
        });

        if let Some(else_block) = &if_expr.else_block {
            self.program.add_statement(cpp! {
                else {
                    // Else block will go here
                }
            });
        }
    }

    fn generate_rep(&mut self, rep: &RepExpr) {
        let count = self.generate_expr_value(&rep.num_iterations);
        self.program.add_statement(cpp! {
            for(int i = 0; i < ${count}; i++) {
                // Loop body will go here
            }
        });
    }

    fn generate_list(&mut self, list: &ListExpr) {
        // For now, just generate a comment
        self.program.add_statement(cpp! {
            // List expression
        });
    }

    fn generate_method_call(&mut self, method: &MethodCallExpr) {
        let name = &method.method_name.value;
        let args: Vec<String> = method
            .args
            .iter()
            .map(|arg| self.generate_expr_value(arg))
            .collect();
        let args_str = args.join(", ");

        self.program.add_statement(cpp! {
            ${name}(${args_str});
        });
    }
}
