use std::{
    fmt::{self, Display},
    rc::Rc,
    vec,
};

use enum_iterator::Sequence;

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
}

impl Line {
    pub fn matching_expressions(&self, other: &Line) -> bool {
        self.expression == other.expression
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Assumption lines should be displayed as a comma separated list first, then the line number in brackets, then the rule used, then the deduction lines
        let assumption_lines = self
            .assumption_lines
            .iter()
            .map(|x| x+1)
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        let deduction_lines = self
            .deduction_lines
            .iter()
            .map(|x| x+1)
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        write!(
            f,
            "{} ({}) {} {} {}",
            assumption_lines,
            self.line_number + 1,
            self.expression,
            self.rule,
            deduction_lines
        )
    }
}

#[derive(Debug, PartialEq, Clone, Sequence)]
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

pub fn possible_lines(antecedent_lines: Vec<Line>, rule: Rule) -> Vec<Line> {
    match rule {
        Rule::Assumption => panic!("Assumption rule does not have possible lines"),
        Rule::ModusPonens => find_possible_of_type(antecedent_lines, possible_mp, rule),
        Rule::ModusTollens => find_possible_of_type(antecedent_lines, possible_mt, rule),
        Rule::ConditionalProof => todo!(),
        Rule::DoubleNegation => find_possible_of_type(antecedent_lines, possible_dn, rule),
        Rule::AndIntroduction => find_possible_of_type(antecedent_lines, possible_and_i, rule),
        Rule::AndElimination => find_possible_of_type(antecedent_lines, possible_and_e, rule),
        Rule::OrIntroduction => find_possible_of_type(antecedent_lines, possible_or_i, rule),
        Rule::OrElimination => todo!(),
        Rule::ReductioAdAbsurdium => todo!(),
    }
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

fn find_possible_of_type<F>(antecedent_lines: Vec<Line>, validator: F, rule: Rule) -> Vec<Line>
where
    F: Fn(&Line, &Line) -> Vec<Line>,
{
    let mut possibles = Vec::new();
    for i in 0..antecedent_lines.len() {
        for j in 0..antecedent_lines.len() {
            let lines = validator(&antecedent_lines[i], &antecedent_lines[j]);
            for line in lines {
                let line = Line::new(
                    get_assumption_line_numbers(
                        antecedent_lines.clone(),
                        line.deduction_lines.clone(),
                    ),
                    antecedent_lines.len(),
                    line.expression,
                    rule.clone(),
                    line.deduction_lines,
                );
                possibles.push(line);
            }
        }
    }
    possibles.dedup();
    possibles
}

fn possible_mp(line_1: &Line, line_2: &Line) -> Vec<Line> {
    match line_1.expression.clone() {
        Expression::Implies(left, right) => match *left == line_2.expression {
            true => {
                let deduction_lines = vec![line_1.line_number, line_2.line_number];
                return vec![Line::new(
                    vec![],
                    0,
                    (*right).clone(),
                    Rule::ModusPonens,
                    deduction_lines,
                )];
            }
            false => vec![],
        },
        _ => vec![],
    }
}

fn possible_mt(line_1: &Line, line_2: &Line) -> Vec<Line> {
    match line_1.expression.clone() {
        Expression::Implies(left, right) => match line_2.expression.clone() {
            Expression::Not(inner) => match *right == *inner {
                true => {
                    let expression = Expression::Not(left);
                    let deduction_lines = vec![line_1.line_number, line_2.line_number];
                    return vec![Line::new(
                        vec![],
                        0,
                        expression,
                        Rule::ModusTollens,
                        deduction_lines,
                    )];
                }
                false => vec![],
            },
            _ => vec![],
        },
        _ => vec![],
    }
}

fn possible_and_e(line_1: &Line, _line_2: &Line) -> Vec<Line> {
    match line_1.expression.clone() {
        Expression::And(left, right) => {
            let deduction_lines = vec![line_1.line_number];
            return vec![
                Line::new(
                    vec![],
                    0,
                    (*left).clone(),
                    Rule::AndElimination,
                    deduction_lines.clone(),
                ),
                Line::new(
                    vec![],
                    0,
                    (*right).clone(),
                    Rule::AndElimination,
                    deduction_lines,
                ),
            ];
        }
        _ => vec![],
    }
}

fn possible_dn(line_1: &Line, _line_2: &Line) -> Vec<Line> {
    let mut lines = Vec::new();
    let dn_removal = match line_1.expression.clone() {
        Expression::Not(inner) => match (*inner).clone() {
            Expression::Not(inner) => {
                let deduction_lines = vec![line_1.line_number];
                Some(Line::new(
                    vec![],
                    0,
                    (*inner).clone(),
                    Rule::DoubleNegation,
                    deduction_lines,
                ))
            }
            _ => None,
        },
        _ => None,
    };
    match dn_removal {
        Some(line) => lines.push(line),
        None => {}
    }
    let deduction_lines = vec![line_1.line_number];
    lines.push(Line::new(
        vec![],
        0,
        Expression::Not(Rc::new(Expression::Not(Rc::new(line_1.expression.clone())))),
        Rule::DoubleNegation,
        deduction_lines,
    ));
    lines
}

fn possible_and_i(line_1: &Line, line_2: &Line) -> Vec<Line> {
    let deduction_lines = vec![line_1.line_number, line_2.line_number];
    vec![Line::new(
        vec![],
        0,
        Expression::And(
            Rc::new(line_1.expression.clone()),
            Rc::new(line_2.expression.clone()),
        ),
        Rule::AndIntroduction,
        deduction_lines,
    )]
}

fn possible_or_i(line_1: &Line, line_2: &Line) -> Vec<Line> {
    let mut deduction_lines = vec![line_1.line_number, line_2.line_number];
    let mut or_with_existing = vec![
        Line::new(
            vec![],
            0,
            Expression::Or(
                Rc::new(line_1.expression.clone()),
                Rc::new(line_2.expression.clone()),
            ),
            Rule::OrIntroduction,
            deduction_lines.clone(),
        ),
        Line::new(
            vec![],
            0,
            Expression::Or(
                Rc::new(line_2.expression.clone()),
                Rc::new(line_1.expression.clone()),
            ),
            Rule::OrIntroduction,
            deduction_lines,
        ),
    ];
    // Add new lines that are line_1 v A..Z
    deduction_lines = vec![line_1.line_number];
    for c in 'A'..='Z' {
        let expression = Expression::Or(
            Rc::new(line_1.expression.clone()),
            Rc::new(Expression::Var((c).to_string())),
        );
        let line = Line::new(
            vec![],
            0,
            expression,
            Rule::OrIntroduction,
            deduction_lines.clone(),
        );
        or_with_existing.push(line);
    }
    or_with_existing.dedup();
    or_with_existing
}

fn get_assumption_line_numbers(
    antecedent_lines: Vec<Line>,
    deduction_lines: Vec<usize>,
) -> Vec<usize> {
    let lines = deduction_lines
        .iter()
        .map(|x| antecedent_lines[*x].clone().assumption_lines)
        .collect::<Vec<Vec<usize>>>();
    let mut lines = lines.iter().flatten().map(|x| *x).collect::<Vec<usize>>();
    lines.sort();
    lines.dedup();
    lines
}
