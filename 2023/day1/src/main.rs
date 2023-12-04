use std::ffi::OsStr;
use std::io::{ErrorKind, Result};

fn main() -> Result<()> {
    use std::env::args_os;

    let input = args_os().nth(1).unwrap_or_default(); // 0th is a garbage/undefined value

    let result = process_file(&input)?;

    println!("{result}");

    Ok(())
}

fn process_file(file: &OsStr) -> Result<usize> {
    use std::fs::read;
    use std::io::Error;

    let rawdata = read(file)?;

    let calibration_file = if rawdata.is_ascii() {
        rawdata
    } else {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "File was not valid ASCII",
        ));
    };

    let calibration_lines = calibration_file.split(|x| *x == b'\n');

    let mut calibration_sum = 0;

    for line in calibration_lines {
        calibration_sum += calculate_calibration_value(line);
    }
    Ok(calibration_sum)
}

fn calculate_calibration_value(line: &[u8]) -> usize {
    const ZERO: u8 = b'0';
    const NINE: u8 = b'9';
    const ASCII_MASK: u8 = b'0';

    let mut first = 0;
    let mut last = 0;

    for char in line {
        match *char {
            ZERO..=NINE => {
                first = *char;
                first ^= ASCII_MASK;
                break;
            }
            _ => continue,
        }
    }

    for char in line.iter().rev() {
        match *char {
            ZERO..=NINE => {
                last = *char;
                last ^= ASCII_MASK;
                break;
            }
            _ => continue,
        }
    }

    println!("{first}{last}");

    (first * 10 + last) as usize
}
