#[derive(Debug, Clone)]
pub struct Program {
    pub expressions: Vec<Expr>,
}

// ast next!

#[derive(Debug, Clone)]
pub struct FunctionDef {
    pub name: Identifier,
    pub args: Vec<TypedIdentifier>,
    pub body: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub enum Type {
    Int,
    Float,
    String,
    List(Box<Type>),
    Tuple(Box<Type>),
    // type of parameters, return type
    FunctionType(Vec<Type>, Box<Option<Type>>),
    None,
}

trait ExprNode {
    fn get_type(&self) -> Type;
}

#[derive(Debug, Clone)]
pub struct TypedIdentifier {
    pub value: Identifier,
    pub associated_type: Type,
}

#[derive(Debug, Clone)]
pub struct IntegerLiteral {
    pub value: i128,
}

#[derive(Debug, Clone)]
pub struct FloatLiteral {
    pub value: f64,
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct AssignmentExpr {
    pub target: TypedIdentifier,
    pub value: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct MethodCallExpr {
    pub method_name: Identifier,
    pub args: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub struct PrintExpr {
    pub arg: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct IfExpr {
    pub condition: Box<Expr>,
    pub then_block: Vec<Expr>,
    pub else_block: Option<Vec<Expr>>,
}

#[derive(Debug, Clone)]
pub struct RepExpr {
    pub num_iterations: Box<Expr>,
    pub body: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct BinOpExpr {
    pub left: Box<Expr>,
    pub op: String,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct UnOpExpr {
    pub op: String,
    pub arg: Box<Expr>,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Integer(IntegerLiteral),
    Float(FloatLiteral),
    Identifier(Identifier),
    AssignmentExpr(AssignmentExpr),
    MethodCallExpr(MethodCallExpr),
    PrintExpr(PrintExpr),
    IfExpr(IfExpr),
    RepExpr(RepExpr),
    BinOp(BinOpExpr),
    UnOp(UnOpExpr),
    FunctionDef(FunctionDef),
    NoneExpr(NoneExpr),
}

#[derive(Debug, Clone)]
pub struct NoneExpr;

pub enum AstNode {
    Program(Program),
    Expr(Expr),
}

impl Expr {
    pub fn get_type(&self) -> Type {
        match self {
            Expr::Integer(_) => Type::Int,
            Expr::Float(_) => Type::Float,
            Expr::Identifier(_) => todo!(),
            Expr::AssignmentExpr(_) => todo!(),
            Expr::MethodCallExpr(_) => todo!(),
            Expr::PrintExpr(_) => todo!(),
            Expr::IfExpr(_) => todo!(),
            Expr::RepExpr(_) => todo!(),
            Expr::BinOp(_) => todo!(),
            Expr::UnOp(expr) => expr.arg.get_type(),
            Expr::FunctionDef(fd) => todo!(),
            Expr::NoneExpr(_) => Type::None,
        }
    }
}

impl AstNode {
    pub fn get_type(&self) -> Type {
        match self {
            AstNode::Program(program) => program
                .expressions
                .last()
                .unwrap_or(&Expr::NoneExpr(NoneExpr))
                .get_type(),
            AstNode::Expr(expr) => expr.get_type(),
        }
    }
}
