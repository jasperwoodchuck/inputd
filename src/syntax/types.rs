use logos::Span;

use crate::syntax::lexer::Token;

#[derive(Debug)]
pub struct Spanned<T> {
	pub item: T,
	pub span: Span,
}

pub type SpannedToken = Spanned<Token>;

pub struct Parser {
	pub(crate) tokens: Vec<SpannedToken>,
	pub(crate) pos: usize,
	pub(crate) source: String,
}
