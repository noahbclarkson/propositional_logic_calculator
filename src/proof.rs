use crate::{
    error::{ParserError, ProofError},
    lines::{Line, Rule},
    possible::PossibleFinder,
};
use std::{
    cell::RefCell,
    collections::VecDeque,
    fmt::{self, Display},
    rc::Rc,
};

use crate::{expression::Expression, parser::Parser};

#[derive(Debug, Clone)]
pub struct SearchSettings {
    pub(crate) max_line_length: usize,
    pub(crate) iterations: usize,
}

impl SearchSettings {
    const DEFAULT_MAX_LINE_LENGTH: usize = 15;
    const DEFAULT_ITERATIONS: usize = 50000;

    fn from_incomplete(max_line_length: Option<usize>, iterations: Option<usize>) -> Self {
        Self {
            max_line_length: max_line_length.unwrap_or(Self::DEFAULT_MAX_LINE_LENGTH),
            iterations: iterations.unwrap_or(Self::DEFAULT_ITERATIONS),
        }
    }
}

impl Default for SearchSettings {
    fn default() -> Self {
        Self {
            max_line_length: Self::DEFAULT_MAX_LINE_LENGTH,
            iterations: Self::DEFAULT_ITERATIONS,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Proof {
    assumptions: Vec<Expression>,
    conclusion: Expression,
    pub(crate) lines: Vec<Line>,
    settings: Rc<SearchSettings>,
    iterations: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SearchState {
    FinishedIteration,
    FinishedProof,
    Searching,
    DeadEnd,
    MaximumLines,
    MaximumIteration,
}

impl Display for SearchState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SearchState::FinishedIteration => write!(f, "Finished iteration"),
            SearchState::FinishedProof => write!(f, "Finished proof"),
            SearchState::Searching => write!(f, "Searching"),
            SearchState::DeadEnd => write!(f, "Dead end"),
            SearchState::MaximumLines => write!(f, "Maximum lines"),
            SearchState::MaximumIteration => write!(f, "Maximum iteration"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SearchNode {
    pub parent: Option<Rc<RefCell<Self>>>,
    pub children: Vec<Rc<RefCell<Self>>>,
    pub lines: Vec<Line>,
    pub conclusion: Expression,
    pub settings: Rc<SearchSettings>,
}

pub struct ProofBuilder {
    assumptions: Vec<Expression>,
    conclusion: Expression,
    max_line_length: Option<usize>,
    iterations: Option<usize>,
}

impl ProofBuilder {
    pub fn new(assumptions: Vec<Expression>, conclusion: Expression) -> Self {
        Self {
            assumptions,
            conclusion,
            max_line_length: None,
            iterations: None,
        }
    }

    pub fn max_line_length(mut self, max_line_length: usize) -> Self {
        self.max_line_length = Some(max_line_length);
        self
    }

    pub fn iterations(mut self, iterations: usize) -> Self {
        self.iterations = Some(iterations);
        self
    }

    pub fn build(self) -> Proof {
        Proof::new_raw(
            self.assumptions.clone(),
            self.conclusion,
            create_assumption_lines(self.assumptions),
            SearchSettings::from_incomplete(self.max_line_length, self.iterations),
        )
    }
}

impl Proof {
    pub(crate) fn new_raw(
        assumptions: Vec<Expression>,
        conclusion: Expression,
        lines: Vec<Line>,
        settings: SearchSettings,
    ) -> Self {
        Proof {
            assumptions,
            conclusion,
            lines,
            settings: Rc::new(settings),
            iterations: 0,
        }
    }

    pub fn new(assumptions: Vec<Expression>, conclusion: Expression) -> Result<Self, ProofError> {
        let lines = create_assumption_lines(assumptions.clone());
        Ok(Proof {
            assumptions,
            conclusion,
            lines,
            settings: Rc::new(SearchSettings::default()),
            iterations: 0,
        })
    }

    pub fn search(&mut self) -> Result<(), ProofError> {
        let head = SearchNode::new(
            self.lines.clone(),
            self.conclusion.clone(),
            self.settings.clone(),
        );
        self.iterations = 0;
        let result = search(head.clone(), self);
        match result {
            Ok(result) => {
                self.lines = result;
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    /// Get all lines that are not assumptions
    pub fn get_deduction_lines(&self) -> Vec<Line> {
        self.lines
            .iter()
            .filter(|x| x.rule != Rule::Assumption)
            .cloned()
            .collect()
    }
}

impl Display for Proof {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut nested_proof_level = 0;

        writeln!(f, "Assumptions: [{}]", join_expressions(&self.assumptions))?;
        writeln!(f, "Conclusion: {}", self.conclusion)?;
        writeln!(f, "Total Proof Steps: {}", self.lines.len())?;
        writeln!(f, "Proof Steps:")?;

        for line in &self.lines {
            // Check if the line starts or ends a nested proof
            match line.rule {
                Rule::OrEliminationAssumption => nested_proof_level += 1,
                Rule::OrElimination => nested_proof_level -= 1,
                Rule::ConditionalProofAssumption => nested_proof_level += 1,
                Rule::ConditionalProof => nested_proof_level -= 1,
                _ => (),
            }

            // Apply indentation if in a nested proof
            let indent = if nested_proof_level > 0 {
                "  ".repeat(nested_proof_level)
            } else {
                "".to_string()
            };
            writeln!(f, "{}{}", indent, line)?;
        }

        Ok(())
    }
}

fn join_expressions(expressions: &[Expression]) -> String {
    expressions
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<String>>()
        .join(", ")
}

impl Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Base line format with assumption lines, line number, and expression
        write!(
            f,
            "Line {}: {} [{}] using {}",
            self.line_number + 1,
            self.expression,
            join(&self.assumption_lines),
            self.rule,
        )?;

        // Append 'from lines' only if there are deduction lines
        if !self.deduction_lines.is_empty() {
            write!(f, " from lines {}", join(&self.deduction_lines))?;
        }

        Ok(())
    }
}

fn join(array: &[usize]) -> String {
    let mut array = array.to_owned();
    array.dedup();
    array.sort();
    array
        .iter()
        .map(|x| x + 1)
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(", ")
}

impl SearchNode {
    pub fn new(
        lines: Vec<Line>,
        conclusion: Expression,
        settings: Rc<SearchSettings>,
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(SearchNode {
            parent: None,
            children: vec![],
            lines,
            conclusion,
            settings,
        }))
    }

    pub fn is_complete(&self) -> bool {
        self.lines.iter().any(|x| x.expression == self.conclusion)
    }

    pub fn assumptions(&self) -> Vec<Expression> {
        self.lines
            .iter()
            .filter(|x| x.rule == Rule::Assumption)
            .map(|x| x.expression.clone())
            .collect()
    }
}

fn search(head: Rc<RefCell<SearchNode>>, proof: &mut Proof) -> Result<Vec<Line>, ProofError> {
    let mut queue = VecDeque::new();
    queue.push_back(head.clone());
    while let Some(current_rc) = queue.pop_front() {
        let current = current_rc.borrow();

        if current.is_complete() {
            return Ok(current.lines.clone());
        }

        if current.lines.len() > current.settings.max_line_length {
            continue;
        }

        proof.iterations += 1;
        if proof.iterations > proof.settings.iterations {
            return Err(ProofError::SearchError(SearchState::MaximumIteration));
        }

        let mut finder = PossibleFinder::new(current.clone());
        finder.find();
        let possibles = finder.possibles();
        if possibles.is_empty() {
            continue;
        }

        for possible in possibles {
            let last = possible.lines.last().unwrap();
            match last.matches_expression(&current.conclusion) {
                true => {
                    let mut new_lines = current.lines.clone();
                    new_lines.extend(possible.lines.clone());
                    return Ok(new_lines);
                }
                false => (),
            }
        }
        for possible in possibles {
            let mut new_lines = current.lines.clone();
            new_lines.extend(possible.lines.clone());
            let new_node = SearchNode::new(
                new_lines,
                current.conclusion.clone(),
                current.settings.clone(),
            );
            queue.push_back(new_node);
        }
    }

    // Work out which error to return
    let current = head.borrow();
    if current.lines.len() > current.settings.max_line_length {
        return Err(ProofError::SearchError(SearchState::MaximumLines));
    }
    Err(ProofError::SearchError(SearchState::DeadEnd))
}

pub fn create_assumption_lines(assumptions: Vec<Expression>) -> Vec<Line> {
    assumptions
        .iter()
        .enumerate()
        .map(|(i, x)| Line::new(vec![i], i, x.clone(), Rule::Assumption, vec![]))
        .collect()
}

pub fn parse_expression(expression: &str) -> Result<Expression, ParserError> {
    let mut parser = Parser::new(expression);
    parser.parse()
}
