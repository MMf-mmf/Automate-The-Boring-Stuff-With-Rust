import time


def printTable(tableData):
    # Step 1: Determine the maximum width of each column
    colWidths = [0] * len(tableData)

    for i in range(len(tableData)):
        colWidths[i] = max(len(item) for item in tableData[i])

    # Step 2: Print the table row by row
    numRows = len(tableData[0])
    for row in range(numRows):
        for col in range(len(tableData)):
            print(tableData[col][row].rjust(colWidths[col]), end=" ")
        print()  # Newline after each row


# Example usage
start_time = time.time()
tableData = [
    ["apples", "oranges", "cherries", "banana"],
    ["Alice", "Bob", "Carol", "David"],
    ["dogs", "cats", "moose", "goose"],
]

printTable(tableData)
end_time = time.time()
elapsed_time = end_time - start_time

print(f"Time taken: {elapsed_time:.6f} seconds")
