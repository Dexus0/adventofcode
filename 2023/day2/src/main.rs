use std::io::Result;
use std::num::NonZeroU8;

fn main() -> Result<()> {
    use std::env::args_os;
    use std::fs::read;

    let input = args_os().nth(1).unwrap_or_default(); // 0th is a garbage/undefined value;
                                                      // everything after is a command line given arg.

    let result = process_data(read(input)?)?;

    println!("{result}");

    Ok(())
}

fn process_data<T: AsMut<[u8]>>(mut rawdata: T) -> Result<usize> {
    use std::io::{Error, ErrorKind};

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
    todo!()
}
