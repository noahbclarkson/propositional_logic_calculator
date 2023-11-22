use std::{
    fmt::{self, Display},
    rc::Rc,
    vec,
};

use enum_iterator::Sequence;

use crate::{
    expression::Expression,
    proof::{Proof, SearchNode, SearchSettings},
};

const INNER_SEARCH_SETTINGS: SearchSettings = SearchSettings {
    max_line_length: 6,
    iterations: 10000,
};

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

    pub fn matching_expressions(&self, other: &Line) -> bool {
        self.expression == other.expression
    }

    pub fn matches_expression(&self, expression: &Expression) -> bool {
        self.expression == *expression
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

#[derive(Debug, Clone)]
pub struct PossibleFinder {
    node: Box<SearchNode>,
    possibles: Vec<Possible>,
    vars: Vec<String>,
}

impl PossibleFinder {
    pub fn new(node: SearchNode) -> Self {
        let possibles = vec![];
        PossibleFinder {
            node: Box::new(node),
            possibles,
            vars: vec![],
        }
    }

    pub fn len(&self) -> usize {
        self.node.lines.len()
    }

    fn find_vars(&self) -> Vec<String> {
        let mut found_vars = Vec::new();
        for line in self.node.lines.iter() {
            find_vars_for_expression(&line.expression, &mut found_vars);
        }
        find_vars_for_expression(&self.node.conclusion, &mut found_vars);
        found_vars
    }

    pub fn possibles(&self) -> &Vec<Possible> {
        &self.possibles
    }

    pub fn find(&mut self) {
        self.vars = self.find_vars();
        self.possible_mp();
        self.possible_mt();
        self.possible_and_e();
        self.possible_and_i();
        self.possible_or_i();
        self.possible_or_i_with_vars();
        self.possible_dn_remove();
        self.possible_dn_add();
        self.possible_or_e();
    }

    pub fn add_possible(&mut self, possible: Possible) {
        self.possibles.push(possible);
    }

    fn assumption_line_nums(&self, deduction_lines: Vec<usize>) -> Vec<usize> {
        let lines = deduction_lines
            .iter()
            .map(|x| self.node.lines.get(*x))
            .collect::<Vec<Option<&Line>>>();
        // Flatten to get all valid lines, sort and dedup
        let mut assumption_lines = lines
            .iter()
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .map(|x| x.assumption_lines.clone())
            .flatten()
            .collect::<Vec<usize>>();
        assumption_lines.sort();
        assumption_lines.dedup();
        assumption_lines
    }

    fn possible_mp(&mut self) {
        for ab in self.clone().iterate_lines_in_dimension(2) {
            // If the first line is an implication
            if let Expression::Implies(left, right) = &ab[0].expression {
                // If the second line matches the left side of the implication
                if !ab[1].matches_expression(&left) {
                    continue;
                }
                let deductions = vec![ab[0].line_number, ab[1].line_number];
                let assumptions = self.assumption_line_nums(deductions.clone());
                let possible = Possible::new_single(Line::new(
                    assumptions,
                    self.len(),
                    right.as_ref().clone(),
                    Rule::ModusPonens,
                    deductions,
                ));
                self.add_possible(possible);
            }
        }
    }

    fn possible_mt(&mut self) {
        for ab in self.clone().iterate_lines_in_dimension(2) {
            // If the first line is an implication
            if let Expression::Implies(left, right) = &ab[0].expression {
                // If the second line is a negation of the right side of the implication
                if !ab[1].matches_expression(&Expression::Not(right.clone())) {
                    continue;
                }
                let deductions = vec![ab[0].line_number, ab[1].line_number];
                let assumptions = self.assumption_line_nums(deductions.clone());
                let possible = Possible::new_single(Line::new(
                    assumptions,
                    self.len(),
                    Expression::Not(Rc::new(left.as_ref().clone())),
                    Rule::ModusTollens,
                    deductions,
                ));
                self.add_possible(possible);
            }
        }
    }

    fn possible_and_e(&mut self) {
        for line in self.clone().node.lines.iter() {
            // If the line is an and expression
            if let Expression::And(left, right) = &line.expression {
                let deductions = vec![line.line_number];
                let assumptions = self.assumption_line_nums(deductions.clone());
                let line = Line::new(
                    assumptions,
                    self.len(),
                    left.as_ref().clone(),
                    Rule::AndElimination,
                    deductions,
                );
                let mut line2 = line.clone();
                line2.expression = right.as_ref().clone();
                let possibles = vec![Possible::new_single(line), Possible::new_single(line2)];
                self.possibles.extend(possibles);
            }
        }
    }

    fn possible_dn_remove(&mut self) {
        for line in self.clone().node.lines.iter() {
            // If the line is a double negation
            if let Expression::Not(inner) = &line.expression {
                if let Expression::Not(inner2) = &inner.as_ref() {
                    let deductions = vec![line.line_number];
                    let assumptions = self.assumption_line_nums(deductions.clone());
                    let possible = Possible::new_single(Line::new(
                        assumptions,
                        self.len(),
                        inner2.as_ref().clone(),
                        Rule::DoubleNegation,
                        deductions,
                    ));
                    self.add_possible(possible);
                }
            }
        }
    }

    fn possible_dn_add(&mut self) {
        for line in self.clone().node.lines.iter() {
            let deductions = vec![line.line_number];
            let assumptions = self.assumption_line_nums(deductions.clone());
            let possible = Possible::new_single(Line::new(
                assumptions,
                self.len(),
                Expression::Not(Rc::new(Expression::Not(Rc::new(line.expression.clone())))),
                Rule::DoubleNegation,
                deductions,
            ));
            self.add_possible(possible);
        }
    }

    fn possible_and_i(&mut self) {
        for ab in self.clone().iterate_lines_in_dimension(2) {
            let deductions = vec![ab[0].line_number, ab[1].line_number];
            let assumptions = self.assumption_line_nums(deductions.clone());
            let possible = Possible::new_single(Line::new(
                assumptions,
                self.len(),
                Expression::And(
                    Rc::new(ab[0].expression.clone()),
                    Rc::new(ab[1].expression.clone()),
                ),
                Rule::AndIntroduction,
                deductions,
            ));
            self.add_possible(possible);
        }
    }

    fn possible_or_i(&mut self) {
        for ab in self.clone().iterate_lines_in_dimension(2) {
            let deductions = vec![ab[0].line_number, ab[1].line_number];
            let assumptions = self.assumption_line_nums(deductions.clone());
            let possible = Possible::new_single(Line::new(
                assumptions,
                self.len(),
                Expression::Or(
                    Rc::new(ab[0].expression.clone()),
                    Rc::new(ab[1].expression.clone()),
                ),
                Rule::OrIntroduction,
                deductions,
            ));
            self.add_possible(possible);
        }
    }

    fn possible_or_i_with_vars(&mut self) {
        for line in self.node.lines.iter() {
            let vars = self.vars.clone();
            for c in vars {
                let deductions = vec![line.line_number];
                let assumptions = self.assumption_line_nums(deductions.clone());
                let line = Line::new(
                    assumptions,
                    self.len(),
                    Expression::Or(
                        Rc::new(line.expression.clone()),
                        Rc::new(Expression::Var(c.clone())),
                    ),
                    Rule::OrIntroduction,
                    deductions,
                );
                let mut line2 = line.clone();
                line2.expression = Expression::Or(
                    Rc::new(Expression::Var(c.clone())),
                    Rc::new(line.expression.clone()),
                );
                let possibles = vec![Possible::new_single(line), Possible::new_single(line2)];
                self.possibles.extend(possibles);
            }
        }
    }

    fn possible_or_e(&mut self) {
        for line in self.clone().node.lines.iter() {
            // If the line is an or expression
            if let Expression::Or(left, right) = &line.expression {
                let mut a_lines = self.node.lines.clone();
                let line = Line::new(
                    vec![line.line_number],
                    self.len(),
                    left.as_ref().clone(),
                    Rule::OrEliminationAssumption,
                    vec![line.line_number],
                );
                a_lines.push(line.clone());
                // Try to contruct a proof for the conclusion using the new assumption (a)
                let mut a_proof = Proof::new_raw(
                    self.node.assumptions().clone(),
                    self.node.conclusion.clone(),
                    a_lines,
                    Some(INNER_SEARCH_SETTINGS),
                );
                let a_result = a_proof.search();
                match a_result {
                    Ok(_) => (),
                    Err(_) => continue,
                }
                let mut line_b = line.clone();
                line_b.expression = right.as_ref().clone();
                let mut b_lines = self.node.lines.clone();
                b_lines.push(line_b);
                // Try to contruct a proof for the conclusion using the new assumption (b)
                let mut b_proof = Proof::new_raw(
                    self.node.assumptions().clone(),
                    self.node.conclusion.clone(),
                    b_lines,
                    Some(INNER_SEARCH_SETTINGS),
                );
                let b_result = b_proof.search();
                match b_result {
                    Ok(_) => (),
                    Err(_) => continue,
                }
                let a_deductions_lines = a_proof.get_deduction_lines();
                // Add the lines from this proof
                let mut resulting_lines = Vec::new();
                for l in a_deductions_lines.clone() {
                    resulting_lines.push(l.clone());
                }
                // Add the lines from the second proof
                let b_deductions_lines = b_proof.get_deduction_lines();
                for l in b_deductions_lines.clone() {
                    let mut l = l.clone();
                    l.line_number += a_deductions_lines.len();
                    resulting_lines.push(l);
                }
                let mut deductions = vec![line.line_number];
                // Add all the lines from the proofs
                for l in resulting_lines.clone() {
                    deductions.push(l.line_number);
                }
                // To get the assumptions we can just input the deductions into the assumption_line_nums function
                // We need to look at all of the new lines and get their assumptions
                let mut assumptions = self.assumption_line_nums(deductions.clone());
                // Add the assumption from the first proof
                for l in resulting_lines.clone() {
                    for a in l.assumption_lines {
                        if !assumptions.contains(&a) {
                            assumptions.push(a);
                        }
                    }
                }

                let final_line = Line::new(
                    assumptions,
                    self.len() + resulting_lines.len(),
                    self.node.conclusion.clone(),
                    Rule::OrElimination,
                    deductions,
                );
                resulting_lines.push(final_line);
                let possible = Possible::new(resulting_lines);
                self.add_possible(possible);
            }
        }
    }

    fn iterate_lines_in_dimension(&self, dimension: usize) -> impl Iterator<Item = Vec<&Line>> {
        // Create an array to store the current indices for each dimension.
        let mut indices = vec![0; dimension];
        let mut is_done = false;

        std::iter::from_fn(move || {
            if is_done {
                return None;
            }

            // Map the current indices to the corresponding lines.
            let result = indices
                .iter()
                .map(|&index| &self.node.lines[index])
                .collect::<Vec<&Line>>();

            // Increment indices to get the next combination.
            for i in 0..dimension {
                if indices[i] < self.node.lines.len() - 1 {
                    // Increment this index and reset all previous indices to 0.
                    indices[i] += 1;
                    for j in 0..i {
                        indices[j] = 0;
                    }
                    return Some(result);
                }
            }

            // If all combinations have been generated, mark as done.
            is_done = true;
            Some(result)
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Possible {
    pub lines: Vec<Line>,
}

impl Possible {
    pub fn new(lines: Vec<Line>) -> Self {
        Possible { lines }
    }

    pub fn new_single(line: Line) -> Self {
        Possible { lines: vec![line] }
    }
}

fn find_vars_for_expression(expression: &Expression, vars: &mut Vec<String>) {
    let expressions = expression.list_expressions();
    for expression in expressions {
        match expression {
            Expression::Var(var) => {
                if !vars.contains(&var) {
                    vars.push(var);
                }
            }
            _ => {}
        }
    }
}
