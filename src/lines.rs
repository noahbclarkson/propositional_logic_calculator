use std::fmt::{Display, self};

use crate::expression::Expression;

#[derive(Debug, PartialEq, Clone)]
pub struct Line {
    pub assumption_lines: Vec<usize>,
    pub line_number: usize,
    pub expression: Expression,
    pub rule: Rule,
    pub deduction_lines: Vec<usize>,
}

impl Line {
    pub fn new(assumption_lines: Vec<usize>, line_number: usize, expression: Expression, rule: Rule, deduction_lines: Vec<usize>) -> Self {
        Line {
            assumption_lines,
            line_number,
            expression,
            rule,
            deduction_lines,
        }
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Assumption lines should be displayed as a comma separated list first, then the line number in brackets, then the rule used, then the deduction lines
        let assumption_lines = self.assumption_lines.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ");
        let deduction_lines = self.deduction_lines.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ");
        write!(f, "{} ({}) {} {} {}", assumption_lines, self.line_number, self.expression, self.rule, deduction_lines)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Rule {
    Assumption,
    ModusPonens,
    ModusTollens,
    ConditionalProof,
    DoubleNegation,
    AndIntroduction,
    AndElimination,
    OrIntroduction,
    OrElimination,
    ReductioAdAbsurdium,
}

impl Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rule::Assumption => write!(f, "A"),
            Rule::ModusPonens => write!(f, "MPP"),
            Rule::ModusTollens => write!(f, "MTT"),
            Rule::ConditionalProof => write!(f, "CP"),
            Rule::DoubleNegation => write!(f, "DN"),
            Rule::AndIntroduction => write!(f, "&I"),
            Rule::AndElimination => write!(f, "&E"),
            Rule::OrIntroduction => write!(f, "vI"),
            Rule::OrElimination => write!(f, "vE"),
            Rule::ReductioAdAbsurdium => write!(f, "RAA"),
        }
    }
}