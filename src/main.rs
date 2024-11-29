use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lines() {
        let input = line.unwrap();

        match input.as_str() {
            "uci" => {
                writeln!(stdout, "id name Fathom").unwrap();
                writeln!(stdout, "id author Nate Davis").unwrap();
                writeln!(stdout, "uciok").unwrap();
            }
            _ => {
                writeln!(stdout, "Unknown command: {}", input).unwrap();
            }
        }
    }
}
