// error.rs:
//
// This module defines custom error types used in the parser and proof modules.
// These error types provide more detailed and context-specific error messages,
// improving the debugging experience and user feedback.

use crate::proof::SearchState;

/// Represents errors that can occur during parsing of logical expressions.
#[derive(Debug, thiserror::Error)]
pub enum ParserError {
    /// Error for characters that don't match valid characters in an expression.
    /// The offending character is provided in the error message.
    #[error("'{0}' did not match one of the valid characters.")]
    InvalidExpression(char),

    /// Error for situations where an expression is expected but none is provided.
    #[error("Empty expression")]
    EmptyExpression,

    /// Error for missing left operand in a binary operation.
    #[error("Expected left operand")]
    ExpectedLeftOperand,

    /// Error for missing expression after a negation '-' operator.
    #[error("Expected expression after '-'")]
    ExpectedExpressionAfterNegation,

    /// Error for an invalid operator in the expression.
    /// The invalid operator character is provided in the error message.
    #[error("Invalid operator: '{0}'")]
    InvalidOperator(char),

    /// Error for unmatched parentheses in an expression.
    /// The partially parsed expression and the bracket index are provided.
    #[error("Unmatched parentheses in expression: {0} at bracket {1}")]
    UnmatchedParentheses(String, usize),
}

/// Represents errors that can occur during the proof process.
#[derive(Debug, thiserror::Error)]
pub enum ProofError {
    /// Wraps a `ParserError`, indicating an error during parsing.
    #[error("Parser error: {0}")]
    ParserError(#[from] ParserError),

    /// Represents errors occurring during the search state of proof generation.
    #[error("Search error: {0}")]
    SearchError(SearchState),
}
