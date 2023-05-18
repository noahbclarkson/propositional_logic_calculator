use std::fmt::{self, Display};
use std::rc::Rc;

use crate::expression::Expression;
use crate::lines::{Line, Rule};

const ITERATIONS: usize = 10_000;
const MAX_LINE_LENGTH: usize = 30;

#[derive(Debug, PartialEq, Clone)]
pub struct Proof {
    pub assumptions: Vec<Expression>,
    pub conclusion: Expression,
    pub lines: Vec<Line>,
    pub max_line_length: usize,
}

impl Proof {
    pub fn new(assumptions: Vec<String>, conclusion: String, max_lines: Option<usize>) -> Self {
        let assumptions: Vec<Expression> =
            assumptions.iter().map(|x| parse_expression(x)).collect();
        let conclusion = parse_expression(&conclusion);

        // Add lines for each assumption
        let mut lines = Vec::new();
        for (i, assumption) in assumptions.iter().enumerate() {
            lines.push(Line::new(
                vec![i + 1],
                i + 1,
                assumption.clone(),
                Rule::Assumption,
                Vec::new(),
            ));
        }

        let mut max_line_length = MAX_LINE_LENGTH;
        if !max_lines.is_none() {
            max_line_length = max_lines.unwrap();
        }

        Proof {
            assumptions,
            conclusion,
            lines,
            max_line_length,
        }
    }

    pub fn run(&mut self) {
        let mut lowest_length = MAX_LINE_LENGTH + 1;
        let mut completed = false;
        let original = self.clone();
        for _ in 0..ITERATIONS {
            let (c, l) = self.iteration(&completed, &lowest_length, original.clone());
            completed = c;
            lowest_length = l;
        }
        if completed {
            println!("Proof found!");
            println!("Proof length: {}", self.lines.len());
        } else {
            println!("No proof found");
        }
    }

    pub fn iteration(
        &mut self,
        completed: &bool,
        lowest_length: &usize,
        original: Proof,
    ) -> (bool, usize) {
        let mut proof = original.clone();
        let mut c = completed.clone();
        let mut l = lowest_length.clone();
        while proof.lines.len() < self.max_line_length {
            proof.apply();
            // Check if the proof is complete
            if proof.is_complete() {
                // Check if the proof is shorter than the current shortest proof
                c = true;

                if proof.lines.len() < l {
                    l = proof.lines.len();
                    println!("Proof found of length {}", proof.lines.len());
                    self.lines = proof.lines;
                    self.max_line_length = self.lines.len();
                }
                break;
            }
        }
        // 1 in 50 chance to randomly print the proof as an example
        if rand::random::<usize>() % 10 == 0 {
            println!("{}", self);
        }
        (c, l)
    }

