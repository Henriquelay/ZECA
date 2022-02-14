//! The AST for the parser to use

/// Return values for ZECA
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Literal {
    /// Numbers
    Num(Number),
    /// Boolean value
    Bool(bool),
    // TODO
    // Str(String),
    // TODO para fazer cidadãs de primeira classe
    // Fn
}

/// Types for ZECA's expressions. Uses mostly native Rust types
#[derive(Debug)]
pub enum Expr {
    /// Literals
    Lit(Literal),

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

    /// Function call expression. `()` operator placed after a symbol, as in `foo()`.
    Call(String, Vec<Expr>),
    /// Function declaration
    Fn {
        /// Name defined to this symbol.
        name: String,
        /// Arguments to be passed to function.
        args: Vec<String>,
        /// `Expr`s to be executed inside function.
        body: Box<Expr>,
        /// `Evaluated after symbol's own evaluation.
        then: Box<Expr>,
    },

    /// Variable "calls"
    Var(String),
    /// Variables declarations
    Let {
        /// Name defined to this symbol.
        name: String,
        /// Value to be assigned to symbol.
        rhs: Box<Expr>,
        /// Evaluated after symbol's own evaluation.
        then: Box<Expr>,
    },
}

/// Types for ZECA's expressions. Uses mostly native Rust types
#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub enum Number {
    /// Integer numbers. -1, 0, 1
    Integer(isize),
    /// Unsigned integer numbers. 0, 1, 2
    UInteger(usize),
    /// Real numbers
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
