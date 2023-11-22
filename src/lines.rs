use std::{
    fmt::{self, Display},
    rc::Rc,
    vec,
};

use enum_iterator::Sequence;

use crate::{expression::Expression, proof::Proof};

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
            .map(|x| x + 1)
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        let deduction_lines = self
            .deduction_lines
            .iter()
            .map(|x| x + 1)
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

pub struct ProofState {
    pub lines: Vec<Line>,
    pub conclusion: Expression,
    iter_i: usize,
    iter_j: usize,
}

impl ProofState {
    pub fn new(lines: Vec<Line>, conclusion: Expression) -> Self {
        ProofState {
            lines,
            conclusion,
            iter_i: 0,
            iter_j: 0,
        }
    }

    fn get_i_line(&self) -> &Line {
        &self.lines[self.iter_i]
    }

    fn get_j_line(&self) -> &Line {
        &self.lines[self.iter_j]
    }

    fn find_vars(&self) -> Vec<String> {
        let mut found_vars = Vec::new();
        for line in self.lines.iter() {
            let expressions = line.expression.list_expressions();
            for expression in expressions {
                match expression {
                    Expression::Var(var) => {
                        if !found_vars.contains(&var) {
                            found_vars.push(var);
                        }
                    }
                    _ => {}
                }
            }
        }
        let expressions = self.conclusion.list_expressions();
        for expression in expressions {
            match expression {
                Expression::Var(var) => {
                    if !found_vars.contains(&var) {
                        found_vars.push(var);
                    }
                }
                _ => {}
            }
        }
        found_vars
    }
}

pub fn possible_lines(state: ProofState, rule: Rule) -> Vec<Line> {
    match rule {
        Rule::Assumption => panic!("Assumption rule does not have possible lines"),
        Rule::ModusPonens => find_possible_of_type(state, possible_mp, rule),
        Rule::ModusTollens => find_possible_of_type(state, possible_mt, rule),
        Rule::ConditionalProof => todo!(),
        Rule::DoubleNegation => find_possible_of_type(state, possible_dn, rule),
        Rule::AndIntroduction => find_possible_of_type(state, possible_and_i, rule),
        Rule::AndElimination => find_possible_of_type(state, possible_and_e, rule),
        Rule::OrIntroduction => find_possible_of_type(state, possible_or_i, rule),
        Rule::OrElimination => find_possible_of_type(state, possible_or_e, rule),
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

fn find_possible_of_type<F>(mut state: ProofState, validator: F, rule: Rule) -> Vec<Line>
where
    F: Fn(&ProofState) -> Vec<Line>,
{
    let mut possibles = Vec::new();
    let length = state.lines.len();
    for i in 0..length {
        for j in 0..length {
            state.iter_i = i;
            state.iter_j = j;
            let lines = validator(&state);
            for line in lines {
                let line = Line::new(
                    get_assumption_line_numbers(state.lines.clone(), line.deduction_lines.clone()),
                    length,
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

fn possible_mp(state: &ProofState) -> Vec<Line> {
    match state.get_i_line().expression.clone() {
        Expression::Implies(left, right) => match *left == state.get_j_line().expression {
            true => {
                let deduction_lines = vec![state.iter_i, state.iter_j];
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

fn possible_mt(state: &ProofState) -> Vec<Line> {
    match state.get_i_line().expression.clone() {
        Expression::Implies(left, right) => match state.get_j_line().expression.clone() {
            Expression::Not(inner) => match *right == *inner {
                true => {
                    let expression = Expression::Not(left);
                    let deduction_lines = vec![state.iter_i, state.iter_j];
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

fn possible_and_e(state: &ProofState) -> Vec<Line> {
    match state.get_i_line().expression.clone() {
        Expression::And(left, right) => {
            let deduction_lines = vec![state.iter_i];
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

fn possible_dn(state: &ProofState) -> Vec<Line> {
    let mut lines = Vec::new();
    let dn_removal = match state.get_i_line().expression.clone() {
        Expression::Not(inner) => match (*inner).clone() {
            Expression::Not(inner) => {
                let deduction_lines = vec![state.iter_i];
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
    let deduction_lines = vec![state.iter_i];
    lines.push(Line::new(
        vec![],
        0,
        Expression::Not(Rc::new(Expression::Not(Rc::new(
            state.get_i_line().expression.clone(),
        )))),
        Rule::DoubleNegation,
        deduction_lines,
    ));
    lines
}

fn possible_and_i(state: &ProofState) -> Vec<Line> {
    let deduction_lines = vec![state.iter_i, state.iter_j];
    vec![Line::new(
        vec![],
        0,
        Expression::And(
            Rc::new(state.get_i_line().expression.clone()),
            Rc::new(state.get_j_line().expression.clone()),
        ),
        Rule::AndIntroduction,
        deduction_lines,
    )]
}

fn possible_or_i(state: &ProofState) -> Vec<Line> {
    let mut deduction_lines = vec![state.iter_i, state.iter_j];
    let mut or_with_existing = vec![
        Line::new(
            vec![],
            0,
            Expression::Or(
                Rc::new(state.get_i_line().expression.clone()),
                Rc::new(state.get_j_line().expression.clone()),
            ),
            Rule::OrIntroduction,
            deduction_lines.clone(),
        ),
        Line::new(
            vec![],
            0,
            Expression::Or(
                Rc::new(state.get_j_line().expression.clone()),
                Rc::new(state.get_i_line().expression.clone()),
            ),
            Rule::OrIntroduction,
            deduction_lines,
        ),
    ];
    // Add new lines that are line_1 v A..Z where A..Z are all variables that occur in the proof and conclusion
    deduction_lines = vec![state.iter_i];
    for c in state.find_vars() {
        let expression = Expression::Or(
            Rc::new(state.get_i_line().expression.clone()),
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

fn possible_or_e(state: &ProofState) -> Vec<Line> {
    let mut lines = Vec::new();
    match state.get_i_line().expression.clone() {
        Expression::Or(left, right) => {
            // If the lines contain an or elimination already, we don't want to add another one
            let mut or_elimination = false;
            for line in state.lines.iter() {
                if line.rule == Rule::OrElimination {
                    or_elimination = true;
                }
            }
            if or_elimination {
                return lines;
            }
            // Now we want to conduct two proofs to see if the conclusion is true for both
            let mut starting_lines_1 = state.lines.clone();
            starting_lines_1.push(Line::new(
                vec![state.lines.len()],
                state.lines.len(),
                (*left).clone(),
                Rule::OrElimination,
                vec![state.iter_i],
            ));
            let mut proof1 =
                Proof::new_raw(vec![], state.conclusion.clone(), starting_lines_1, None);
            match proof1.search() {
                Ok(_) => (),
                Err(_) => return lines,
            };
            let mut starting_lines_2 = state.lines.clone();
            starting_lines_2.push(Line::new(
                vec![state.lines.len()],
                state.lines.len(),
                (*right).clone(),
                Rule::OrElimination,
                vec![state.iter_i],
            ));
            let mut proof2 =
                Proof::new_raw(vec![], state.conclusion.clone(), starting_lines_2, None);
            match proof2.search() {
                Ok(_) => (),
                Err(_) => return lines,
            };
            let deduction_lines = vec![state.iter_i];
            lines.push(Line::new(
                vec![state.iter_i],
                0,
                state.conclusion.clone(),
                Rule::OrElimination,
                deduction_lines,
            ));
        }
        _ => {}
    }
    lines
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
