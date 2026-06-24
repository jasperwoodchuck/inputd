use crate::input::mapping::str_to_input_token;
use crate::input::types::Hotkey;
use crate::syntax::ast::{Action, Binding, Program, Settings};
use crate::syntax::error::SyntaxError;
use crate::syntax::lexer::Token;
use crate::syntax::types::Parser;

impl Parser {
	pub fn parse(&mut self) -> Result<Program, SyntaxError> {
		let mut bindings = Vec::<Binding>::new();

		self.skip_eols();

		let settings = self.parse_settings()?;

		loop {
			match self.current_token() {
				Some(Token::Eol) => self.skip_eols(),
				Some(_) => bindings.push(self.parse_binding()?),
				None => break,
			}
		}

		Ok(Program { settings, bindings })
	}

	pub(crate) fn parse_settings(&mut self) -> Result<Settings, SyntaxError> {
		let mut keyboard: Option<String> = None;
		let mut mousedev: Option<String> = None;

		self.skip_eols();

		while let Some(token) = self.current_token() {
			match token {
				Token::Ident(ident) if ident == "keyboard" => {
					if keyboard.is_some() {
						return self.err("duplicate keyboard decleration");
					}

					keyboard = Some(self.parse_variable()?)
				}
				Token::Ident(ident) if ident == "mousedev" => {
					if mousedev.is_some() {
						return self.err("duplicate mousedev decleration");
					}

					mousedev = Some(self.parse_variable()?)
				}
				Token::Eol => self.skip_eols(),
				_ => break,
			}
		}

		let keyboard =
			keyboard.ok_or_else(|| self.error_unlocated("missing keyboard declaration"))?;

		let mousedev =
			mousedev.ok_or_else(|| self.error_unlocated("missing mousedev declaration"))?;

		Ok(Settings { keyboard, mousedev })
	}

	pub(crate) fn parse_binding(&mut self) -> Result<Binding, SyntaxError> {
		let hotkey = self.parse_hotkey()?;

		self.expect(Token::Rebind)?;

		let action = self.parse_action()?;

		self.expect(Token::Eol)?;

		Ok(Binding { hotkey, action })
	}

	pub(crate) fn parse_hotkey(&mut self) -> Result<Hotkey, SyntaxError> {
		let mut hotkey = Hotkey::new();

		loop {
			let key_token = match self.current_token() {
				Some(Token::Ident(ident)) => ident.clone(),
				Some(Token::Integer(10..)) => {
					return self.err("expected input");
				}
				Some(Token::Integer(int)) => int.to_string(),
				_ => return self.err("expected input"),
			};

			let input_token = str_to_input_token(&key_token).map_err(|msg| self.error(msg))?;

			hotkey.push(input_token);

			self.advance();

			if !matches!(self.current_token(), Some(Token::Plus)) {
				break;
			}

			self.advance();
		}

		Ok(hotkey)
	}

	pub(crate) fn parse_action(&mut self) -> Result<Action, SyntaxError> {
		let action = match self.current_token() {
			Some(Token::Ident(_)) | Some(Token::Integer(..10)) => {
				Action::Emit(self.parse_hotkey()?)
			}
			_ => return self.err("expected action"),
		};

		Ok(action)
	}
}
