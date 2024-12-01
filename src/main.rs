mod generate_moves;

use generate_moves::generate_moves;
use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut initialized = false;
    let mut boards: [u64; 12] = [0; 12];
    let mut side = "white";

    for line in stdin.lines() {
        let command = line.unwrap();
        let parts: Vec<&str> = command.split_whitespace().collect();

        match parts[0] {
            "uci" => {
                writeln!(stdout, "id name Fathom").unwrap();
                writeln!(stdout, "id author Nate Davis").unwrap();
                writeln!(stdout, "uciok").unwrap();
            }
            "isready" => {
                writeln!(stdout, "readyok").unwrap();
            }
            "ucinewgame" => {}
            "position" => {
                if initialized {
                } else {
                    if parts[1] == "startpos" {
                        boards = [
                            0b11111111 << 8,
                            (1 << 1) | (1 << 6),
                            (1 << 2) | (1 << 5),
                            (1 << 0) | (1 << 7),
                            (1 << 3),
                            (1 << 4),
                            0b11111111 << 48,
                            (1 << 57) | (1 << 62),
                            (1 << 58) | (1 << 61),
                            (1 << 56) | (1 << 63),
                            (1 << 59),
                            (1 << 60),
                        ]
                    }

                    if parts.contains(&"moves") {
                        side = "black";

                        writeln!(stdout, "{}", parts.last().unwrap()).unwrap();
                    }

                    initialized = true
                }
            }
            "go" => {
                let moves = generate_moves(boards, side);
            }
            "quit" => break,
            _ => writeln!(stdout, "unknown").unwrap(),
        }
    }
}
