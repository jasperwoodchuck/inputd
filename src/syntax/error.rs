use std::fmt;

use logos::Span;

#[derive(Debug)]
pub struct SyntaxError {
	pub message: String,
	pub span: Option<Span>,
	pub source: String,
}

impl SyntaxError {
	pub fn new(message: impl Into<String>, span: Option<Span>, source: impl Into<String>) -> Self {
		Self {
			message: message.into(),
			span,
			source: source.into(),
		}
	}
}

fn line_col(source: &str, pos: usize) -> (usize, usize) {
	let mut row = 1;
	let mut col = 1;

	for ch in source[..pos].chars() {
		if ch == '\n' {
			row += 1;
			col = 1;
		} else {
			col += 1;
		}
	}

	(row, col)
}

impl SyntaxError {
	fn fmt_unlocated(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.message)
	}

	fn fmt_located(&self, f: &mut fmt::Formatter<'_>, span: &Span) -> fmt::Result {
		let (row, col) = line_col(&self.source, span.start);
		let line = self.source.lines().nth(row.saturating_sub(1)).unwrap_or("");
		let caret_count = (span.end - span.start).max(1);
		let gutter = row.to_string().len();

		let marker = if caret_count == 1 {
			"^".to_string()
		} else {
			"~".repeat(caret_count)
		};

		writeln!(f, "syntax error")?;
		writeln!(f, "{:>width$}  ┌─ {}:{}", "", row, col, width = gutter - 1,)?;
		writeln!(f, "{:>gutter$} │", "")?;
		writeln!(f, "{:>gutter$} │ {}", row, line)?;

		writeln!(
			f,
			"{:>gutter$} │ {}{} {}",
			"",
			" ".repeat(col - 1),
			marker,
			self.message,
		)?;

		write!(f, "{}┘", "─".repeat(gutter + 1))
	}
}

impl fmt::Display for SyntaxError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match &self.span {
			Some(span) => self.fmt_located(f, span),
			None => self.fmt_unlocated(f),
		}
	}
}
