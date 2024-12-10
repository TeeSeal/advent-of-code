from pathlib import Path
import re

def mul(instruction: str) -> int:
    if not instruction.startswith("mul"):
        return 0

    number_pattern = re.compile(r'\d+')
    matches = re.findall(number_pattern, instruction)
    return int(matches[0]) * int(matches[1])

input_path = Path(__file__).parent / "input.txt"
data = input_path.read_text()

pattern = re.compile(r'mul\(\d+,\d+\)|do\(\)|don\'t\(\)')
matches = re.findall(pattern, data)

part_1 = sum(mul(match) for match in matches)
print(f"Part 1: {part_1}")

enabled = True
part_2 = 0

for match in matches:
    if match == "do()":
        enabled = True
    elif match == "don't()":
        enabled = False
    else:
        if enabled:
            part_2 += mul(match)

print(f"Part 2: {part_2}")
