use std::cmp::Ordering::{Equal, Greater, Less};
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer: Vec<f64> = Vec::new();
    let mut input: String = String::new();
    let mut peaks = Vec::new();

    while stdin.read_line(&mut input)? != 0 {
        match std::mem::take(&mut input).trim().to_string().parse::<f64>() {
            Ok(float) => buffer.push(float),
            Err(_) => println!("Invalid input"),
        }
    }

    if buffer.len() == 0 {
        println!("Invalid input");
        return Ok(());
    }

    let instant = std::time::Instant::now();

    let window_size = 400;

    for i in 1 + window_size..buffer.len() - window_size {
        let window = &buffer[i - window_size..i + window_size];
        match window
            .iter()
            .enumerate()
            .max_by(|(_, x), (_, y)| x.partial_cmp(y).unwrap())
        {
            Some((j, _)) => {
                if i == (i - window_size - 1 + j) {
                    peaks.push(i);
                    // writeln!(stdout, "{}", i)?;
                }
            }
            None => (),
        }
    }

    writeln!(stdout, "{:?} - Reference", instant.elapsed())?;

    peaks.clear();

    let instant = std::time::Instant::now();

    // Not really happy with the name 'window_size', it is actually the half of the window size
    let window_size = 400;
    let mut index = window_size + 1;

    while index < buffer.len() - window_size + 1 {
        let window = &buffer[index - window_size..index + window_size];
        match window
            .iter()
            .enumerate()
            // Beware of teh Unwrap
            .max_by(|(_, x), (_, y)| x.partial_cmp(y).unwrap())
        {
            Some((j, _)) => match j.cmp(&window_size) {
                Less => index += j + 1,
                Greater => index += j - window_size,
                Equal => {
                    // writeln!(stdout, "{}", index)?;
                    peaks.push(index);
                    index += window_size;
                }
            },
            None => (),
        }
    }

    writeln!(
        stdout,
        "{:?} - Reference 'compare-swap' optimized",
        instant.elapsed()
    )?;

    peaks.clear();

    let instant = std::time::Instant::now();

    let targeted_age = 400;
    let mut age = 0;
    let mut value = 0.0;

    buffer.iter().enumerate().for_each(|(index, new)| {
        if *new > value {
            value = *new;
            age = 0;
        } else {
            age += 1;
        }

        if age == targeted_age {
            value = *new;
            age = 0;
            peaks.push(index - targeted_age);
        }
    });

    writeln!(stdout, "{:?} - Streamable distant-based", instant.elapsed())?;

    Ok(())
}
