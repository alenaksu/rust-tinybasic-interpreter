#[derive(Debug, Clone, PartialEq)]
pub enum RelationOperator {
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Plus,
    Minus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArithmeticOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpression {
    pub operator: Option<UnaryOperator>,
    pub argument: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpression {
    pub operator: ArithmeticOperator,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarDeclaration {
    pub name: String,
    pub value: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Number { value: usize },
    String { value: String },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    UnaryExpression(UnaryExpression),
    BinaryExpression(BinaryExpression),
    Identifier(Identifier),
    Literal(Literal),
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfCondition {
    pub operator: RelationOperator,
    pub left: Expression,
    pub right: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    IfStatement {
        condition: IfCondition,
        then: Box<Statement>,
    },
    PrintStatement {
        expressions: Vec<Expression>,
    },
    InputStatement {
        variables: Vec<Identifier>,
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
    NewStatement,
    RunStatement,
    ReturnStatement,
    EndStatement,
    HelpStatement,
    ClsStatement,
    ListStatement,
    Empty,
}

#[derive(Debug, Clone)]
pub struct Line {
    pub number: Option<usize>,
    pub statement: Statement,
    pub source: String,
}
