use std::collections::HashMap;
use std::time::Instant;

fn is_valid_chess_board(board: &HashMap<&str, &str>) -> bool {
    const VALID_POSITIONS: [&str; 64] = [
        "1a", "1b", "1c", "1d", "1e", "1f", "1g", "1h", "2a", "2b", "2c", "2d", "2e", "2f", "2g",
        "2h", "3a", "3b", "3c", "3d", "3e", "3f", "3g", "3h", "4a", "4b", "4c", "4d", "4e", "4f",
        "4g", "4h", "5a", "5b", "5c", "5d", "5e", "5f", "5g", "5h", "6a", "6b", "6c", "6d", "6e",
        "6f", "6g", "6h", "7a", "7b", "7c", "7d", "7e", "7f", "7g", "7h", "8a", "8b", "8c", "8d",
        "8e", "8f", "8g", "8h",
    ];

    const PIECE_NAMES: [&str; 6] = ["pawn", "knight", "bishop", "rook", "queen", "king"];
    // establish the piece count for us to keep track of how many pieces are on the board or each color
    let mut w_pieces = 0;
    let mut b_pieces = 0;
    let mut w_kings = 0;
    let mut b_kings = 0;
    let mut w_pawns = 0;
    let mut b_pawns = 0;

    for (pos, piece) in board {
        // Check position validity
        if !VALID_POSITIONS.contains(pos) {
            return false;
        }

        // Check piece validity and count pieces
        let (color, name) = piece.split_at(1);
        if !PIECE_NAMES.contains(&name) || (color != "w" && color != "b") {
            return false;
        }

        match color {
            "w" => {
                w_pieces += 1;
                if name == "king" {
                    w_kings += 1
                }
                if name == "pawn" {
                    w_pawns += 1
                }
            }
            "b" => {
                b_pieces += 1;
                if name == "king" {
                    b_kings += 1
                }
                if name == "pawn" {
                    b_pawns += 1
                }
            }
            _ => return false,
        }

        // Early returns for invalid conditions
        if w_kings > 1
            || b_kings > 1
            || w_pieces > 16
            || b_pieces > 16
            || w_pawns > 8
            || b_pawns > 8
        {
            return false;
        }
    }

    w_kings == 1 && b_kings == 1
}

fn main() {
    let start_time = Instant::now();
    let board = HashMap::from([
        ("1h", "bking"),
        ("6c", "wqueen"),
        ("2g", "bbishop"),
        ("5h", "bqueen"),
        ("3e", "wking"),
    ]);
    let result = is_valid_chess_board(&board);
    let elapsed_time = start_time.elapsed();
    println!("Result: {}", result);
    println!("Time taken: {:.6} seconds", elapsed_time.as_secs_f64());
}
