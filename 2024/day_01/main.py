from pathlib import Path

input_path = Path(__file__).parent / "input.txt"
data = [int(num) for num in input_path.read_text().split()]

set1 = data[::2]
set2 = data[1::2]

pairs = zip(sorted(set1), sorted(set2))
part_1 = sum(abs(pair[0] - pair[1]) for pair in pairs)
print(f"Part 1: {part_1}")

part_2 = sum(num * set2.count(num) for num in set1)
print(f"Part 2: {part_2}")
