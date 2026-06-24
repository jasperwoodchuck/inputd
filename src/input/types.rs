use evdev::KeyCode;

#[derive(Debug, Clone, Copy)]
pub enum Axis {
	X,
	Y,
}

#[derive(Debug, Clone, Copy)]
pub enum Magnitude {
	Positive,
	Negative,
}

#[derive(Debug, Clone, Copy)]
pub enum Input {
	Key(KeyCode),
	Btn(KeyCode),
	MouseMotion(Axis, Magnitude),
	MouseScroll(Axis, Magnitude),
}

pub type Hotkey = Vec<Input>;
