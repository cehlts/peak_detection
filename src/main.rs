use std::io::{self, Write};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer: Vec<f64> = Vec::new();
    let mut input: String = String::new();

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

    let window_size = 400;
    let buffer = buffer
        .into_iter()
        .enumerate()
        .collect::<Vec<(usize, f64)>>();

    for window in buffer.windows(window_size) {
        match window
            .iter()
            .enumerate()
            .max_by(|(_, (_, x)), (_, (_, y))| x.partial_cmp(y).unwrap())
        {
            Some((relative, (absolute, value))) => {
                if relative == 0 {
                    writeln!(stdout, "{} - {}", absolute, value)?;
                }
            }
            None => (),
        }
    }

    Ok(())
}
