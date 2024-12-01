pub fn generate_moves(boards: [u64; 12], side: &str) -> Vec<(u8, u8, Option<char>)> {
    let mut moves = Vec::new();

    let offset = if side == "white" { 0 } else { 6 };

    let mut pawns = boards[offset];

    while pawns != 0 {
        let index = pawns.trailing_zeros() as u8;

        moves.push((index, 0, None));

        pawns &= pawns - 1;
    }

    return moves;
}
