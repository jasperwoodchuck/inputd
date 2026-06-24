use crate::syntax::error::SyntaxError;
use crate::syntax::lexer::Token;
use crate::syntax::types::{Parser, SpannedToken};

impl Parser {
	pub(crate) fn new(tokens: Vec<SpannedToken>, src: impl Into<String>) -> Self {
		Self {
			tokens,
			pos: 0,
			source: src.into(),
		}
	}

	pub(crate) fn current(&self) -> Option<&SpannedToken> {
		self.tokens.get(self.pos)
	}

	pub(crate) fn advance(&mut self) {
		self.pos += 1;
	}

	pub(crate) fn current_token(&self) -> Option<&Token> {
		self.current().map(|spanned_token| &spanned_token.item)
	}

	pub(crate) fn skip_eols(&mut self) {
		while matches!(self.current_token(), Some(&Token::Eol)) {
			self.advance();
		}
	}

	pub(crate) fn parse_variable(&mut self) -> Result<String, SyntaxError> {
		self.advance();

		self.expect(Token::Equal)?;

		let value = match self.current_token() {
			Some(Token::Literal(value)) => {
				let value = value.clone();
				self.advance();
				value
			}

			Some(Token::UnterminatedLiteral) => {
				return self.err("unterminated string literal");
			}

			_ => return self.err("expected string value"),
		};

		self.expect(Token::Eol)?;

		Ok(value)
	}

	pub(crate) fn expect(&mut self, expected: Token) -> Result<(), SyntaxError> {
		match self.current_token() {
			Some(token) if *token == expected => {
				self.advance();
				Ok(())
			}

			_ => self.err(format!("expected {expected}")),
		}
	}

	pub(crate) fn error(&self, msg: impl Into<String>) -> SyntaxError {
		let span = self
			.current()
			.map(|token| token.span.clone())
			.or(Some(self.source.len()..self.source.len()));

		SyntaxError {
			message: msg.into(),
			span,
			source: self.source.clone(),
		}
	}

	pub(crate) fn err<T>(&self, msg: impl Into<String>) -> Result<T, SyntaxError> {
		let span = self
			.current()
			.map(|token| token.span.clone())
			.or(Some(self.source.len()..self.source.len()));

		Err(SyntaxError {
			message: msg.into(),
			span,
			source: self.source.clone(),
		})
	}

	pub(crate) fn error_unlocated(&self, msg: impl Into<String>) -> SyntaxError {
		SyntaxError {
			message: msg.into(),
			span: None,
			source: self.source.clone(),
		}
	}
}
