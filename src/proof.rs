use crate::{lines::{possible_lines, Line, Rule}, error::{ParserError, ProofError}};
use derive_builder::Builder;
use enum_iterator::all;
use std::{
    cell::RefCell,
    collections::VecDeque,
    fmt::{self, Display},
    rc::Rc,
    sync::atomic::AtomicUsize,
    sync::atomic::Ordering::SeqCst,
};

use crate::{expression::Expression, parser::Parser};

const DEFAULT_MAX_LINE_LENGTH: usize = 6;
const DEFAULT_ITERATIONS: usize = 10000;

static ITERATIONS: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone, Builder)]
pub struct SearchSettings {
    #[builder(default = "DEFAULT_MAX_LINE_LENGTH")]
    max_line_length: usize,
    #[builder(default = "DEFAULT_ITERATIONS")]
    iterations: usize,
}

#[derive(Debug, Clone)]
pub struct Proof {
    assumptions: Vec<Expression>,
    conclusion: Expression,
    lines: Vec<Line>,
    settings: Rc<SearchSettings>,
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

pub struct SearchNode {
    pub parent: Option<Rc<RefCell<Self>>>,
    pub children: Vec<Rc<RefCell<Self>>>,
    pub lines: Vec<Line>,
    pub conclusion: Expression,
    pub settings: Rc<SearchSettings>,
}

impl Proof {
    pub fn new(
        assumptions: Vec<String>,
        conclusion: String,
        settings: Option<SearchSettings>,
    ) -> Result<Self, ProofError> {
        let assumptions = assumptions
            .iter()
            .map(|x| parse_expressions(x))
            .collect::<Result<Vec<Expression>, ParserError>>()?;
        let conclusion = parse_expressions(&conclusion)?;
        let lines = create_assumption_lines(assumptions.clone());
        Ok(Proof {
            assumptions,
            conclusion,
            lines,
            settings: Rc::new(
                settings.unwrap_or_else(|| SearchSettingsBuilder::default().build().unwrap()),
            ),
        })
    }

    pub fn search(&mut self) -> Result<(), ProofError> {
        let head = SearchNode::new(
            self.lines.clone(),
            self.conclusion.clone(),
            self.settings.clone(),
        );
        let result = search(head.clone());
        ITERATIONS.store(0, std::sync::atomic::Ordering::SeqCst);
        match result {
            Ok(result) => {
                self.lines = result;
                Ok(())
            }
            Err(err) => return Err(err),
        }
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

    fn find_possible_lines(&self) -> Vec<Line> {
        all::<Rule>()
            .skip(1)
            // TODO: Implement these rules
            .filter(|x| {
                x.clone() != Rule::ConditionalProof && x.clone() != Rule::ReductioAdAbsurdium && x.clone() != Rule::OrElimination
            })
            .flat_map(|x| possible_lines(self.lines.clone(), x))
            .collect::<Vec<Line>>()
    }
}

fn search(head: Rc<RefCell<SearchNode>>) -> Result<Vec<Line>, ProofError> {
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

        ITERATIONS.fetch_add(1, SeqCst);
        if ITERATIONS.load(SeqCst) > current.settings.iterations {
            break;
        }

        let possibles = current.find_possible_lines();
        for possible_line in possibles {
            let mut new_lines = current.lines.clone();
            new_lines.push(possible_line);
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
    Err(ProofError::SearchError(SearchState::MaximumIteration))
}

pub fn create_assumption_lines(assumptions: Vec<Expression>) -> Vec<Line> {
    assumptions
        .iter()
        .enumerate()
        .map(|(i, x)| Line::new(vec![i], i, x.clone(), Rule::Assumption, vec![]))
        .collect()
}

pub fn parse_expressions(expression: &str) -> Result<Expression, ParserError> {
    let mut parser = Parser::new(expression);
    parser.parse()
}