    pub fn apply(&mut self) {
        // Find all the possible rules
        let mut possible_rules = Vec::new();
        if self.find_mpp().len() > 0 {
            // Push Rule::Modus ponens to the list of possible rules x times where x is the number of times modus ponens can be applied
            for _ in 0..self.find_mpp().len() {
                possible_rules.push(Rule::ModusPonens);
            }
        }
        if self.find_mtt().len() > 0 {
            for _ in 0..self.find_mtt().len() {
                possible_rules.push(Rule::ModusTollens);
            }
        }
        if self.find_and_elimination().len() > 0 {
            for _ in 0..self.find_and_elimination().len() {
                possible_rules.push(Rule::AndElimination);
            }
        }
        if self.find_double_negation_remove().len() > 0 {
            for _ in 0..self.find_double_negation_remove().len() {
                possible_rules.push(Rule::DoubleNegation);
            }
        }
        if self.find_or_elimination().len() > 0 {
            for _ in 0..self.find_or_elimination().len() {
                possible_rules.push(Rule::OrElimination);
            }
        }
        possible_rules.push(Rule::DoubleNegation);
        if self.lines.len() > 1 {
            possible_rules.push(Rule::AndIntroduction);
        }
        if self.lines.len() > 1 {
            possible_rules.push(Rule::OrIntroduction);
        }
        if rand::random::<usize>() % 10 == 0 && self.max_line_length - self.lines.len() > 3 {
            possible_rules.push(Rule::ConditionalProof);
        }

        let rand = rand::random::<usize>() % possible_rules.len();
        let rule = &possible_rules[rand];

        match rule.clone() {
            Rule::ModusPonens => {
                // Find the places where modus ponens is valid
                let mpp_indices = self.find_mpp();
                let rand = rand::random::<usize>() % mpp_indices.len();
                let (line1, line2) = mpp_indices[rand];
                // See if mpp has been applied here before
                let mut applied = false;
                for line in self.lines.iter() {
                    if line.rule == Rule::ModusPonens
                        && line.deduction_lines == vec![line1 + 1, line2 + 1]
                    {
                        applied = true;
                        break;
                    }
                }
                if !applied {
                    self.mpp(line1 + 1, line2 + 1);
                }
            }
            Rule::ModusTollens => {
                // Find the places where modus tollens is valid
                let mtt_indices = self.find_mtt();
                let rand = rand::random::<usize>() % mtt_indices.len();
                let (line1, line2) = mtt_indices[rand];
                self.mtt(line1 + 1, line2 + 1);
            }
            Rule::AndElimination => {
                // Find the places where and elimination is valid
                let and_elimination_indices = self.find_and_elimination();
                let rand = rand::random::<usize>() % and_elimination_indices.len();
                let line = and_elimination_indices[rand];
                self.and_elimination(line + 1);
            }
            Rule::DoubleNegation => {
                // Find the places where double negation is valid
                let double_negation_indices = self.find_double_negation_remove();
                let valid_remove = double_negation_indices.len() > 0;
                let rand = rand::random::<usize>() % 2;
                if valid_remove && rand == 0 {
                    let rand = rand::random::<usize>() % double_negation_indices.len();
                    let line = double_negation_indices[rand];
                    self.double_negation(false, line + 1);
                } else {
                    let rand = rand::random::<usize>() % self.lines.len();
                    self.double_negation(true, rand + 1);
                }
            }
            Rule::Assumption => {
                todo!()
            }
            Rule::AndIntroduction => {
                // Select two random lines and & them together (not the same lines)
                let line1 = rand::random::<usize>() % self.lines.len();
                let mut line2 = rand::random::<usize>() % self.lines.len();
                while line1 == line2 {
                    line2 = rand::random::<usize>() % self.lines.len();
                }
                self.and_introduction(line1 + 1, line2 + 1);
            }
            Rule::OrIntroduction => {
                let mut expressions = Vec::new();
                for line in self.lines.iter() {
                    expressions.extend(line.expression.list_expressions());
                }
                expressions.extend(self.conclusion.list_expressions());
                expressions.dedup();
                let rand = rand::random::<usize>() % expressions.len();
                let expression = expressions[rand].clone();
                let rand = rand::random::<usize>() % self.lines.len();
                let line = rand + 1;
                // Ensure that the new expression is not the same as the line
                if self.lines[line - 1].expression != expression {
                    self.or_introduction(line, expression);
                }
            }
            Rule::ConditionalProof => {
                // For the conditional proof we will need to find a valid expression to assume
                // For this we will find a random expression that isn't already assumed and isn't the desired conclusion
                // We then add this expression to the assumptions and then run .apply on the proof a random number of times or
                // either until the proof is complete or until we run out of lines
                let mut expressions = Vec::new();
                for line in self.lines.iter() {
                    expressions.extend(line.expression.list_expressions());
                    // Find "line" in expressions and remove it
                    let index = expressions
                        .iter()
                        .position(|x| *x == line.expression)
                        .unwrap();
                    expressions.remove(index);
                }
                expressions.extend(self.conclusion.list_expressions());
                let index = expressions
                    .iter()
                    .position(|x| *x == self.conclusion)
                    .unwrap();
                expressions.remove(index);
                expressions.dedup();
                // Now we add the expression as an assumption line
                let rand = rand::random::<usize>() % expressions.len();
                let expression = expressions[rand].clone();
                let current_line = self.lines.len();
                let new_line = Line {
                    assumption_lines: vec![current_line + 1],
                    line_number: current_line + 1,
                    expression,
                    rule: Rule::Assumption,
                    deduction_lines: Vec::new(),
                };
                self.lines.push(new_line);
                // Now we apply the proof a random number of times (from 2 to the max_lines - current_line)
                let rand = rand::random::<usize>() % (self.max_line_length - current_line - 1) + 2;
                let mut possible_cp = None;
                for _ in 0..rand {
                    self.apply();
                    while self.is_complete() {
                        // We can't find a valid proof by based on the assumption
                        // We can only find it by using the assumption to show that the assumption implies the conclusion of the CP
                        // Therefore we must remove this line and try again
                        self.lines.pop();
                        self.apply();
                    }
                    // However, we want to stop the conditional proof if the assumption->latest_line is the conclusion
                    possible_cp = Some(Expression::Implies(
                        Rc::new(self.lines[current_line - 1].expression.clone()),
                        Rc::new(self.lines[self.lines.len() - 1].expression.clone()),
                    ));
                    if possible_cp.clone().unwrap() == self.conclusion {
                        break;
                    }
                }
                // Now regardless of whether we found a valid proof or not we must add the CP line
                // We also want to get the assumptions used in the CP, this should include all the lines from the assumption to the last line (not including the assumption)
                let mut assumption_lines = Vec::new();
                for line in (current_line + 1)..self.lines.len() - 1 {
                    assumption_lines.extend(self.lines[line].assumption_lines.clone());
                }
                assumption_lines.dedup();
                // The deduction lines are the assumption line and the last line
                let deduction_lines = vec![current_line, self.lines.len()];
                let cp_line = Line {
                    assumption_lines,
                    line_number: self.lines.len() + 1,
                    expression: possible_cp.unwrap(),
                    rule: Rule::ConditionalProof,
                    deduction_lines,
                };
                self.lines.push(cp_line);
            }
            Rule::OrElimination => {
                // Find the places where or elimination is valid
                let or_elimination_indices = self.find_or_elimination();
                let rand = rand::random::<usize>() % or_elimination_indices.len();
                let (line1, line2, line3) = or_elimination_indices[rand];
                self.or_elimination(line1 + 1, line2 + 1, line3 + 1);
            }
            Rule::ReductioAdAbsurdium => {
                todo!()
            }
        }
    }

