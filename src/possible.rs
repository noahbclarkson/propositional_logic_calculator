use rand::Rng;

use crate::{
    expression::Expression,
    lines::{Line, Rule},
    proof::{Proof, SearchNode, SearchSettings},
};

const INNER_SEARCH_SETTINGS: SearchSettings = SearchSettings {
    max_line_length: 15,
    iterations: 50000,
};

#[derive(Debug, Clone)]
pub struct PossibleFinder {
    node: Box<SearchNode>,
    possibles: Vec<Possible>,
    vars: Vec<char>,
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

    pub fn is_empty(&self) -> bool {
        self.node.lines.is_empty()
    }

    fn find_vars(&self) -> Vec<char> {
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
        self.possible_cp();
        // Check if an expression already exists in the proof (it is useless to add it again)
        self.possibles.retain(|x| {
            !self
                .node
                .lines
                .iter()
                .any(|y| x.lines.iter().any(|z| z.expression == y.expression))
        });
    }

    fn add_possible(&mut self, possible: Possible) {
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
            .flatten()
            .flat_map(|x| x.assumption_lines.clone())
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
                if !ab[1].matches_expression(left) {
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
                    Expression::Not(left.as_ref().clone().wrap()),
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
                Expression::Not(Expression::Not(line.expression.clone().wrap()).wrap()),
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
                    ab[0].expression.clone().wrap(),
                    ab[1].expression.clone().wrap(),
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
                    ab[0].expression.clone().wrap(),
                    ab[1].expression.clone().wrap(),
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
                let poss_1 = Line::new(
                    assumptions,
                    self.len(),
                    Expression::Or(line.expression.clone().wrap(), Expression::Var(c).wrap()),
                    Rule::OrIntroduction,
                    deductions,
                );
                let mut poss_2 = poss_1.clone();
                poss_2.expression =
                    Expression::Or(Expression::Var(c).wrap(), line.expression.clone().wrap());
                let possibles = vec![Possible::new_single(poss_1), Possible::new_single(poss_2)];
                self.possibles.extend(possibles);
            }
        }
    }

    fn possible_or_e(&mut self) {
        // If the a line already contains an orEliminationAssumption and not orElimination, we can't add another one or we'll end up in an infinite loop
        for line in self.clone().node.lines.iter() {
            if line.rule == Rule::OrEliminationAssumption {
                // Here we've encountered a new sub-proof, we need to check whether this subproof ends in an orElimination
                // If it doesn't it means were still in the middle of a subproof and we can't add another orEliminationAssumption
                // If it does it means we've reached the end of the subproof and we can add another orEliminationAssumption
                let line_num = line.line_number;
                let mut found = false;
                for l in self.clone().node.lines.iter().skip(line_num) {
                    if l.rule == Rule::OrElimination && l.deduction_lines.contains(&line_num) {
                        found = true;
                        break;
                    }
                }
                if found {
                    continue;
                }
                return;
            }
        }
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
                let a_deduction_lines = match self.search_sub_proof(a_lines, None) {
                    Ok(lines) => lines,
                    Err(_) => continue,
                };
                let mut line_b = line.clone();
                line_b.expression = right.as_ref().clone();
                let mut b_lines = self.node.lines.clone();
                b_lines.push(line_b);
                // Try to contruct a proof for the conclusion using the new assumption (b)
                let b_deduction_lines = match self.search_sub_proof(b_lines, None) {
                    Ok(lines) => lines,
                    Err(_) => continue,
                };
                // Add the lines from this proof
                let mut resulting_lines = Vec::new();
                for l in a_deduction_lines.clone() {
                    resulting_lines.push(l.clone());
                }
                // Add the lines from the second proof
                for l in b_deduction_lines.clone() {
                    let mut l = l.clone();
                    for d in l.deduction_lines.iter_mut() {
                        if *d >= self.len() {
                            *d += a_deduction_lines.len();
                        }
                    }
                    l.line_number += a_deduction_lines.len();
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

    fn possible_cp(&mut self) {
        // If the conclusion is an implies we might need to use a conditional proof
        if let Expression::Implies(left, right) = &self.node.conclusion {
            for line in self.clone().node.lines.iter() {
                if line.rule == Rule::ConditionalProofAssumption {
                    // Here we've encountered a new sub-proof, we need to check whether this subproof ends in an orElimination
                    // If it doesn't it means were still in the middle of a subproof and we can't add another orEliminationAssumption
                    // If it does it means we've reached the end of the subproof and we can add another orEliminationAssumption
                    let line_num = line.line_number;
                    let mut found = false;
                    for l in self.clone().node.lines.iter().skip(line_num) {
                        if l.rule == Rule::ConditionalProof && l.deduction_lines.contains(&line_num)
                        {
                            found = true;
                            break;
                        }
                    }
                    if found {
                        continue;
                    }
                    return;
                }
            }
            // First we need to assume the left side
            let mut lines = self.node.lines.clone();
            let assumption = Line::new(
                vec![self.len()],
                self.len(),
                left.as_ref().clone(),
                Rule::ConditionalProofAssumption,
                vec![],
            );
            lines.push(assumption);
            // Then we need to construct a proof for the right side using the assumption
            let deduction_lines = match self.search_sub_proof(lines, Some(right.as_ref().clone())) {
                Ok(lines) => lines,
                Err(_) => return,
            };
            let rand = rand::thread_rng().gen_range(0..10000);
            if rand < 5 {
                for l in deduction_lines.clone() {
                    println!("{}", l);
                }
            }

            let mut deduction_line_nums = vec![self.len()];
            for l in deduction_lines.clone() {
                deduction_line_nums.push(l.line_number);
            }
            let mut assumptions = Vec::new();
            for l in deduction_lines.clone() {
                for a in l.assumption_lines {
                    if !assumptions.contains(&a) {
                        assumptions.push(a);
                    }
                }
            }
            let final_line = Line::new(
                assumptions,
                deduction_lines.last().unwrap().line_number + 1,
                self.node.conclusion.clone(),
                Rule::ConditionalProof,
                deduction_line_nums,
            );
            if rand < 5 {
                println!("Final Line: \n{}", final_line);
            }
            let mut deduction_lines = deduction_lines.clone();
            deduction_lines.push(final_line);
            let possible = Possible::new(deduction_lines);
            self.add_possible(possible);
        }
    }

    fn search_sub_proof(
        &self,
        lines: Vec<Line>,
        conclusion: Option<Expression>,
    ) -> Result<Vec<Line>, ()> {
        let mut proof = Proof::new_raw(
            self.node.assumptions().clone(),
            conclusion.unwrap_or(self.node.conclusion.clone()),
            lines,
            INNER_SEARCH_SETTINGS,
        );
        let result = proof.search();
        match result {
            Ok(_) => Ok(proof.get_deduction_lines()),
            Err(_) => Err(()),
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
                    for j in indices.iter_mut().take(i) {
                        *j = 0;
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

#[derive(Debug, Clone)]
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

fn find_vars_for_expression(expression: &Expression, vars: &mut Vec<char>) {
    let expressions = expression.list_expressions();
    for expression in expressions {
        if let Expression::Var(var) = expression {
            if !vars.contains(&var) {
                vars.push(var);
            }
        }
    }
    vars.sort();
    vars.dedup();
}

#[cfg(test)]
mod tests {
    use crate::expression::Expression;

    use super::find_vars_for_expression;

    #[test]
    fn check_vars() {
        let expression = Expression::Or(Expression::Var('Q').wrap(), Expression::Var('P').wrap());
        let expression_2 = Expression::Or(Expression::Var('P').wrap(), Expression::Var('Q').wrap());
        let mut vars = Vec::new();
        let mut vars_2 = Vec::new();
        find_vars_for_expression(&expression, &mut vars);
        find_vars_for_expression(&expression_2, &mut vars_2);
        assert_eq!(vars, vars_2);
    }
}
