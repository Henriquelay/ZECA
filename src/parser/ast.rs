//! The AST for the parser to use

/// Return values for ZECA
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Literal {
    /// Unit types ([`()`]) but tuples are not yet implemented
    Null,
    /// Numbers
    Num(Number),
    /// Boolean value
    Bool(bool),
    /// String value
    Str(String),
    /// Function variables
    Fn(Function),
    /// Array of literals
    Array(Vec<Literal>),
    /// Break special value
    Break,
}

/// Types for ZECA's expressions. Uses mostly native Rust types
#[derive(Debug, Clone)]
pub enum Expr {
    /// Literals values used within `Expr`s , e.g **1** + **1**
    Literal(Literal),

    /// Negation expression. Both things like `-1` and `!true`
    Neg(Box<Expr>),

    /// Binary +
    Add(Box<Expr>, Box<Expr>),
    /// Binary -
    Sub(Box<Expr>, Box<Expr>),
    /// Binary *
    Mul(Box<Expr>, Box<Expr>),
    /// Binary /
    Div(Box<Expr>, Box<Expr>),

    /// Expr1 < Expr2
    Lt(Box<Expr>, Box<Expr>),
    /// Expr1 > Expr2
    Gt(Box<Expr>, Box<Expr>),
    /// Expr1 == Expr2
    Eq(Box<Expr>, Box<Expr>),
    /// Expr1 && Expr2. >0 is truthy
    And(Box<Expr>, Box<Expr>),
    /// Expr1 || Expr2. >0 is truthy
    Or(Box<Expr>, Box<Expr>),

    /// Declare an array of expressions
    Array(Vec<Expr>),

    /// Function call expression. `()` operator placed after a symbol, as in `foo()`
    Call(String, Vec<Expr>),
    /// Variable invocation. Index is for array variable
    Var {
        /// Name of the variable
        name: String,
        /// Index offset from start of the array. If `[]` is not used, this is defaulted to `0`.
        index: Box<Expr>,
    },
}

/// Types for ZECA's expressions. Uses mostly native Rust types
#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub enum Number {
    /// Fake numbers. -1, 0, 1
    Integer(isize),
    /// Unsigned integer numbers. 0, 1, 2
    // UInteger(usize),
    /// Real numbers
    Float(f64),
}

impl std::ops::Neg for Number {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::Integer(x) => Self::Integer(-x),
            // Notice the casting
            // Self::UInteger(x) => Self::Integer(-(x as isize)),
            Self::Float(x) => Self::Float(-x),
        }
    }
}

/// A function declaration
#[derive(Debug, Clone)]
pub struct Function {
    /// Function name symbol
    pub name: String,
    // TODO argument typing
    /// List of function argument names
    pub args: Vec<String>,
    /// Function body, a block of statements
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

/// Top-level constructs. Declared "with no indentation"
#[derive(Debug, Clone)]
pub enum Item {
    /// A function declaration
    Function(Function),
}

/// A statement is a component of a block, which is in turn a component of an outer expression or function
#[derive(Debug, Clone)]
pub enum Statement {
    /// A null statement (contains only `;`)
    Null,
    /// A Break statement. Breaks out of [Loop]s.
    Break,
    /// A item construct. Those can be placed wherever a statement can
    Item(Box<Item>),
    /// Conditional execution. It Expr is true, executes first block, else executes second block
    Conditional {
        /// The expression to check for it will jump or not
        r#if: Box<Expr>,
        /// Jumps to this if `r#if`[Statement::Conditional.r#if] is true
        r#then: Box<Block>,
        /// Jumps to this if `r#if`[Statement::Conditional.r#if] is false. Is also optional, and will keep execution as normal if field is `None`
        r#else: Option<Box<Block>>,
    },
    /// Variable declaration
    Let {
        /// Name defined to this symbol
        lvalue: String,
        /// Value to be assigned to symbol
        rvalue: Box<Expr>,
    },
    /// Variable assignment
    Assign {
        /// Name defined to this symbol
        lvalue: String,
        /// Value to be assigned to symbol
        rvalue: Box<Expr>,
    },
    /// Expression (includes call and invoking)
    Expr(Box<Expr>),
    /// A Block statement
    Block(Box<Block>),
    /// A Loop statement
    Loop(Box<Loop>),
}

/// A Statement Block. Simply a list of sequential statements
#[derive(Debug, Clone)]
pub struct Block(pub Vec<Statement>);

/// A Loop statement. Only contains a block.
#[derive(Debug, Clone)]
pub struct Loop(pub Box<Block>);