    pub fn is_complete(&self) -> bool {
        // Check if the proof is complete
        for line in &self.lines {
            if line.expression == self.conclusion {
                return true;
            }
        }
        false
    }

    pub fn find_mpp(&self) -> Vec<(usize, usize)> {
        let mut mpp_indices = Vec::new();
        // Find the lines where mpp is valid
        for i in 0..self.lines.len() {
            for j in 0..self.lines.len() {
                if let Expression::Implies(left, _) = &self.lines[i].expression {
                    if **left == self.lines[j].expression {
                        mpp_indices.push((i, j));
                    }
                }
            }
        }
        mpp_indices
    }

    pub fn find_mtt(&self) -> Vec<(usize, usize)> {
        let mut mtt_indices = Vec::new();
        // Find the lines where mtt is valid
        for i in 0..self.lines.len() {
            for j in 0..self.lines.len() {
                if let Expression::Implies(_, q) = &self.lines[i].expression {
                    if let Expression::Not(not_q) = &self.lines[j].expression {
                        if **q == **not_q {
                            mtt_indices.push((i, j));
                        }
                    }
                }
            }
        }
        mtt_indices
    }

    pub fn find_and_elimination(&self) -> Vec<usize> {
        let mut and_elimination_indices = Vec::new();
        // Find the lines where and elimination is valid
        for i in 0..self.lines.len() {
            if let Expression::And(_, _) = &self.lines[i].expression {
                and_elimination_indices.push(i);
            }
        }
        and_elimination_indices
    }

