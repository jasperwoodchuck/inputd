use crate::config::loader::load_config;
use crate::syntax::lexer::tokenize;
use crate::syntax::types::Parser;

mod config;
mod input;
mod syntax;

mod utils;

fn main() {
	tracing_subscriber::fmt().without_time().init();

	let source = try_or!(return load_config());

	let spanned_tokens = try_or!(return tokenize(&source));

	let mut parser = Parser::new(spanned_tokens, &source);

	let program = try_or!(return parser.parse());

	println!("{:#?}", program.bindings);
}
