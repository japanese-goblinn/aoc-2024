use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;

fn main() -> Result<(), Box<dyn Error>> {
  println!("{}", first()?);
  println!("{}", second()?);
  Ok(())
}

fn second() -> Result<i32, Box<dyn Error>> {
  let mut first_column_numbers = Vec::new();
  let mut second_column_numbers = HashMap::new();
  let file = File::open("day1.txt")?;
  let lines = BufReader::new(file).lines();
  for line in lines {
    if let Ok(paresed_line) = line { 
      let parts = paresed_line.split_whitespace().collect::<Vec<_>>();
      let first_num: i32 = parts[0].parse().unwrap();
      let second_num: i32 = parts[1].parse().unwrap();
      first_column_numbers.push(first_num);
      *second_column_numbers.entry(second_num).or_insert(0) += 1;
    }
  }
  let mut result = 0;
  for v in &first_column_numbers {
    if let Some(s) = second_column_numbers.get(v) {
      result += v * s;
    }
  }
  Ok(result)
}

fn first() -> Result<i32, Box<dyn Error>> { 
  let mut first_column_numbers = Vec::new();
  let mut second_column_numbers = Vec::new();
  let file = File::open("day1.txt")?;
  let lines = BufReader::new(file).lines();
  for line in lines {
    if let Ok(paresed_line) = line { 
       let parts = paresed_line.split_whitespace().collect::<Vec<_>>();
       let first_num: i32 = parts[0].parse().unwrap();
       let second_num: i32 = parts[1].parse().unwrap();
       first_column_numbers.push(first_num);
       second_column_numbers.push(second_num);
    }
  }
  first_column_numbers.sort();
  second_column_numbers.sort();

  Ok(zip(first_column_numbers, second_column_numbers).fold(
    0, 
    |acc, x,| acc + (x.0 - x.1).abs()
  ))
}