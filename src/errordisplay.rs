/* 
 * CatSQL - the low cost SQL Terminal!
 * Copyright (C) 2026, Float314. 
 * 
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 3 of the License, or (at your option) any later version.

 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 * 
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program; if not, write to the Free Software Foundation,
 * Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.
*/

/* 
 * Pretty-print errors coming from sqlparser-rs.
 *
 * sqlparser only exposes the location of an error as text inside its message
 * (for example "Expected: an expression, found: FROM at Line: 1, Column: 15").
 * We split that suffix back out and turn it into a compiler-style diagnostic:
 *
 *   Error: Expected: an expression, found: FROM
 *     ╭─[<query>:1:15]
 *     │
 *   1 │ SELECT * FROM FROM users;
 *     │                 ──┬─
 *     │                   ╰── error starts here
 *   ────╯
 *
 * The goal is to show where the problem is, not to suggest a fix.
 */

use std::ops::Range;

use ariadne::{Color, Label, Report, ReportKind, Source};
use colorize::AnsiColor;
use sqlparser::parser::ParserError;

/// Prefix added by `ParserError`'s `Display` implementation.
const PARSER_PREFIX: &str = "sql parser error: ";
/// Suffix sqlparser appends to messages that carry a source location.
const LOCATION_MARKER: &str = " at Line: ";

/// Print a sqlparser error as a source-annotated diagnostic.
///
/// `query` must be the exact SQL string that was handed to sqlparser, so the
/// line/column in the error message points to the right place.
pub fn catch_err_print(query: &str, error: &ParserError) {
    let rendered = error.to_string();
    let raw_message = rendered.strip_prefix(PARSER_PREFIX).unwrap_or(&rendered);
    let (message, location) = split_location(raw_message);

    let offset = match location
        .and_then(|(line, column)| line_col_to_offset(query, line, column))
        // EOF errors often have no location attached by sqlparser. In that
        // case the problem is at the end of the input.
        .or_else(|| message.contains("found: EOF").then(|| query.chars().count()))
    {
        Some(offset) => offset,
        None => {
            // No usable source location (e.g. recursion limit exceeded).
            eprintln!("{} {}", "Error!".bold().red(), message);
            return;
        }
    };

    let span = error_span(query, offset, message);
    let source_id = "<query>";
    let diagnostic_span = (source_id, span);

    let report = Report::build(ReportKind::Error, diagnostic_span.clone())
        .with_message(message)
        .with_label(
            Label::new(diagnostic_span)
                .with_message("error starts here")
                .with_color(Color::Red),
        )
        .with_note("while parsing this SQL statement")
        .finish();

    if let Err(render_error) = report.eprint((source_id, Source::from(query))) {
        // Rendering should never be the reason the user loses the error.
        eprintln!("{} {}", "Error!".bold().red(), message);
        eprintln!("(could not render diagnostic: {render_error})");
    }
}

/// Split sqlparser's `"... at Line: L, Column: C"` suffix from the message.
fn split_location(message: &str) -> (&str, Option<(u64, u64)>) {
    let Some(marker_index) = message.rfind(LOCATION_MARKER) else {
        return (message, None);
    };

    let location = &message[marker_index + LOCATION_MARKER.len()..];
    let Some((line, column)) = location.split_once(", Column: ") else {
        return (message, None);
    };

    match (line.trim().parse::<u64>(), column.trim().parse::<u64>()) {
        (Ok(line), Ok(column)) => (&message[..marker_index], Some((line, column))),
        _ => (message, None),
    }
}

/// Convert a 1-based line/column pair into a 0-based character offset.
///
/// Ariadne uses character offsets by default, which also matches how
/// sqlparser counts columns.
fn line_col_to_offset(source: &str, line: u64, column: u64) -> Option<usize> {
    if line == 0 || column == 0 {
        return None;
    }

    let mut current_line = 1u64;
    let mut line_start = 0usize;

    for (index, character) in source.chars().enumerate() {
        if current_line == line {
            break;
        }
        if character == '\n' {
            current_line += 1;
            line_start = index + 1;
        }
    }

    if current_line != line {
        return None;
    }

    let line_length = source
        .chars()
        .skip(line_start)
        .take_while(|character| *character != '\n')
        .count();

    // Clamp to the end of the line so EOF locations still point somewhere.
    Some(line_start + (column as usize - 1).min(line_length))
}

/// Find the span of the token named in the error, falling back to one character.
fn error_span(source: &str, offset: usize, message: &str) -> Range<usize> {
    let characters: Vec<char> = source.chars().collect();

    if offset >= characters.len() {
        return characters.len()..characters.len();
    }

    if let Some(found) = found_token(message) {
        let found: Vec<char> = found.chars().collect();
        if !found.is_empty() && characters[offset..].starts_with(&found) {
            return offset..offset + found.len();
        }
    }

    offset..(offset + 1).min(characters.len())
}

/// Extract the token from sqlparser's `"Expected: ..., found: <token>"` text.
fn found_token(message: &str) -> Option<&str> {
    let found = message.split(", found: ").nth(1)?.trim();
    (!found.is_empty()).then_some(found)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_location_from_message() {
        let (message, location) = split_location("Expected: X, found: Y at Line: 2, Column: 7");

        assert_eq!(message, "Expected: X, found: Y");
        assert_eq!(location, Some((2, 7)));
    }

    #[test]
    fn keeps_messages_without_locations() {
        let (message, location) = split_location("recursion limit exceeded");

        assert_eq!(message, "recursion limit exceeded");
        assert_eq!(location, None);
    }

    #[test]
    fn maps_line_and_column_to_character_offset() {
        let source = "SELECT 1;\nSELECT * FROM;";
        let second_line = source.chars().position(|c| c == '\n').unwrap() + 1;

        assert_eq!(line_col_to_offset(source, 2, 1), Some(second_line));
        assert_eq!(line_col_to_offset(source, 2, 8), Some(second_line + 7));
        assert_eq!(line_col_to_offset(source, 1, 1), Some(0));
    }

    #[test]
    fn clamps_columns_past_the_end_of_a_line() {
        let source = "SELECT * FROM";
        assert_eq!(line_col_to_offset(source, 1, 100), Some(source.chars().count()));
    }

    #[test]
    fn highlights_the_found_token() {
        let source = "SELECT * FROM FROM users";
        let offset = source.chars().position(|c| c == 'F').unwrap();
        // The first "FROM" is the found token in this example.
        let span = error_span(source, offset, "Expected: an expression, found: FROM");

        assert_eq!(span, offset..offset + 4);
    }

    #[test]
    fn falls_back_to_a_single_character() {
        let source = "SELECT 'unterminated";
        let offset = source.chars().position(|c| c == '\'').unwrap();

        assert_eq!(error_span(source, offset, "Unterminated string literal"), offset..offset + 1);
    }
}
