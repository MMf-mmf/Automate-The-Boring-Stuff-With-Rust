import time


def isValidChessBoard(board):
    # Define valid positions and piece names
    valid_positions = [
        "1a",
        "1b",
        "1c",
        "1d",
        "1e",
        "1f",
        "1g",
        "1h",
        "2a",
        "2b",
        "2c",
        "2d",
        "2e",
        "2f",
        "2g",
        "2h",
        "3a",
        "3b",
        "3c",
        "3d",
        "3e",
        "3f",
        "3g",
        "3h",
        "4a",
        "4b",
        "4c",
        "4d",
        "4e",
        "4f",
        "4g",
        "4h",
        "5a",
        "5b",
        "5c",
        "5d",
        "5e",
        "5f",
        "5g",
        "5h",
        "6a",
        "6b",
        "6c",
        "6d",
        "6e",
        "6f",
        "6g",
        "6h",
        "7a",
        "7b",
        "7c",
        "7d",
        "7e",
        "7f",
        "7g",
        "7h",
        "8a",
        "8b",
        "8c",
        "8d",
        "8e",
        "8f",
        "8g",
        "8h",
    ]
    piece_names = ["pawn", "knight", "bishop", "rook", "queen", "king"]

    # Initialize counters for pieces
    piece_count = {"w": 0, "b": 0}
    king_count = {"w": 0, "b": 0}
    pawn_count = {"w": 0, "b": 0}

    for position, piece in board.items():
        # Check if position is valid
        if position not in valid_positions:
            return False

        # Check if piece name is valid
        if piece[0] not in "wb" or piece[1:] not in piece_names:
            return False

        # Count pieces
        color = piece[0]
        piece_count[color] += 1

        # Count kings
        if piece[1:] == "king":
            king_count[color] += 1

        # Count pawns
        if piece[1:] == "pawn":
            pawn_count[color] += 1

    # Check for exactly one king of each color
    if king_count["w"] != 1 or king_count["b"] != 1:
        return False

    # Check for at most 16 pieces of each color
    if piece_count["w"] > 16 or piece_count["b"] > 16:
        return False

    # Check for at most 8 pawns of each color
    if pawn_count["w"] > 8 or pawn_count["b"] > 8:
        return False

    return True


# Example usage
start_time = time.time()
board = {"1h": "bking", "6c": "wqueen", "2g": "bbishop", "5h": "bqueen", "3e": "wking"}
result = isValidChessBoard(board)
end_time = time.time()
elapsed_time = end_time - start_time
print(result)
print(f"Time taken: {elapsed_time:.6f} seconds")
