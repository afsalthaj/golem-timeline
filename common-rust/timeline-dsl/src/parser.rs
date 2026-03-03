use std::fmt;

use common_lib::{
    EventColumnName, EventColumnValue, GolemEventPredicate, GolemEventValue, TimeLineOp,
};

use crate::lexer::{Lexer, LexError, Spanned, Token};

// ── Public types ──

#[derive(Clone, Debug, PartialEq)]
pub enum AggregationFunction {
    Count,
    Sum,
    Avg,
    Min,
    Max,
}

#[derive(Clone, Debug)]
pub struct ParsedAggregation {
    pub group_by: String,
    pub functions: Vec<AggregationFunction>,
}

#[derive(Clone, Debug)]
pub struct ParsedTimeline {
    pub op: TimeLineOp,
    pub aggregation: Option<ParsedAggregation>,
}

// ── Errors ──

#[derive(Debug, Clone)]
pub struct ParseError {
    pub offset: usize,
    pub message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "parse error at offset {}: {}", self.offset, self.message)
    }
}

impl std::error::Error for ParseError {}

impl From<LexError> for ParseError {
    fn from(e: LexError) -> Self {
        ParseError {
            offset: e.offset,
            message: e.message,
        }
    }
}

// ── Parser ──

/// Parse a timeline DSL query into a `ParsedTimeline`.
///
/// Grammar (simplified):
/// ```text
/// query       = expr ("|" aggregate)?
/// expr        = or_expr
/// or_expr     = and_expr ("||" and_expr)*
/// and_expr    = unary ("&&" unary)*
/// unary       = "!" unary | postfix
/// postfix     = primary (cmp_op value)?
/// primary     = "(" expr ")"
///             | "latest_event_to_state" "(" string ")"
///             | "has_existed" "(" predicate ")"
///             | "has_existed_within" "(" predicate "," int ")"
///             | "duration_where" "(" expr ")"
///             | "duration_in_cur_state" "(" expr ")"
/// predicate   = ident pred_op value
/// pred_op     = "==" | ">" | "<"
/// cmp_op      = "==" | ">" | ">=" | "<" | "<="
/// value       = string | int | float | bool
/// aggregate   = "aggregate" "(" "group_by" "=" string "," agg_fn ("," agg_fn)* ")"
/// agg_fn      = "count" | "sum" | "avg" | "min" | "max"
/// ```
pub fn parse(input: &str) -> Result<ParsedTimeline, ParseError> {
    let tokens = Lexer::new(input).tokenize()?;
    let mut p = Parser::new(tokens);
    let result = p.parse_query()?;
    p.expect(Token::Eof)?;
    Ok(result)
}

struct Parser {
    tokens: Vec<Spanned>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Spanned>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn current(&self) -> &Spanned {
        &self.tokens[self.pos]
    }

    fn peek(&self) -> &Token {
        &self.current().token
    }

    fn offset(&self) -> usize {
        self.current().offset
    }

    fn advance(&mut self) -> &Token {
        let tok = &self.tokens[self.pos].token;
        if self.pos + 1 < self.tokens.len() {
            self.pos += 1;
        }
        tok
    }

    fn expect(&mut self, expected: Token) -> Result<(), ParseError> {
        if *self.peek() == expected {
            self.advance();
            Ok(())
        } else {
            Err(self.error(format!("expected '{}', found '{}'", expected, self.peek())))
        }
    }

    fn error(&self, message: String) -> ParseError {
        ParseError {
            offset: self.offset(),
            message,
        }
    }

    // ── Grammar rules ──

    fn parse_query(&mut self) -> Result<ParsedTimeline, ParseError> {
        let op = self.parse_expr()?;
        let aggregation = if *self.peek() == Token::Pipe {
            self.advance();
            Some(self.parse_aggregate()?)
        } else {
            None
        };
        Ok(ParsedTimeline { op, aggregation })
    }

