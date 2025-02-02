use bdl_frontend::ast::{
    AssignmentExpr, BinOpExpr, Expr, FloatLiteral, FunctionDef, Identifier, IfExpr, IntegerLiteral,
    ListExpr, MethodCallExpr, PrintExpr, ReassignmentExpr, RepExpr, ReturnExpr, StringLiteral,
    Type, UnOpExpr,
};

use crustal as CG;
use std::fmt::Write;
use std::path::Path;

pub fn generate(ast: &bdl_frontend::ast::Program) -> String {
    // Add standard includes
    let mut scope = CG::Scope::new();
    scope.new_include("bits/stdc++.h", true);

    // The language only generates a singular main function.
    // Everything is pushed into the main function from here
    let main_fn = scope.new_function("main", CG::Type::new_int32());
    let body = main_fn.body();

    // Generate code for each expression
    for expr in &ast.expressions {
        process_expression(body, expr);
    }
    main_fn.to_string()
}

enum ExprResult {
    integer(i64),
    float(f64),
    string(String),
    list(Vec<ExprResult>),
    none,
}

// optionally returns a CG::Expression
fn process_expression(context: &mut CG::Block, expr: &Expr) -> Option<CG::Expr> {
    match expr {
        // doesn't do anything
        // TODO: Support different integers
        Expr::Integer(i) => Some(CG::Expr::new_num(i.value as u64)),
        Expr::AssignmentExpr(assign) => {
            generate_assignment(context, assign);
            None
        }
        Expr::ReassignmentExpr(reassign) => {
            generate_reassignment(context, reassign);
            None
        }
        Expr::MethodCallExpr(method) => {
            generate_method_call(context, method);
            None
        }
        Expr::PrintExpr(print) => {
            generate_print(context, print);
            None
        }
        Expr::IfExpr(if_expr) => {
            generate_if(context, if_expr);
            None
        }
        Expr::RepExpr(rep) => {
            generate_rep(context, rep);
            None
        }
        Expr::Identifier(id) => Some(generate_identifier(context, id)),
        Expr::ListExpr(list) => Some(generate_list_expr(context, list)),
        Expr::BinOp(binop) => Some(generate_binop(context, binop)),
        Expr::UnOp(unop) => Some(generate_unop(context, unop)),
        Expr::FunctionDef(func) => todo!(),
        Expr::ReturnExpr(ret) => todo!(),
        _ => todo!(),
    }
}

fn get_string_type(t: &Type) -> String {
    match t {
        Type::Int => CG::Type::new_int32().to_string(),
        Type::String => CG::Type::new_std_string().to_string(),
        Type::List(t) => CG::Type::new(CG::BaseType::TemplateClass(
            "std::vector".to_string(),
            vec![get_string_type(t)],
        ))
        .to_string(),
        _ => todo!(),
    }
}

fn get_crustal_type(t: &Type) -> CG::Type {
    match t {
        Type::Int => CG::Type::new_int32(),
        Type::String => CG::Type::new_std_string(),
        Type::List(t) => CG::Type::new(CG::BaseType::TemplateClass(
            "std::vector".to_string(),
            vec![get_string_type(t)],
        )),
        _ => todo!(),
    }
}

fn generate_assignment(context: &mut CG::Block, assign: &AssignmentExpr) {
    let var = context.new_variable(
        &assign.target.value.value,
        get_crustal_type(&assign.target.associated_type),
    );
    let expr = var.to_expr();
    let rhs = process_expression(context, &assign.value).unwrap();
    context.assign(expr, rhs);
}

fn generate_reassignment(context: &mut CG::Block, assign: &ReassignmentExpr) {
    let var_expr = CG::Expr::new_var(&assign.target.value, CG::Type::new_void());
    let rhs = process_expression(context, &assign.value).unwrap();
    context.assign(var_expr, rhs);
}

fn generate_method_call(context: &mut CG::Block, call: &MethodCallExpr) {
    let obj_name = &call.method_name.value;
    let args_expr = call
        .args
        .iter()
        .map(|arg| process_expression(context, arg).unwrap())
        .collect();
    context.fn_call(obj_name, args_expr);
}

fn generate_print(context: &mut CG::Block, print: &PrintExpr) {
    let expr = process_expression(context, &print.arg).unwrap();
    // TODO: Print string
}

fn generate_if(context: &mut CG::Block, if_expr: &IfExpr) {
    let cond = process_expression(context, &if_expr.condition).unwrap();
    let if_else_expr = context.new_ifelse(&cond);

    let then_block = if_else_expr.then_branch();
    for expr in if_expr.then_block.iter() {
        process_expression(then_block, expr).unwrap();
    }
    let else_block = if_else_expr.other_branch();
    if let Some(else_vec) = &if_expr.else_block {
        for expr in else_vec.iter() {
            process_expression(else_block, expr).unwrap();
        }
    }
}

fn generate_variable_name() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let length = rng.gen_range(5..10);
    let mut name = String::with_capacity(length);
    name.push(rng.gen_range(b'a'..=b'z') as char);
    for _ in 1..length {
        name.push(rng.gen_range(b'a'..=b'z') as char);
    }
    name
}

fn generate_rep(context: &mut CG::Block, rep: &RepExpr) {
    let cond = process_expression(context, &rep.num_iterations).unwrap();
    // assume integer for now
    let var_name = generate_variable_name();
    let var_expr = context
        .new_variable(&var_name, CG::Type::new_int32())
        .to_expr();

    // This is very stupid
    let var_expr_copy = var_expr.clone();
    let var_expr_copy_2 = var_expr.clone();
    let var_expr_copy_3 = var_expr.clone();

    context.assign(var_expr, CG::Expr::new_num(0));
    let cond_expr = CG::Expr::binop(var_expr_copy, "<", cond);

    let while_loop = context.new_while_loop(&cond_expr);
    let while_loop_body = while_loop.body();
    for expr in rep.body.iter() {
        process_expression(while_loop_body, expr);
    }
    while_loop_body.assign(
        var_expr_copy_2,
        CG::Expr::binop(var_expr_copy_3, "+", CG::Expr::new_num(1)),
    );
}

fn generate_binop(context: &mut CG::Block, binop: &BinOpExpr) -> CG::Expr {
    let left = process_expression(context, &binop.left).unwrap();
    let right = process_expression(context, &binop.right).unwrap();
    let op = binop.op.as_str();
    CG::Expr::binop(left, op, right)
}

fn generate_unop(context: &mut CG::Block, unop: &UnOpExpr) -> CG::Expr {
    let expr = process_expression(context, &unop.arg).unwrap();
    let op = unop.op.as_str();
    CG::Expr::uop(op, expr)
}

fn generate_list_expr(context: &mut CG::Block, list: &ListExpr) -> CG::Expr {
    let cg_elems = list
        .elems
        .iter()
        .map(|expr| process_expression(context, expr).unwrap())
        .collect::<Vec<CG::Expr>>();

    CG::Expr::Raw(format!(
        "vector{{ {} }}",
        cg_elems
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    ))
}

fn generate_identifier(context: &mut CG::Block, id: &Identifier) -> CG::Expr {
    CG::Expr::Variable {
        name: id.value.clone(),
        ty: CG::Type::new_int32(),
    }
}
