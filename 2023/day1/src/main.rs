use std::io::{ErrorKind, Result};
use std::num::NonZeroU8;

fn main() -> Result<()> {
    use std::env::args_os;
    use std::fs::read;

    let input = args_os().nth(1).unwrap_or_default(); // 0th is a garbage/undefined value;
                                                      // everything after is a command line given arg.

    let result = process_data(&mut read(input)?)?;

    println!("{result}");

    Ok(())
}

fn process_data<T: AsMut<[u8]>>(mut rawdata: T) -> Result<usize> {
    use std::io::Error;

    let rawdata = rawdata.as_mut();

    let calibration_file = if rawdata.is_ascii() {
        rawdata.make_ascii_lowercase();
        rawdata
    } else {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "File was not valid ASCII",
        ));
    };

    let calibration_lines = calibration_file.split(|x| *x == b'\n');

    let mut calibration_sum: usize = 0;

    for line in calibration_lines {
        // line needs to be valid ASCII to get here
        calibration_sum += calculate_calibration_value(line) as usize;
    }
    Ok(calibration_sum)
}

/// # Usage
/// Assumes input is lowercase ASCII
fn calculate_calibration_value(line: &[u8]) -> u8 {
    fn find_first_and_last(line: &[u8]) -> (u8, u8) {
        let first;
        let last;

        let mut begin = 0;
        let len = line.len();
        loop {
            if begin == len {
                first = 0;
                last = first;
                return (first, last);
            } else if let Some(val) = match_ascii(&line[begin..len]) {
                first = val.into();
                break;
            };
            begin += 1;
        }
        begin += 1; // like this we won't match the same num twice; this saves us some performance as the end branch sets `last = first` anyway.
        let mut i = len;
        loop {
            i -= 1;
            if let Some(val) = match_ascii(&line[i..len]) {
                last = val.into();
                break;
            } else if i == begin {
                last = first;
                return (first, last);
            }
        }
        (first, last)
    }
    unsafe {
        use std::str;
        let print = str::from_utf8_unchecked(line);
        print!("{print} ");
    }

    let (first, last) = find_first_and_last(line);

    print!("{first}{last} ");

    let result = first * 10 + last;
    println!("{result}");
    result
}
/// Takes an ASCII str and return appropriate u8 value
/// # Usage
/// Assumes input is lowercase ASCII
fn match_ascii<T: AsRef<[u8]>>(str: T) -> Option<NonZeroU8> {
    const ASCII_MASK: u8 = 0xF;
    const NUM_WORDS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    const MIN_WORD_LEN: usize = 3;

    let str = str.as_ref();

    if let char @ b'1'..=b'9' = str[0] {
        // we do not match b'0', so it can never be 0
        return Some(unsafe { NonZeroU8::new_unchecked(char & ASCII_MASK) });
    }
    if str.len() < MIN_WORD_LEN {
        return None;
    }
    for word in NUM_WORDS {
        if word.len() > str.len() {
            continue;
        }
        let str = &str[..word.len()];
        unsafe {
            return Some(NonZeroU8::new_unchecked(match str {
                b"one" => 1,
                b"two" => 2,
                b"three" => 3,
                b"four" => 4,
                b"five" => 5,
                b"six" => 6,
                b"seven" => 7,
                b"eight" => 8,
                b"nine" => 9,
                _ => continue,
            }));
        }
    }
    None
}

#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    fn part1() {
        let rawdata = include_bytes!("../examples/part1.txt").to_owned();

        let result = process_data(rawdata).unwrap_or_default();

        assert_eq!(result, 142);
    }

    #[test]
    fn part2() {
        let rawdata = include_bytes!("../examples/part2.txt").to_owned();

        let result = process_data(rawdata).unwrap_or_default();

        assert_eq!(result, 281);
    }
}
