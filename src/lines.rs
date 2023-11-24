use std::fmt::{self, Display};

use enum_iterator::Sequence;

use crate::expression::Expression;

#[derive(Debug, Clone)]
pub struct Line {
    pub assumption_lines: Vec<usize>,
    pub line_number: usize,
    pub expression: Expression,
    pub rule: Rule,
    pub deduction_lines: Vec<usize>,
}

impl Line {
    pub fn new(
        assumption_lines: Vec<usize>,
        line_number: usize,
        expression: Expression,
        rule: Rule,
        deduction_lines: Vec<usize>,
    ) -> Self {
        Line {
            assumption_lines,
            line_number,
            expression,
            rule,
            deduction_lines,
        }
    }

    pub fn matches_expression(&self, expression: &Expression) -> bool {
        self.expression == *expression
    }
}

impl PartialEq<Expression> for Line {
    fn eq(&self, other: &Expression) -> bool {
        self.expression == *other
    }
}

#[derive(Debug, PartialEq, Clone, Sequence)]
pub enum Rule {
    Assumption,
    ModusPonens,
    ModusTollens,
    ConditionalProof,
    ConditionalProofAssumption,
    DoubleNegation,
    AndIntroduction,
    AndElimination,
    OrIntroduction,
    OrElimination,
    OrEliminationAssumption,
    ReductioAdAbsurdium,
}

impl Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rule::Assumption => write!(f, "A"),
            Rule::ModusPonens => write!(f, "MPP"),
            Rule::ModusTollens => write!(f, "MTT"),
            Rule::ConditionalProof => write!(f, "CP"),
            Rule::ConditionalProofAssumption => write!(f, "A(CP)"),
            Rule::DoubleNegation => write!(f, "DN"),
            Rule::AndIntroduction => write!(f, "&I"),
            Rule::AndElimination => write!(f, "&E"),
            Rule::OrIntroduction => write!(f, "vI"),
            Rule::OrElimination => write!(f, "vE"),
            Rule::OrEliminationAssumption => write!(f, "A(vE)"),
            Rule::ReductioAdAbsurdium => write!(f, "RAA"),
        }
    }
}
