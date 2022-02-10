//! The AST for the parser to use

/// Types for ZECA's expressions. Uses mostly native Rust types
#[derive(Debug)]
pub enum Expr {
    /// Numbers. Both integers and float
    Num(Number),
    /// True or false
    Bool(bool),

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

    /// Variable "calls"
    Var(String),
    /// Function call expression. `()` operator placed after a symbol, as in `foo()`.
    Call(String, Vec<Expr>),
    /// Variables declarations
    Let {
        /// Name defined to this symbol.
        name: String,
        /// Value to be assigned to symbol.
        rhs: Box<Expr>,
        /// Evaluated after symbol's own evaluation.
        then: Box<Expr>,
    },
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
}

/// Types for ZECA's expressions. Uses mostly native Rust types
#[derive(Debug)]
pub enum Number {
    /// Integer numbers. -1, 0, 1
    Integer(isize),
    /// Real numbers
    Float(f64),
}
