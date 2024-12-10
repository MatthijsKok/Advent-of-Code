import math
from pathlib import Path

input_file = Path(__file__).parent / "day05_input.txt"
input = input_file.read_text()
input_lines = input.splitlines()

type Rule = tuple[int, int]
type Update = list[int]

input_rules: list[Rule] = []
input_updates: list[Update] = []

while len(input_lines) > 0:
    line = input_lines.pop(0)
    if line == "":
        break
    r1, r2 = line.split("|")
    input_rules.append((int(r1), int(r2)))

while len(input_lines) > 0:
    line = input_lines.pop(0)
    update = list(map(int, line.split(",")))
    input_updates.append(update)

print(input_rules[0:10])
print()
print(input_updates[0:10])
print()

def get_active_rules(update: Update) -> list[Rule]:
    active_rules = []
    for rule in input_rules:
        if rule[0] in update and rule[1] in update:
            active_rules.append(rule)
    return active_rules

update_1_rules = get_active_rules(input_updates[0])
print(update_1_rules)
print()

def update_follows_rules(update: Update, active_rules: list[Rule]) -> bool:
    for rule in active_rules:
        if update.index(rule[0]) > update.index(rule[1]):
            return False
    return True


correct_updates: list[Update] = []
for update in input_updates:
    active_rules = get_active_rules(update)
    if update_follows_rules(update, active_rules):
        correct_updates.append(update)


def get_middle_index(update: Update) -> int:
    return math.floor(len(update) / 2)


result = 0
for update in correct_updates:
    middle_index = get_middle_index(update)
    result += update[middle_index]

print("correct updates middle index sum:", result)
print("")

##############################################################################
print("================ PART 2 ================")
print()

incorrect_updates: list[Update] = []
for update in input_updates:
    active_rules = get_active_rules(update)
    if not update_follows_rules(update, active_rules):
        incorrect_updates.append(update)


def get_first_number_index(update: Update, active_rules: list[Rule]) -> int:
    rule_ends = [rule[1] for rule in active_rules]
    for i, number in enumerate(update):
        if number not in rule_ends:
            return i
    return -1


incorrect_updates_sorted = []
for update in incorrect_updates:
    # active_rules = get_active_rules(update)
    sorted_update: list[int] = []
    unsorted_update: list[int] = update.copy()
    while len(unsorted_update) > 0:
        unsorted_active_rules = get_active_rules(unsorted_update)
        first_number_index = get_first_number_index(unsorted_update, unsorted_active_rules)
        first_number = unsorted_update.pop(first_number_index)
        sorted_update.append(first_number)
    incorrect_updates_sorted.append(sorted_update)

# print(incorrect_updates_sorted[0:10])


for update in incorrect_updates_sorted:
    if not update_follows_rules(update, get_active_rules(update)):
        print("incorrect update WRONG:", update)


result_only_incorrect = 0
for update in incorrect_updates_sorted:
    middle_index = get_middle_index(update)
    result_only_incorrect += update[middle_index]

print("incorrect updates middle index sum:", result_only_incorrect)
print()
