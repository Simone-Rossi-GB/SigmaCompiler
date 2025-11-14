#[derive(Debug)]
pub struct Program {
    pub functions: Vec<Function>
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub return_type: Type,
    pub parameters: Vec<Parameter>,
    pub body: Vec<Statement>
}

#[derive(Debug)]
pub enum Statement {
    VarDecl { var_type: Type, name: String, value: Expression },
    Assignment { name: String, value: Expression },
    Print { expr: Expression },
    Return { expr: Option<Expression> },
    Break  // ohio - esce dal loop
}

#[derive(Debug)]
pub struct Parameter {
    pub name: String,
    pub parameter_type: Type
}

#[derive(Debug)]
pub enum Expression {
    Integer(i32),       // based - numeri interi piccoli
    Long(i64),          // superBased - numeri interi grandi
    Float(f64),         // chill - numeri decimali
    StringLit(String),  // vibes - stringhe
    CharLit(char),      // Chad - carattere
    Variable(String),    // nome di variabile
    BinOp {
        left: Box<Expression>,
        op: BinOp,
        right: Box<Expression>
    }
}

#[derive(Debug)]
pub enum BinOp {
    // Operatori aritmetici
    Add,      // +
    Sub,      // -
    Mul,      // *
    Div,      // /

    // Operatori di comparazione
    Equal,    // ==
    NotEqual, // !=
    Less,     // <
    Greater,  // >
    LessEq,   // <=
    GreaterEq // >=
}

#[derive(Debug)]
pub enum Type {
    Based,      // int
    SuperBased, // long
    Chill,      // float
    Vibes,      // string
    Chad,       // char
    Ghost       // void
}