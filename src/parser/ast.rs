//! The AST for the parser to use

/// Types for ZECA's expressions. Uses mostly native Rust types
#[derive(Debug)]
pub enum Expr {
    /// Numbers. `f64` should be the largest upcastable number and calculations will be done with these.
    Num(f64),

    /// Negation expression. Unary `-` operator.
    Neg(Box<Expr>),
    /// Addition expression. Binary `+` operator.
    Add(Box<Expr>, Box<Expr>),
    /// Subtaction expression. Binary `-` operator.
    Sub(Box<Expr>, Box<Expr>),
    /// Multiplication expression. Binary `*` operator.
    Mul(Box<Expr>, Box<Expr>),
    /// Division expression. Binary `/` operator.
    Div(Box<Expr>, Box<Expr>),

    /// Variables. Inside is stored the symbol name.
    Var(String),
    /// Function call expression. `()` operator placed after a symbol, as in `foo()`.
    Call(String, Vec<Expr>),
    /// Used to declare variables.
    Let {
        /// Name defined to this symbol.
        name: String,
        /// Value to be assigned to symbol.
        rhs: Box<Expr>,
        /// Evaluated after symbol's own evaluation.
        then: Box<Expr>,
    },
    /// Used to declare functions
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