    fn parse_expr(&mut self) -> Result<TimeLineOp, ParseError> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<TimeLineOp, ParseError> {
        let mut left = self.parse_and()?;
        while *self.peek() == Token::PipePipe {
            self.advance();
            let right = self.parse_and()?;
            left = TimeLineOp::Or(Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_and(&mut self) -> Result<TimeLineOp, ParseError> {
        let mut left = self.parse_unary()?;
        while *self.peek() == Token::AmpAmp {
            self.advance();
            let right = self.parse_unary()?;
            left = TimeLineOp::And(Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<TimeLineOp, ParseError> {
        if *self.peek() == Token::Bang {
            self.advance();
            let inner = self.parse_unary()?;
            return Ok(TimeLineOp::Not(Box::new(inner)));
        }
        self.parse_postfix()
    }

    fn parse_postfix(&mut self) -> Result<TimeLineOp, ParseError> {
        let expr = self.parse_primary()?;
        match self.peek() {
            Token::EqualEqual => {
                self.advance();
                let val = self.parse_value()?;
                Ok(TimeLineOp::EqualTo(Box::new(expr), val))
            }
            Token::Greater => {
                self.advance();
                let val = self.parse_value()?;
                Ok(TimeLineOp::GreaterThan(Box::new(expr), val))
            }
            Token::GreaterEqual => {
                self.advance();
                let val = self.parse_value()?;
                Ok(TimeLineOp::GreaterThanOrEqual(Box::new(expr), val))
            }
            Token::Less => {
                self.advance();
                let val = self.parse_value()?;
                Ok(TimeLineOp::LessThan(Box::new(expr), val))
            }
            Token::LessEqual => {
                self.advance();
                let val = self.parse_value()?;
                Ok(TimeLineOp::LessThanOrEqual(Box::new(expr), val))
            }
            _ => Ok(expr),
        }
    }

    fn parse_primary(&mut self) -> Result<TimeLineOp, ParseError> {
        match self.peek().clone() {
            Token::LParen => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(Token::RParen)?;
                Ok(expr)
            }
            Token::LatestEventToState => {
                self.advance();
                self.expect(Token::LParen)?;
                let col = self.parse_column_name()?;
                self.expect(Token::RParen)?;
                Ok(TimeLineOp::TlLatestEventToState(EventColumnName(col)))
            }
            Token::HasExisted => {
                self.advance();
                self.expect(Token::LParen)?;
                let pred = self.parse_predicate()?;
                self.expect(Token::RParen)?;
                Ok(TimeLineOp::TlHasExisted(pred))
            }
            Token::HasExistedWithin => {
                self.advance();
                self.expect(Token::LParen)?;
                let pred = self.parse_predicate()?;
                self.expect(Token::Comma)?;
                let duration = self.parse_int_lit()?;
                self.expect(Token::RParen)?;
                Ok(TimeLineOp::TlHasExistedWithin(pred, duration as u64))
            }
            Token::DurationWhere => {
                self.advance();
                self.expect(Token::LParen)?;
                let inner = self.parse_expr()?;
                self.expect(Token::RParen)?;
                Ok(TimeLineOp::TlDurationWhere(Box::new(inner)))
            }
            Token::DurationInCurState => {
                self.advance();
                self.expect(Token::LParen)?;
                let inner = self.parse_expr()?;
                self.expect(Token::RParen)?;
                Ok(TimeLineOp::TlDurationInCurState(Box::new(inner)))
            }
            _ => Err(self.error(format!("unexpected token '{}'", self.peek()))),
        }
    }

    fn parse_predicate(&mut self) -> Result<GolemEventPredicate<GolemEventValue>, ParseError> {
        let col_name = self.parse_ident()?;
        let col = EventColumnName(col_name);

        match self.peek().clone() {
            Token::EqualEqual => {
                self.advance();
                let val = EventColumnValue(self.parse_value()?);
                Ok(GolemEventPredicate::Equals(col, val))
            }
            Token::Greater => {
                self.advance();
                let val = EventColumnValue(self.parse_value()?);
                Ok(GolemEventPredicate::GreaterThan(col, val))
            }
            Token::Less => {
                self.advance();
                let val = EventColumnValue(self.parse_value()?);
                Ok(GolemEventPredicate::LessThan(col, val))
            }
            _ => Err(self.error(format!(
                "expected predicate operator ('==', '>', '<'), found '{}'",
                self.peek()
            ))),
        }
    }

    fn parse_value(&mut self) -> Result<GolemEventValue, ParseError> {
        match self.peek().clone() {
            Token::StringLit(s) => {
                self.advance();
                Ok(GolemEventValue::StringValue(s))
            }
            Token::IntLit(n) => {
                self.advance();
                Ok(GolemEventValue::IntValue(n))
            }
            Token::FloatLit(f) => {
                self.advance();
                Ok(GolemEventValue::FloatValue(f))
            }
            Token::True => {
                self.advance();
                Ok(GolemEventValue::BoolValue(true))
            }
            Token::False => {
                self.advance();
                Ok(GolemEventValue::BoolValue(false))
            }
            _ => Err(self.error(format!("expected value, found '{}'", self.peek()))),
        }
    }

    fn parse_column_name(&mut self) -> Result<String, ParseError> {
        match self.peek().clone() {
            Token::Ident(s) => { self.advance(); Ok(s) }
            Token::StringLit(s) => { self.advance(); Ok(s) }
            _ => Err(self.error(format!("expected column name, found '{}'", self.peek()))),
        }
    }

    fn parse_int_lit(&mut self) -> Result<i64, ParseError> {
        match self.peek().clone() {
            Token::IntLit(n) => {
                self.advance();
                Ok(n)
            }
            _ => Err(self.error(format!("expected integer, found '{}'", self.peek()))),
        }
    }

    fn parse_ident(&mut self) -> Result<String, ParseError> {
        match self.peek().clone() {
            Token::Ident(s) => {
                self.advance();
                Ok(s)
            }
            _ => Err(self.error(format!("expected identifier, found '{}'", self.peek()))),
        }
    }

    fn parse_aggregate(&mut self) -> Result<ParsedAggregation, ParseError> {
        self.expect(Token::Aggregate)?;
        self.expect(Token::LParen)?;
        self.expect(Token::GroupBy)?;
        self.expect(Token::Eq)?;
        let group_by = self.parse_column_name()?;
        self.expect(Token::Comma)?;

        let mut functions = vec![self.parse_agg_fn()?];
        while *self.peek() == Token::Comma {
            self.advance();
            functions.push(self.parse_agg_fn()?);
        }

        self.expect(Token::RParen)?;
        Ok(ParsedAggregation {
            group_by,
            functions,
        })
    }

    fn parse_agg_fn(&mut self) -> Result<AggregationFunction, ParseError> {
        match self.peek().clone() {
            Token::Count => { self.advance(); Ok(AggregationFunction::Count) }
            Token::Sum => { self.advance(); Ok(AggregationFunction::Sum) }
            Token::Avg => { self.advance(); Ok(AggregationFunction::Avg) }
            Token::Min => { self.advance(); Ok(AggregationFunction::Min) }
            Token::Max => { self.advance(); Ok(AggregationFunction::Max) }
            _ => Err(self.error(format!(
                "expected aggregation function (count, sum, avg, min, max), found '{}'",
                self.peek()
            ))),
        }
    }
}
