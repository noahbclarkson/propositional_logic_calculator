use crate::proof::SearchState;

#[derive(Debug, thiserror::Error)]
pub enum ParserError {
    #[error("'{0}' did not match one of the valid characters.")]
    InvalidExpression(char),
    #[error("Empty expression")]
    EmptyExpression,
    #[error("Expected left operand")]
    ExpectedLeftOperand,
    #[error("Expected expression after '-'")]
    ExpectedExpressionAfterNegation,
    #[error("Invalid operator: '{0}'")]
    InvalidOperator(char),
    #[error("Unmatched parentheses in expression: {0} at bracket {1}")]
    UnmatchedParentheses(String, usize),
}

#[derive(Debug, thiserror::Error)]
pub enum ProofError {
    #[error("Parser error: {0}")]
    ParserError(#[from] ParserError),
    #[error("Search error: {0}")]
    SearchError(SearchState),
}