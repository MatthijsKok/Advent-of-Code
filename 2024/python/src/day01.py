import time
from pathlib import Path

input_file = Path(__file__).parent / "day01_input.txt"
input = input_file.read_text()

t1 = time.perf_counter_ns()

left: list[int] = []
right: list[int] = []

for line in input.splitlines():
    left.append(int(line[:5]))
    right.append(int(line[8:]))

left.sort()
right.sort()

sum = 0
for left_item, right_item in zip(left, right, strict=True):
    sum += abs(left_item - right_item)
print(f"Part 1: {sum}")

similarity = 0
lookup = [0] * 100_000
for i in right:
    lookup[i] += i
for i in left:
    similarity += lookup[i]
print(f"Part 2: {similarity}")

t2 = time.perf_counter_ns()
print(f"Time: {(t2 - t1) / 1_000_000:.3f}ms")
