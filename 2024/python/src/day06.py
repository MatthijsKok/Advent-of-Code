from enum import Enum
from pathlib import Path

GRID_HEIGHT = 130
GRID_WIDTH = 130

input_file = Path(__file__).parent / "day06_input.txt"
input = input_file.read_text()
input_grid: list[list[str]] = [list(line) for line in input.splitlines()]

visited_cells = [[False] * GRID_WIDTH] * GRID_HEIGHT

print("grid height", len(input_grid))
print("grid width", len(input_grid[0]))


class Direction(Enum):
    NORTH = 0
    EAST = 1
    SOUTH = 2
    WEST = 3


def find_guard_coordinates(grid: list[list[str]]) -> tuple[int, int]:
    for i, row in enumerate(grid):
        for j, cell in enumerate(row):
            if cell == "^":
                return (i, j)
    return (-1, -1)


guard_direction = Direction.NORTH
guard_coordinates = find_guard_coordinates(input_grid)

# override position of initial guard location with "." to allow for re-visiting
input_grid[guard_coordinates[0]][guard_coordinates[1]] = "."


while True:
    x, y = guard_coordinates
    if x < 0 or x >= GRID_HEIGHT or y < 0 or y >= GRID_WIDTH:
        break
    visited_cells[x][y] = True
    cell_ahead = None
    match guard_direction:
        case Direction.NORTH:
            cell_ahead = input_grid[x - 1][y]
            # if cell_ahead == "#":
            #     print("turning east")
            #     guard_direction = Direction.EAST
            # else:
            #     print("moving north to", x - 1, y)
            #     guard_coordinates = (x - 1, y)
        case Direction.EAST:
            cell_ahead = input_grid[x][y + 1]
            # if cell_ahead == "#":
            #     print("turning south")
            #     guard_direction = Direction.SOUTH
            # else:
            #     print("moving east to", x, y + 1)
            #     guard_coordinates = (x, y + 1)
        case Direction.SOUTH:
            cell_ahead = input_grid[x + 1][y]
            # if cell_ahead == "#":
            #     print("turning west")
            #     guard_direction = Direction.WEST
            # else:
            #     print("moving south to", x + 1, y)
            #     guard_coordinates = (x + 1, y)
        case Direction.WEST:
            cell_ahead = input_grid[x][y - 1]
            # if cell_ahead == "#":
            #     print("turning north")
            #     guard_direction = Direction.NORTH
            # else:
            #     print("moving west to", x, y - 1)
            #     guard_coordinates = (x, y - 1)
    match cell_ahead:
        case "#":
            match guard_direction:
                case Direction.NORTH:
                    print("turning east")
                    guard_direction = Direction.EAST
                case Direction.EAST:
                    print("turning south")
                    guard_direction = Direction.SOUTH
                case Direction.SOUTH:
                    print("turning west")
                    guard_direction = Direction.WEST
                case Direction.WEST:
                    print("turning north")
                    guard_direction = Direction.NORTH
        case ".":
            match guard_direction:
                case Direction.NORTH:
                    print("moving north to", x - 1, y)
                    guard_coordinates = (x - 1, y)
                case Direction.EAST:
                    print("moving east to", x, y + 1)
                    guard_coordinates = (x, y + 1)
                case Direction.SOUTH:
                    print("moving south to", x + 1, y)
                    guard_coordinates = (x + 1, y)
                case Direction.WEST:
                    print("moving west to", x, y - 1)
                    guard_coordinates = (x, y - 1)

print("guard coordinates", guard_coordinates)

visited_count = 0
visited_grid = input_grid.copy()

for i, visited_row in enumerate(visited_cells):
    for j, visited_cell in enumerate(visited_row):
        if visited_cell is True:
            visited_grid[i][j] = "X"
            visited_count += 1

print("visited count", visited_count)

visited_file = Path(__file__).parent / "day06_visited.txt"
visited_file.write_text("\n".join(["".join(row) for row in visited_grid]))
