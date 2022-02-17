//! The AST for the parser to use

/// Return values for ZECA.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Literal {
    /// Unit types ([`()`]) but tuples are not yet implemented.
    Null,
    /// Numbers.
    Num(Number),
    /// Boolean value.
    Bool(bool),
    /// String value
    Str(String),
    /// Function variables
    Fn(Function),
}

/// Types for ZECA's expressions. Uses mostly native Rust types
#[derive(Debug, Clone)]
pub enum Expr {
    /// Literals values used within `Expr`s , e.g **1** + **1**.
    Literal(Literal),

    /// Negation expression. Both things like `-1` and `!true`.
    Neg(Box<Expr>),

    /// Binary +.
    Add(Box<Expr>, Box<Expr>),
    /// Binary -.
    Sub(Box<Expr>, Box<Expr>),
    /// Binary *.
    Mul(Box<Expr>, Box<Expr>),
    /// Binary /.
    Div(Box<Expr>, Box<Expr>),

    /// Expr1 < Expr2.
    Lt(Box<Expr>, Box<Expr>),
    /// Expr1 > Expr2.
    Gt(Box<Expr>, Box<Expr>),
    /// Expr1 == Expr2.
    Eq(Box<Expr>, Box<Expr>),

    /// Function call expression. `()` operator placed after a symbol, as in `foo()`.
    Call(String, Vec<Expr>),
    /// Variable invocation.
    Var(String),
}

/// Types for ZECA's expressions. Uses mostly native Rust types
#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub enum Number {
    /// Fake numbers. -1, 0, 1.
    Integer(isize),
    /// Unsigned integer numbers. 0, 1, 2.
    UInteger(usize),
    /// Real numbers.
    Float(f64),
}

impl std::ops::Neg for Number {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::Integer(x) => Self::Integer(-x),
            // Notice the casting
            Self::UInteger(x) => Self::Integer(-(x as isize)),
            Self::Float(x) => Self::Float(-x),
        }
    }
}

/// A function declaration.
#[derive(Debug, Clone)]
pub struct Function {
    /// Function name symbol
    pub name: String,
    // TODO argument typing
    /// List of function argument names. 
    pub args: Vec<String>,
    /// Function body, a block of statements.
    pub body: Box<Block>,
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl PartialOrd for Function {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

/// Top-level constructs. Declared "with no indentation".
#[derive(Debug, Clone)]
pub enum Item {
    /// A function declaration
    Function(Function)
}

/// A statement. Can be wither a Block or a StatementWithoutBlock.
#[derive(Debug, Clone)]
pub enum Statement {
    /// A null statement (contains only `;`).
    Null,
    /// A item construct. Those can be placed wherever a statement can.
    Item(Box<Item>),
    /// Variable declaration.
    Let {
        /// Name defined to this symbol.
        name: String,
        /// Value to be assigned to symbol.
        rhs: Box<Expr>,
    },
    /// Expression (includes call and invoking).
    Expr(Expr),
}

/// A Statement Block. Simply a list of sequential statements.
#[derive(Debug, Clone)]
pub struct Block(pub Vec<Statement>);
