use regex::bytes::Regex;
use std::io::{ErrorKind, Result};

fn main() -> Result<()> {
    use std::env::args_os;
    use std::fs::read;

    let input = args_os().nth(1).unwrap_or_default(); // 0th is a garbage/undefined value

    let result = process_data(read(input)?)?;

    println!("{result}");

    Ok(())
}

fn process_data(mut rawdata: Vec<u8>) -> Result<usize> {
    use std::io::Error;

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
    use std::str;
    let regex = unsafe {
        // This should be valid regex
        Regex::new(r"[1-9]|one|two|three|four|five|six|seven|eight|nine").unwrap_unchecked()
    };

    let mut iter = regex.find_iter(line);

    unsafe {
        print!("{} ", str::from_utf8_unchecked(line));
    }

    let first = if let Some(matched) = iter.next() {
        unsafe {
            print!("{}", str::from_utf8_unchecked(matched.as_bytes()));
        }
        ascii_to_value(matched.as_bytes())
    } else {
        0
    };

    let last = if let Some(matched) = iter.last() {
        unsafe {
            print!("{}", std::str::from_utf8_unchecked(matched.as_bytes()));
        }
        ascii_to_value(matched.as_bytes())
    } else {
        first
    };

    print!(" {first}{last} ");

    let result = first * 10 + last;
    println!("{result}");
    result
}

/// # Usage
/// Assumes input is lowercase ASCII
fn ascii_to_value(str: &[u8]) -> u8 {
    const ASCII_MASK: u8 = 0x0F;

    match str {
        b"one" => 1,
        b"two" => 2,
        b"three" => 3,
        b"four" => 4,
        b"five" => 5,
        b"six" => 6,
        b"seven" => 7,
        b"eight" => 8,
        b"nine" => 9,
        str => match str[0] {
            char @ b'1'..=b'9' => char & ASCII_MASK,
            _ => 0,
        },
    }
}
