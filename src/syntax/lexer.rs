use std::fmt;

use logos::{Lexer, Logos};

use crate::syntax::error::SyntaxError;
use crate::syntax::types::SpannedToken;

fn unquote_string(lex: &mut Lexer<Token>) -> String {
	let slice = lex.slice();
	slice[1..slice.len() - 1].to_owned()
}

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\f]+")]
pub enum Token {
	#[regex(r"#[^\n]*", logos::skip, allow_greedy = true)]
	#[regex(r"\n")]
	Eol,

	#[token("::")]
	Rebind,

	#[token("+")]
	Plus,

	#[token("=")]
	Equal,

	#[token("Disable")]
	Disable,

	#[regex(r"[A-Za-z][A-Za-z0-9_]*", |lex| lex.slice().to_owned())]
	Ident(String),

	#[regex(r"[0-9]+", |lex| lex.slice().parse().ok())]
	Integer(u64),

	#[regex(r#""([^"\\\r\n]|\\.)*""#, unquote_string)]
	Literal(String),

	#[regex(r#""[^"\r\n]*"#)]
	UnterminatedLiteral,
}

pub fn tokenize(src: &str) -> Result<Vec<SpannedToken>, SyntaxError> {
	let mut lexer = Token::lexer(src);

	let mut spanned_tokens = Vec::<SpannedToken>::new();

	while let Some(result) = lexer.next() {
		let span = lexer.span();

		if let Ok(token) = result {
			spanned_tokens.push(SpannedToken { item: token, span });
		} else {
			return Err(SyntaxError::new("invalid token", Some(span), src));
		}
	}

	Ok(spanned_tokens)
}

impl fmt::Display for Token {
	#[rustfmt::skip]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Eol     => write!(f, "newline"),

            Token::Rebind  => write!(f, "::"),
            Token::Plus    => write!(f, "+"),
            Token::Equal   => write!(f, "="),

            Token::Disable => write!(f, "Disable"),

            Token::Literal(s) | Token::Ident(s) => write!(f, "{s}"),
            Token::Integer(n) => write!(f, "{n}"),

            _ => unreachable!()
        }
    }
}
