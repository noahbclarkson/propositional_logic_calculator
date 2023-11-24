use std::{iter::Peekable, str::Chars};

use crate::{error::ParserError, expression::Expression};

/// The `Parser` struct is responsible for parsing logical expressions represented as strings into an abstract syntax tree (AST).
/// It works with basic logical operators and handles nested expressions.
pub struct Parser<'a> {
    // Stream of characters from the input string to be parsed.
    chars: Peekable<Chars<'a>>,
}

impl<'a> Parser<'a> {
    /// Creates a new instance of `Parser`.
    ///
    /// # Arguments
    ///
    /// * `input`: A string slice representing the logical expression to be parsed.
    pub fn new(input: &'a str) -> Self {
        Parser {
            chars: input.chars().peekable(),
        }
    }

    /// Parses a logical expression into an `Expression` enum.
    ///
    /// The function processes a string slice representing a logical expression
    /// and constructs a corresponding abstract syntax tree (AST) represented by the `Expression` enum.
    /// It supports basic logical operators such as AND ('&'), OR ('|' or 'v'), IMPLIES ('>'), and NOT ('-').
    /// The function handles nested expressions and respects the standard precedence of logical operators.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - A mutable reference to the `Parser` instance, allowing the function to consume the input.
    ///
    /// # Returns
    ///
    /// This function returns a `Result<Expression, String>`. On successful parsing, it returns `Ok(Expression)`,
    /// where `Expression` is the root of the constructed AST. On failure (due to syntax errors, invalid characters,
    /// unmatched parentheses, etc.), it returns an `Err(String)` with a descriptive error message.
    ///
    /// # Errors
    ///
    /// The function can return errors in several cases, including but not limited to:
    /// - Unmatched parentheses in the expression.
    /// - Use of invalid characters or operators.
    /// - Improperly formatted expressions (e.g., missing operands or operators).
    ///
    /// # Note
    ///
    /// The parser assumes that the input expression is a well-formed logical expression
    /// composed of uppercase alphabetic characters (A-Z) for variables, and the symbols
    /// '&', '|', 'v', '>', and '-' for logical operators. Spaces in the input are ignored.
    pub fn parse(&mut self) -> Result<Expression, ParserError> {
        let mut stack = Vec::new();

        while let Some(c) = self.chars.next() {
            match c {
                '(' => self.handle_parenthesis(&mut stack)?,
                'A'..='Z' => self.handle_variable(&mut stack, c)?,
                '-' => self.handle_negation(&mut stack)?,
                '&' | 'v' | '>' | '|' => self.handle_binary_operator(&mut stack, c)?,
                ' ' => (),
                _ => {
                    return Err(ParserError::InvalidExpression(c));
                }
            }
        }

        stack.pop().ok_or(ParserError::EmptyExpression)
    }

    /// Handles the opening parenthesis by parsing the content within brackets.
    ///
    /// # Arguments
    ///
    /// * `stack`: A mutable reference to a vector of `Expression` objects representing the current state of the parser.
    ///
    /// # Errors
    ///
    /// Returns a `ParserError` if the bracketed content is not a valid expression or if parentheses are unmatched.
    fn handle_parenthesis(&mut self, stack: &mut Vec<Expression>) -> Result<(), ParserError> {
        let bracket = self.extract_bracket_contents()?;
        stack.push(Parser::new(&bracket).parse()?);
        Ok(())
    }

    /// Handles a variable character by adding it to the parser stack as an `Expression::Var`.
    ///
    /// # Arguments
    ///
    /// * `stack`: Mutable reference to the parser stack.
    /// * `c`: The character representing the variable.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if successfully handled, otherwise a `ParserError`
    fn handle_variable(&mut self, stack: &mut Vec<Expression>, c: char) -> Result<(), ParserError> {
        stack.push(Expression::Var(c.to_string()));
        Ok(())
    }

    /// Handles negation in an expression.
    ///
    /// # Arguments
    ///
    /// * `stack`: Mutable reference to the parser stack.
    ///
    /// # Errors
    ///
    /// Returns `ParserError` if the subsequent expression after negation is invalid.
    fn handle_negation(&mut self, stack: &mut Vec<Expression>) -> Result<(), ParserError> {
        stack.push(self.parse_negation()?);
        Ok(())
    }

    /// Handles binary operators by constructing the appropriate binary expression.
    ///
    /// # Arguments
    ///
    /// * `stack`: Mutable reference to the parser stack.
    /// * `operator`: The character representing the binary operator.
    ///
    /// # Errors
    ///
    /// Returns `ParserError` for invalid binary expressions or missing operands.
    fn handle_binary_operator(
        &mut self,
        stack: &mut Vec<Expression>,
        operator: char,
    ) -> Result<(), ParserError> {
        let left = stack.pop().ok_or(ParserError::ExpectedLeftOperand)?;
        let operation_result = self.parse_binary_operation(operator, left)?;
        stack.push(operation_result);
        Ok(())
    }

    /// Parses a negation operation and returns the corresponding `Expression`.
    ///
    /// # Errors
    ///
    /// Returns a `ParserError` if the negation is not followed by a valid expression.
    fn parse_negation(&mut self) -> Result<Expression, ParserError> {
        let next = self
            .chars
            .next()
            .ok_or(ParserError::ExpectedExpressionAfterNegation)?;
        let right = if next == '(' {
            let bracket = self.extract_bracket_contents()?;
            Parser::new(&bracket).parse()?
        } else {
            Expression::Var(next.to_string())
        };

        Ok(Expression::Not(right.wrap()))
    }

    /// Parses a binary operation (AND, OR, IMPLIES) and returns the corresponding `Expression`.
    ///
    /// # Arguments
    ///
    /// * `operator`: Character representing the binary operator.
    /// * `left`: The left operand of the binary operation.
    ///
    /// # Errors
    ///
    /// Returns a `ParserError` for invalid operators or if the right operand is missing.
    fn parse_binary_operation(
        &mut self,
        operator: char,
        left: Expression,
    ) -> Result<Expression, ParserError> {
        self.consume_whitespace();
        let right = self.parse()?;

        let expr = match operator {
            '&' => Expression::And(left.wrap(), right.wrap()),
            'v' | '|' => Expression::Or(left.wrap(), right.wrap()),
            '>' => Expression::Implies(left.wrap(), right.wrap()),
            _ => return Err(ParserError::InvalidOperator(operator)),
        };
        Ok(expr)
    }

    /// Consumes and ignores any whitespace characters in the current parsing context.
    fn consume_whitespace(&mut self) {
        while let Some(&' ') = self.chars.peek() {
            self.chars.next();
        }
    }

    /// Extracts the contents within a pair of matching parentheses.
    ///
    /// # Errors
    ///
    /// Returns a `ParserError` if the parentheses are unmatched.
    fn extract_bracket_contents(&mut self) -> Result<String, ParserError> {
        let mut bracket = String::new();
        let mut bracket_count = 1;

        for c in self.chars.by_ref() {
            match c {
                '(' => bracket_count += 1,
                ')' => bracket_count -= 1,
                _ => bracket.push(c),
            }

            if bracket_count == 0 {
                return Ok(bracket);
            }
        }

        Err(ParserError::UnmatchedParentheses(bracket, bracket_count))
    }
}
