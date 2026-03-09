// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0
mod lexer;
mod parser;

pub use parser::{parse, ParsedTimeline, ParsedAggregation, AggregationFunction, ParseError};

#[cfg(test)]
mod tests;