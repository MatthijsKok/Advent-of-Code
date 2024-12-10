from pathlib import Path
import re

input_file = Path(__file__).parent / "day03_input.txt"
input = input_file.read_text()

########################### PART 1 ###########################
part1_total = 0
part1_regex = re.compile(r"mul\((\d{1,3}),(\d{1,3})\)")
part1_matches = part1_regex.findall(input)

for match in part1_matches:
    part1_total += int(match[0]) * int(match[1])

print(f"Part 1 - Matches: {len(part1_matches)} - Total: {part1_total}")

########################### PART 2 ###########################
part2_total = 0
part2_input = ""
part2_regex = re.compile(r"mul\((\d{1,3}),(\d{1,3})\)")
part2_regex_enable = re.compile(r"do\(\)")
part2_regex_disable = re.compile(r"don't\(\)")
unparsed = input

while True:
    disable_match = part2_regex_disable.search(unparsed)
    if disable_match is None:
        part2_input += unparsed
        break
    part2_input += unparsed[: disable_match.start()]
    unparsed = unparsed[disable_match.end() :]
    enable_match = part2_regex_enable.search(unparsed)
    if enable_match is None:
        break
    unparsed = unparsed[enable_match.end() :]

part2_matches = part2_regex.findall(part2_input)

for match in part2_matches:
    part2_total += int(match[0]) * int(match[1])

print(f"Part 2 - Matches: {len(part2_matches)} - Total: {part2_total}")
