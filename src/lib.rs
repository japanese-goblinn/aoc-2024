use std::{error::Error, fs::File, io::{BufRead, BufReader}, path::Path};

pub(crate) type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub(crate) fn lines_of_file<P: AsRef<Path>>(path: P) -> Result<impl Iterator<Item = String>> {
  let file = File::open(path)?;
  Ok(BufReader::new(file).lines().map(|l| l.unwrap()))
}