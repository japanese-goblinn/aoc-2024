use core::num;
use std::{error::Error, fs::File, io::{BufRead, BufReader}, iter::zip, path::Path};

pub(crate) type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub(crate) fn lines_of_file<P: AsRef<Path>>(path: P) -> Result<impl Iterator<Item = String>> {
  let file = File::open(path)?;
  Ok(BufReader::new(file).lines().map(|l| l.unwrap()))
}

fn main() -> Result<()> {
  let f = vec![1, 2, 3, 4];

  // println!("{}", first()?);
  // println!("{}", second()?);

  // println!("{}", day1::first()?);
  // println!("{}", day1::second()?);
  Ok(())
}

#[derive(PartialEq)]
enum Growth {
  Increasing,
  Decreasing
}

fn second() -> Result<i32> {
  let mut safe_count = 0;

  for line in lines_of_file("day2.txt")? {
    let nums = line.split_whitespace()
      .filter_map(|num| num.parse::<i32>().ok() );
    let mut is_safe = false;
    let mut growth: Option<Growth> = None;
    for (f, s) in zip(nums.clone(), nums.skip(1)) {
      let diff = f - s;
      let distance = diff.abs();
      is_safe = distance >= 1 && distance <= 3;
      if !is_safe {
        break
      }
      let current_growth = if diff < 0 {
        Growth::Decreasing
      } else {
        Growth::Increasing
      };
      if let Some(_growth) = &growth {
        if *_growth != current_growth {
          break;
        }
      } else {
        growth = Some(current_growth)
      }
    }
    if is_safe && growth != None {
      safe_count += 1;
    }
  }

  Ok(safe_count)
}

fn first() -> Result<i32> {
  let mut safe_count = 0;
  for line in lines_of_file("day2.txt")? {
    let nums = line.split_whitespace()
      .filter_map(|num| num.parse::<i32>().ok() );
    let mut is_safe = false;
    let mut growth: Option<Growth> = None;
    for (f, s) in zip(nums.clone(), nums.skip(1)) {
      let diff = f - s;
      let distance = diff.abs();
      is_safe = distance >= 1 && distance <= 3;
      if !is_safe {
        break
      }
      let current_growth = if diff < 0 {
        Growth::Decreasing
      } else {
        Growth::Increasing
      };
      if let Some(_growth) = &growth {
        if *_growth != current_growth {
          growth = None;
          break;
        }
      } else {
        growth = Some(current_growth)
      }
    }
    if is_safe && growth != None {
      safe_count += 1;
    }
  }

  

  Ok(safe_count)
}