    pub fn find_double_negation_remove(&self) -> Vec<usize> {
        let mut double_negation_indices = Vec::new();
        // Find the lines where double negation (removing two nots) is valid
        for i in 0..self.lines.len() {
            if let Expression::Not(not_not_p) = &self.lines[i].expression {
                if let Expression::Not(_) = &**not_not_p {
                    double_negation_indices.push(i);
                }
            }
        }
        double_negation_indices
    }

    pub fn find_or_elimination(&self) -> Vec<(usize, usize, usize)> {
        let mut or_elimination_indices = Vec::new();
        for i in 0..self.lines.len() {
            if let Expression::Or(or_left, or_right) = &self.lines[i].expression {
                for j in 0..self.lines.len() {
                    if let Expression::Implies(implies_left, implies_right) =
                        &self.lines[j].expression
                    {
                        if implies_left == or_left {
                            for k in 0..self.lines.len() {
                                if let Expression::Implies(implies_left2, implies_right2) =
                                    &self.lines[k].expression
                                {
                                    if implies_left2 == or_right && implies_right == implies_right2
                                    {
                                        or_elimination_indices.push((i, j, k));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        or_elimination_indices
    }

    pub fn mpp(&mut self, line1: usize, line2: usize) {
        let (expression1, dependencies1) = self.get_line_expression(line1);
        let (expression2, dependencies2) = self.get_line_expression(line2);

        // Extract the rhs of the implies expression
        let new_expression = match expression1 {
            Expression::Implies(_, rhs) => rhs.as_ref().clone(),
            _ => panic!("Expression1 must be an Implies expression"),
        };

        if let Expression::Implies(lhs, _) = expression1 {
            if lhs.as_ref().clone() != expression2.clone() {
                panic!("The lhs of expression1 must be equal to expression2");
            }
        }

        // Get the lines that the new expression depends on
        let mut dependencies = dependencies1.clone();
        dependencies.extend(dependencies2.clone());
        dependencies.sort();
        dependencies.dedup();

        // Create the new line
        let new_line = Line::new(
            dependencies,
            self.lines.len() + 1,
            new_expression,
            Rule::ModusPonens,
            vec![line1, line2],
        );

        // Add the new line to the proof
        self.lines.push(new_line);
    }

    pub fn mtt(&mut self, line1: usize, line2: usize) {
        // Get the expressions from the lines
        let (expression1, dependencies1) = self.get_line_expression(line1);
        let (expression2, dependencies2) = self.get_line_expression(line2);

        // Get the lines that the new expression depends on
        let mut dependencies = dependencies1.clone();
        dependencies.extend(dependencies2.clone());
        dependencies.sort();
        dependencies.dedup();

        // Find the new expression after applying MTT
        let new_expression = match (expression1, expression2) {
            (Expression::Implies(p, q), Expression::Not(not_q)) if **q == **not_q => {
                Expression::Not(Rc::new((**p).clone()))
            }
            (Expression::Not(not_p), Expression::Implies(p, q)) if **not_p == **p => {
                Expression::Not(Rc::new((**q).clone()))
            }
            _ => panic!(
                "Invalid expressions for MTT on lines {} and {}",
                line1, line2
            ),
        };

        // Create the new line
        let new_line = Line::new(
            dependencies,
            self.lines.len() + 1,
            new_expression,
            Rule::ModusTollens,
            vec![line1, line2],
        );

        // Add the new line to the proof
        self.lines.push(new_line);
    }

    pub fn double_negation(&mut self, add: bool, line: usize) {
        let (expression, dependencies) = self.get_line_expression(line);

        // Create the new expression with double negation added or removed
        let new_expression = if add {
            Expression::Not(Rc::new(Expression::Not(Rc::new(expression.clone()))))
        } else {
            match expression {
                Expression::Not(inner) => {
                    if let Expression::Not(inner_inner) = &**inner {
                        (**inner_inner).clone()
                    } else {
                        panic!("No double negation found on line {}", line);
                    }
                }
                _ => panic!("No double negation found on line {}", line),
            }
        };

        // Create the new line
        let new_line = Line::new(
            dependencies.clone(),
            self.lines.len() + 1,
            new_expression,
            Rule::DoubleNegation,
            vec![line],
        );

        // Add the new line to the proof
        self.lines.push(new_line);
    }

    pub fn and_introduction(&mut self, line1: usize, line2: usize) {
        let (expression1, dependencies1) = self.get_line_expression(line1);
        let (expression2, dependencies2) = self.get_line_expression(line2);

        // Get the lines that the new expression depends on
        let mut dependencies = dependencies1.clone();
        dependencies.extend(dependencies2.clone());
        dependencies.sort();
        dependencies.dedup();

        // Create the new expression with AND introduction
        let new_expression =
            Expression::And(Rc::new(expression1.clone()), Rc::new(expression2.clone()));

        // Create the new line
        let new_line = Line::new(
            dependencies,
            self.lines.len() + 1,
            new_expression,
            Rule::AndIntroduction,
            vec![line1, line2],
        );

        // Add the new line to the proof
        self.lines.push(new_line);
    }

    pub fn or_introduction(&mut self, line: usize, new_expr: Expression) {
        let (expression, dependencies) = self.get_line_expression(line);

        // Create the new expression with OR introduction
        let new_expression = Expression::Or(Rc::new(expression.clone()), Rc::new(new_expr.clone()));

        // Create the new line
        let new_line = Line::new(
            dependencies.clone(),
            self.lines.len() + 1,
            new_expression,
            Rule::OrIntroduction,
            vec![line],
        );

        // Add the new line to the proof
        self.lines.push(new_line);
    }

    pub fn and_elimination(&mut self, line: usize) {
        let (expression, dependencies) = self.get_line_expression(line);

        // Check if the expression is an AND expression
        let (expression1, expression2) = if let Expression::And(left, right) = expression {
            (left.clone(), right.clone())
        } else {
            panic!("Expression on line {} is not an AND expression", line);
        };

        // Create the new lines
        let new_line1 = Line::new(
            dependencies.clone(),
            self.lines.len() + 1,
            expression1.as_ref().clone(),
            Rule::AndElimination,
            vec![line],
        );

        let new_line2 = Line::new(
            dependencies.clone(),
            self.lines.len() + 2,
            expression2.as_ref().clone(),
            Rule::AndElimination,
            vec![line],
        );

        // Add the new lines to the proof
        self.lines.push(new_line1);
        self.lines.push(new_line2);
    }

    pub fn or_elimination(&mut self, or_line: usize, premise1_line: usize, premise2_line: usize) {
        // Line 1 is the or line, line 2 is the implies line with the left side of the or, line 3 is the implies line with the right side of the or
        let (or_expression, dependencies_or) = self.get_line_expression(or_line);
        let (premise1_expression, dependencies_premise1) = self.get_line_expression(premise1_line);
        let (premise2_expression, dependencies_premise2) = self.get_line_expression(premise2_line);

        // Check if the expression is an OR expression
        let (expression1, expression2) = if let Expression::Or(left, right) = or_expression {
            (left.clone(), right.clone())
        } else {
            panic!("Expression on line {} is not an OR expression", or_line);
        };

        let left_true = if let Expression::Implies(left, _) = premise1_expression {
            left.as_ref().clone() == expression1.as_ref().clone()
        } else {
            false
        };

        let right_true = if let Expression::Implies(left, _) = premise2_expression {
            left.as_ref().clone() == expression2.as_ref().clone()
        } else {
            false
        };

        if !left_true || !right_true {
            // Print the expressions
            panic!("The OR expression does not have matching premises");
        }

        // Get the lines that the new expression depends on
        let mut dependencies = dependencies_or.clone();
        dependencies.extend(dependencies_premise1.clone());
        dependencies.extend(dependencies_premise2.clone());
        dependencies.sort();
        dependencies.dedup();

        let right = if let Expression::Implies(_, right) = premise1_expression {
            right.clone()
        } else {
            panic!(
                "Expression on line {} is not an implies expression",
                premise1_line
            );
        };

        // Create the new line
        let new_line = Line::new(
            dependencies,
            self.lines.len() + 1,
            right.as_ref().clone(),
            Rule::OrElimination,
            vec![or_line, premise1_line, premise2_line],
        );

        // Add the new line to the proof
        self.lines.push(new_line);
    }

    fn get_line_expression(&self, line: usize) -> (&Expression, Vec<usize>) {
        let line_index = line - 1;
        if line_index >= self.lines.len() {
            panic!("Invalid line number: {}", line);
        }
        let expression = &self.lines[line_index].expression;
        let dependencies = &self.lines[line_index].assumption_lines;
        (&expression, dependencies.clone())
    }
}

impl Display for Proof {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let assumptions = self
            .assumptions
            .iter()
            .map(|a| a.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "{} / {}", assumptions, self.conclusion)?;
        for line in &self.lines {
            write!(f, "\n{}", line)?;
        }
        Ok(())
    }
}

fn parse_expression(input: &str) -> Expression {
    let mut chars = input.chars().peekable();
    let mut stack = Vec::new();

    while let Some(c) = chars.next() {
        match c {
            '(' => {
                let bracket = extract_bracket_contents(&mut chars);
                stack.push(parse_expression(&bracket));
            }
            'A'..='Z' => stack.push(Expression::Var(c.to_string())),
            '-' => {
                let mut not_count = 1;
                while let Some('-') = chars.peek() {
                    not_count += 1;
                    chars.next(); // Consume the additional '-'
                }
                let next = chars.next().unwrap();
                let mut right = if next == '(' {
                    let bracket = extract_bracket_contents(&mut chars);
                    parse_expression(&bracket)
                } else {
                    Expression::Var(next.to_string())
                };

                for _ in 0..not_count {
                    right = Expression::Not(Rc::new(right));
                }
                stack.push(right);
            }
            '&' | 'v' | '>' => {
                let left = stack.pop().unwrap();
                while let Some(&next_char) = chars.peek() {
                    if next_char == '('
                        || next_char == '-'
                        || (next_char >= 'A' && next_char <= 'Z')
                    {
                        break;
                    }
                    chars.next(); // Consume the spaces
                }
                let remaining_chars: String = chars.collect();
                let right = parse_expression(&remaining_chars);
                let expr = match c {
                    '&' => Expression::And(Rc::new(left), Rc::new(right)),
                    'v' => Expression::Or(Rc::new(left), Rc::new(right)),
                    '>' => Expression::Implies(Rc::new(left), Rc::new(right)),
                    _ => panic!("Invalid operator"),
                };
                stack.push(expr);
                break;
            }
            ' ' => (),
            _ => panic!("Invalid expression: \"{}\"", c),
        }
    }
    stack.pop().unwrap()
}

fn extract_bracket_contents(chars: &mut std::iter::Peekable<std::str::Chars>) -> String {
    let mut bracket = String::new();
    let mut bracket_count = 1;
    while let Some(c) = chars.next() {
        match c {
            '(' => bracket_count += 1,
            ')' => bracket_count -= 1,
            _ => (),
        }
        if bracket_count == 0 {
            break;
        }
        bracket.push(c);
    }
    bracket
}
