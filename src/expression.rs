use std::fmt::{self, Display};
use std::rc::Rc;

/// Represents logical expressions in abstract syntax tree (AST) form.
/// Supports basic logical operations like AND, OR, IMPLIES, and NOT, as well as variables.
#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    /// Logical AND operation with two child `Expression` nodes.
    And(Rc<Expression>, Rc<Expression>),

    /// Logical OR operation with two child `Expression` nodes.
    Or(Rc<Expression>, Rc<Expression>),

    /// Logical IMPLIES operation with two child `Expression` nodes.
    Implies(Rc<Expression>, Rc<Expression>),

    /// Logical NOT operation with a single child `Expression` node.
    Not(Rc<Expression>),

    /// Represents a variable in the logical expression, stored as a `String`.
    Var(String),
}

/// Implementation of the `Display` trait for the `Expression` enum.
/// This allows for the pretty printing of `Expression` instances in a human-readable format.
impl Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::And(left, right) => write!(f, "({} & {})", left, right),
            Expression::Or(left, right) => write!(f, "({} v {})", left, right),
            Expression::Implies(left, right) => write!(f, "({} -> {})", left, right),
            Expression::Not(expr) => write!(f, "~{}", expr),
            Expression::Var(name) => write!(f, "{}", name),
        }
    }
}

impl Expression {
    /// Extracts and lists all unique sub-expressions (including the current one) from this `Expression`.
    /// It traverses the AST recursively to gather all expressions.
    ///
    /// Returns a `Vec<Expression>` containing all unique expressions found.
    pub fn list_expressions(&self) -> Vec<Expression> {
        let mut expressions = Vec::new();
        match self {
            Expression::And(left, right)
            | Expression::Or(left, right)
            | Expression::Implies(left, right) => {
                expressions.push(self.clone());
                expressions.extend(left.list_expressions());
                expressions.extend(right.list_expressions());
            }
            Expression::Not(expr) => {
                expressions.push(self.clone());
                expressions.extend(expr.list_expressions());
            }
            Expression::Var(_) => expressions.push(self.clone()),
        }
        expressions.dedup();
        expressions
    }
}
