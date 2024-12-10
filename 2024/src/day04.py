# /// script
# requires-python = ">=3.13"
# ///
from pathlib import Path
from typing import Literal

MATRIX_HEIGHT = 140
MATRIX_WIDTH = 140

type Chars = Literal["X", "M", "A", "S"]

input_file = Path(__file__).parent / "day04_input.txt"
input = input_file.read_text()
input_matrix: list[list[Chars]] = [list(line) for line in input.splitlines()]


def get_rows(matrix: list[list[Chars]]) -> list[str]:
    return ["".join(row) for row in matrix]


def get_columns(matrix: list[list[Chars]]) -> list[str]:
    return ["".join(column) for column in zip(*matrix)]


def get_diagonals(
    matrix: list[list[Chars]],
    direction: Literal["topleft", "topright"],
) -> list[str]:
    diagonals: list[list[Chars]] = []
    if direction == "topleft":
        for i in range(MATRIX_HEIGHT):
            diagonal = []
            for j in range(i + 1):
                diagonal.append(matrix[i - j][j])
            diagonals.append(diagonal)
        for i in range(1, MATRIX_WIDTH):
            diagonal = []
            for j in range(MATRIX_HEIGHT - i):
                diagonal.append(matrix[MATRIX_HEIGHT - j - 1][i + j])
            diagonals.append(diagonal)
    elif direction == "topright":
        for i in range(MATRIX_HEIGHT):
            diagonal = []
            for j in range(i + 1):
                diagonal.append(matrix[i - j][MATRIX_WIDTH - j - 1])
            diagonals.append(diagonal)
        for i in range(1, MATRIX_WIDTH):
            diagonal = []
            for j in range(MATRIX_HEIGHT - i):
                diagonal.append(matrix[MATRIX_HEIGHT - j - 1][MATRIX_WIDTH - i - j - 1])
            diagonals.append(diagonal)
    return ["".join(diagonal) for diagonal in diagonals]


rows = get_rows(input_matrix)
rows_reversed = [row[::-1] for row in rows]
columns = get_columns(input_matrix)
columns_reversed = [column[::-1] for column in columns]
topleft_diagonals = get_diagonals(input_matrix, "topleft")
topleft_diagonals_reversed = [diagonal[::-1] for diagonal in topleft_diagonals]
topright_diagonals = get_diagonals(input_matrix, "topright")
topright_diagonals_reversed = [diagonal[::-1] for diagonal in topright_diagonals]

directions = [
    rows,
    rows_reversed,
    columns,
    columns_reversed,
    topleft_diagonals,
    topleft_diagonals_reversed,
    topright_diagonals,
    topright_diagonals_reversed,
]

xmas_count = 0
for direction in directions:
    for line in direction:
        xmas_count += line.count("XMAS")
print(f'"XMAS" count: {xmas_count}')


# def is_x_mas(subgrid: list[list[int]]) -> bool:
#     if subgrid[1][1] is not "A":
#         return False

#     return False

type Coord = tuple[int, int]


def is_x_mas(center: Coord) -> bool:
    if center[0] <= 0 or center[0] >= MATRIX_HEIGHT - 1:
        return False
    if center[1] <= 0 or center[1] >= MATRIX_HEIGHT - 1:
        return False
    if input_matrix[center[0]][center[1]] != "A":
        return False
    if (
        input_matrix[center[0] - 1][center[1] - 1] == "M"
        and input_matrix[center[0] - 1][center[1] + 1] == "M"
        and input_matrix[center[0] + 1][center[1] - 1] == "S"
        and input_matrix[center[0] + 1][center[1] + 1] == "S"
    ):
        return True
    if (
        input_matrix[center[0] - 1][center[1] - 1] == "M"
        and input_matrix[center[0] - 1][center[1] + 1] == "S"
        and input_matrix[center[0] + 1][center[1] - 1] == "M"
        and input_matrix[center[0] + 1][center[1] + 1] == "S"
    ):
        return True
    if (
        input_matrix[center[0] - 1][center[1] - 1] == "S"
        and input_matrix[center[0] - 1][center[1] + 1] == "S"
        and input_matrix[center[0] + 1][center[1] - 1] == "M"
        and input_matrix[center[0] + 1][center[1] + 1] == "M"
    ):
        return True
    if (
        input_matrix[center[0] - 1][center[1] - 1] == "S"
        and input_matrix[center[0] - 1][center[1] + 1] == "M"
        and input_matrix[center[0] + 1][center[1] - 1] == "S"
        and input_matrix[center[0] + 1][center[1] + 1] == "M"
    ):
        return True
    return False


x_mas_count = 0
for i in range(MATRIX_HEIGHT):
    for j in range(MATRIX_WIDTH):
        if is_x_mas((i, j)):
            x_mas_count += 1

print(f'"X-MAS" count: {x_mas_count}')
