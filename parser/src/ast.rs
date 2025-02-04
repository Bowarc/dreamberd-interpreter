use std::collections::HashMap;
#[derive(Debug)]
pub enum Statement {
    Empty,
    // Bloc(Box<Node>),
    Assignment(AssignmentExpression),
    Expression(Expression),
    FunctionSignature(FunctionSignature),
    If(IfStatement),
    While(WhileStatement),
    Return(ReturnStatement)

    // FunctionSignature {
    //     name: String,
    //     args: HashMap<String, types::DreamberdType>,
    //     return_type: Option<types::DreamberdType>,
    // },
    // Expr(Vec<lexer::Token>)
}

#[derive(Debug)]
pub struct AssignmentExpression {
    pub outer_mutability: bool,
    pub inner_mutability: bool,
    pub global: bool,
    pub global_mutability: bool,
    pub name: String,
    pub _type: types::DreamberdType,
    pub value: Expression,
}

#[derive(Debug)]
pub enum Expression{
    Variable(Variable),
    // Unary(UnaryExpression),
    Litteral(Litteral),
    Binary(Binary),
    Call(Call),
}

#[derive(Debug)]
pub struct Variable{
    pub name: String,
    pub _type: types::DreamberdType
}

#[derive(Debug)]
pub enum Litteral{
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
}

#[derive(Debug)]
pub struct Binary{
    pub lhs: Box<Expression>,
    pub operator: Operator,
    pub rhs: Box<Expression>
}
#[derive(Debug)]
pub enum Operator{
    Plus,Minus,Mul,Div,Mod, //..
}
#[derive(Debug)]
pub struct Call{
    pub name: String,
    pub arguments: Vec<Expression>
}

#[derive(Debug)]
pub struct FunctionSignature{
    pub name: String,
    pub args: HashMap<String, types::DreamberdType>,
    pub return_type: Option<types::DreamberdType>
}

#[derive(Debug)]
pub struct IfStatement{
    pub condition: Expression,
    pub then_branch: Box<Statement>,
    pub else_branch: Option<Box<Statement>>
}

#[derive(Debug)]
pub struct WhileStatement{
    pub condition: Expression,
    pub body: Box<Statement>,
}

#[derive(Debug)]
pub struct ReturnStatement{
    pub expr: Option<Expression>
}
