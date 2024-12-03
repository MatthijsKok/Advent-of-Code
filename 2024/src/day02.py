from pathlib import Path
import time
from typing import Literal

input_file = Path(__file__).parent / "day02_input.txt"
input = input_file.read_text()

total_safe = 0
total_safe_part2 = 0
reports: list[list[int]] = []

for line in input.splitlines():
    report: list[int] = [int(lvl) for lvl in line.split(sep=" ")]
    reports.append(report)


def in_bounds(prev_level: int, level: int) -> bool:
    return 1 <= abs(prev_level - level) <= 3

def is_safe(report: list[int]) -> bool:
    report_kind: Literal["asc"] | Literal["desc"] | None = None
    prev_lvl: int | None = None
    for level in report:
        if prev_lvl is None:
            prev_lvl = level
            continue
        if not in_bounds(prev_lvl, level):
            return False
        if report_kind is None:
            if prev_lvl > level:
                report_kind = "desc"
            elif prev_lvl < level:
                report_kind = "asc"
        if report_kind == "asc" and prev_lvl > level:
            return False
        if report_kind == "desc" and prev_lvl < level:
            return False
        prev_lvl = level
    return True


for report in reports:
    if is_safe(report):
        total_safe += 1


for report in reports:
    if is_safe(report):
        total_safe_part2 += 1
    else:
        for i, level in enumerate(report):
            new_report = report.copy()
            new_report.pop(i)
            if is_safe(new_report):
                total_safe_part2 += 1
                break




print(f"Part 1: {total_safe}")
print(f"Part 2: {total_safe_part2}")
