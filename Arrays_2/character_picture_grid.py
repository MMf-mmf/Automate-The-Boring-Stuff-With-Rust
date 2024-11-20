import time

start_time = time.time()
grid = [
    [".", ".", ".", ".", ".", "."],
    [".", "O", "O", ".", ".", "."],
    ["O", "O", "O", "O", ".", "."],
    ["O", "O", "O", "O", "O", "."],
    [".", "O", "O", "O", "O", "O"],
    ["O", "O", "O", "O", "O", "."],
    ["O", "O", "O", "O", ".", "."],
    [".", "O", "O", ".", ".", "."],
    [".", ".", ".", ".", ".", "."],
]

width = len(grid[0])
height = len(grid)
result = []

for col in range(width):
    for row in grid:
        result.append(row[col])
    result.append("\n")

result_string = "".join(result)
end_time = time.time()
print(result_string)
elapsed_time = end_time - start_time
print(f"Time taken: {elapsed_time:.6f} seconds")
