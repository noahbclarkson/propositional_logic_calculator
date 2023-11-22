use std::fmt::{self, Display};
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    And(Rc<Expression>, Rc<Expression>),
    Or(Rc<Expression>, Rc<Expression>),
    Implies(Rc<Expression>, Rc<Expression>),
    Not(Rc<Expression>),
    Var(String),
}

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
    pub fn list_expressions(&self) -> Vec<Expression> {
        // Get all the nested expressions in a list
        let mut expressions = Vec::new();
        match self {
            Expression::And(left, right) => {
                expressions.push(Expression::And(left.clone(), right.clone()));
                expressions.extend(left.list_expressions());
                expressions.extend(right.list_expressions());
            }
            Expression::Or(left, right) => {
                expressions.push(Expression::Or(left.clone(), right.clone()));
                expressions.extend(left.list_expressions());
                expressions.extend(right.list_expressions());
            }
            Expression::Implies(left, right) => {
                expressions.push(Expression::Implies(left.clone(), right.clone()));
                expressions.extend(left.list_expressions());
                expressions.extend(right.list_expressions());
            }
            Expression::Not(expr) => {
                expressions.push(Expression::Not(expr.clone()));
                expressions.extend(expr.list_expressions());
            }
            Expression::Var(_) => expressions.push(self.clone()),
        }
        // Remove duplicates
        expressions.dedup();
        expressions
    }
}
