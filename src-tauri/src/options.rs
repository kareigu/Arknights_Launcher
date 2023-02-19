use std::fmt::Display;

use ron::{error::SpannedError, ser::PrettyConfig};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Background {
  Default(BackgroundComponents),
  Custom(BackgroundComponents),
}

impl Default for Background {
  fn default() -> Self {
    Self::Default(BackgroundComponents {
      background: "ptilopsis_epoque.webp".to_string(),
      character: "celebration.webp".to_string(),
    })
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BackgroundComponents {
  pub background: String,
  pub character: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Options {
  pub executable_path: String,
  pub background: Background,
}

impl Options {
  pub fn load_from_file<S>(path: S) -> Result<Self, LoadError>
  where
    S: ToString,
  {
    let file = std::fs::File::open(path.to_string()).map_err(|_| LoadError::NoFile)?;
    let file = std::io::read_to_string(file).map_err(|e| LoadError::ReadingFile(e))?;

    ron::from_str(file.as_str()).map_err(|e| LoadError::Deserialize(e))
  }

  pub fn save_to_file<S>(&self, path: S) -> Result<(), SaveError>
  where
    S: ToString,
  {
    let contents = ron::ser::to_string_pretty(self, PrettyConfig::default())
      .map_err(|e| SaveError::Serialize(e))?;

    std::fs::write(path.to_string(), contents).map_err(|e| SaveError::WriteFile(e))
  }
}

impl Default for Options {
  fn default() -> Self {
    Self {
      executable_path: String::default(),
      background: Background::default(),
    }
  }
}

pub enum LoadError {
  NoFile,
  ReadingFile(std::io::Error),
  Deserialize(SpannedError),
}

impl Display for LoadError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::NoFile => f.write_str("No file found"),
      Self::ReadingFile(e) => f.write_fmt(format_args!("Error reading file: {}", e)),
      Self::Deserialize(e) => f.write_fmt(format_args!("Error deserializing file: {}", e)),
    }
  }
}

pub enum SaveError {
  Serialize(ron::Error),
  WriteFile(std::io::Error),
}

impl Display for SaveError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Serialize(e) => f.write_fmt(format_args!("Error serializing file: {}", e)),
      Self::WriteFile(e) => f.write_fmt(format_args!("Error writing file: {}", e)),
    }
  }
}
