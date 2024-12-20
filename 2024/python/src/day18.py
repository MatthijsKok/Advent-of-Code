from pathlib import Path

SIZE = 71

type Vertex = tuple[int, int]
type Edge = tuple[Vertex, Vertex]

input_file = Path(__file__).parent / "day18_input.txt"
input = input_file.read_text()
input_lines = input.splitlines()

memory_space = [["." for _ in range(SIZE)] for _ in range(SIZE)]

# for line in input_lines[0:1000]:
#     l1, l2 = line.split(",")
#     memory_space[int(l1)][int(l2)] = "#"

for i in range(len(input_lines), 0, -1):
    memory_space = [["." for _ in range(SIZE)] for _ in range(SIZE)]
    for line in input_lines[:i]:
        l1, l2 = line.split(",")
        memory_space[int(l1)][int(l2)] = "#"

    vertices: list[Vertex] = []
    distances: dict[Vertex, int] = {}
    previous: dict[Vertex, Vertex | None] = {}
    queue: list[Vertex] = []

    for x in range(SIZE):
        for y in range(SIZE):
            if memory_space[x][y] == "#":
                continue
            vertex = (x, y)
            vertices.append(vertex)
            distances[vertex] = 1_000_000
            previous[vertex] = None
            queue.append(vertex)

    distances[(0, 0)] = 0

    while len(queue) > 0:
        # find vertex still in the queue with the smallest distance
        current_vertex = queue[0]
        for vertex in queue:
            if distances[vertex] < distances[current_vertex]:
                current_vertex = vertex
        queue.remove(current_vertex)

        if current_vertex == (SIZE - 1, SIZE - 1):
            # print("FOUND IT")
            # print(distances[current_vertex])
            print(f"After adding wall n{i} ({l1}, {l2}), the shortest path is {distances[current_vertex]}")
            if distances[current_vertex] == 1_000_000:
                print("FOUND IT")
                print(distances[current_vertex])
            break

        # find neighbors of current_vertex that are still in the queue, and not walls
        neighbors = []
        x, y = current_vertex
        if x > 0 and memory_space[x - 1][y] != "#" and (x - 1, y) in queue:
            neighbors.append((x - 1, y))
        if y > 0 and memory_space[x][y - 1] != "#" and (x, y - 1) in queue:
            neighbors.append((x, y - 1))
        if x < SIZE - 1 and memory_space[x + 1][y] != "#" and (x + 1, y) in queue:
            neighbors.append((x + 1, y))
        if y < SIZE - 1 and memory_space[x][y + 1] != "#" and (x, y + 1) in queue:
            neighbors.append((x, y + 1))

        for neighbor in neighbors:
            alt = distances[current_vertex] + 1
            if alt < distances[neighbor]:
                distances[neighbor] = alt
                previous[neighbor] = current_vertex
