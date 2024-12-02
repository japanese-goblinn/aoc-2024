use std::error::Error;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
  println!("{}", first()?);
  println!("{}", second()?);
  Ok(())
}

fn lines_of_file<P: AsRef<Path>>(path: P) -> Result<impl Iterator<Item = String>> {
  let file = File::open(path)?;
  Ok(BufReader::new(file).lines().map(|l| l.unwrap()))
}

fn second() -> Result<i32> {
  let mut first_column_numbers = Vec::<i32>::new();
  let mut second_column_numbers = HashMap::<i32, i32>::new();

  for line in lines_of_file("day1.txt")? {
      let mut iter = line.split_whitespace();
      first_column_numbers
        .push(iter.next().unwrap().parse().unwrap());
      *second_column_numbers
        .entry(iter.next().unwrap().parse().unwrap())
        .or_insert(0) += 1;
  }

  Ok(first_column_numbers.into_iter()
    .filter_map(|num| 
      second_column_numbers.get(&num).map(|score| num * score)
    )
    .sum())
}

fn first() -> Result<i32> { 
  let mut first_column_numbers = vec![];
  let mut second_column_numbers = vec![];

  for line in lines_of_file("day1.txt")? {
    let mut iter = line.split_whitespace();
    first_column_numbers.push(iter.next().unwrap().parse::<i32>().unwrap());
    second_column_numbers.push(iter.next().unwrap().parse::<i32>().unwrap());

  }
  
  first_column_numbers.sort();
  second_column_numbers.sort();

  Ok(zip(first_column_numbers, second_column_numbers).fold(
    0, 
    |acc, pair,| acc + (pair.0 - pair.1).abs()
  ))
}