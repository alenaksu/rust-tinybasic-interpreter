pub struct Digit(pub i16);

pub struct Var(String);

#[derive(Debug)]
pub enum RelationOperator {
    Equal,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

#[derive(Debug)]
pub enum UnaryOperator {
    Plus,
    Minus,
}

#[derive(Debug)]
pub enum ArithmeticOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
pub struct UnaryExpression {
    pub operator: Option<UnaryOperator>,
    pub argument: Box<Expression>,
}

#[derive(Debug)]
pub struct BinaryExpression {
    pub operator: ArithmeticOperator,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Debug)]
pub struct VarDeclaration {
    pub name: String,
    pub value: Expression,
}

#[derive(Debug)]
pub enum Expression {
    UnaryExpression { expression: UnaryExpression },
    BinaryExpression { expression: BinaryExpression },
    Identifier { name: String },
    Number { value: usize },
}

#[derive(Debug)]
pub struct IfCondition {
    pub operator: RelationOperator,
    pub left: Expression,
    pub right: Expression,
}

#[derive(Debug)]
pub enum Statement {
    IfStatement {
        condition: IfCondition,
        then: Box<Statement>,
    },
    PrintStatement {
        expressions: Vec<Expression>,
    },
    VarStatement {
        declaration: VarDeclaration,
    },
    GoToStatement {
        location: Expression,
    },
    GoSubStatement {
        location: Expression,
    },
    ReturnStatement,
    EndStatement,
    Empty,
}

#[derive(Debug)]
pub struct Line {
    pub number: Option<usize>,
    pub statement: Statement,
}
