use crate::input::types::Hotkey;

#[derive(Debug)]
pub struct Program {
	pub settings: Settings,
	pub bindings: Vec<Binding>,
}

#[derive(Debug)]
pub struct Binding {
	pub hotkey: Hotkey,
	pub action: Action,
}

#[derive(Debug)]
pub enum Action {
	Emit(Hotkey),
	Disable,
}

#[derive(Debug)]
pub struct Settings {
	pub keyboard: String,
	pub mousedev: String,
}
