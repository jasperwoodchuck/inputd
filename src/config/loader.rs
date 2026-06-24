use std::path::PathBuf;
use std::{env, fs, io};

pub fn config_dir() -> Result<PathBuf, io::Error> {
	let config_dir = env::var("XDG_CONFIG_HOME")
		.map(PathBuf::from)
		.or_else(|_| env::var("HOME").map(|home| PathBuf::from(home).join(".config")));

	match config_dir {
		Ok(path) => Ok(path.join("inputd")),
		Err(err) => Err(io::Error::new(
			io::ErrorKind::NotFound,
			format!("config dir not found: {err}"),
		)),
	}
}

pub fn config_path() -> Result<PathBuf, io::Error> {
	Ok(config_dir()?.join("config.binds"))
}
pub fn load_config() -> Result<String, io::Error> {
	fs::read_to_string(config_path()?)
}
