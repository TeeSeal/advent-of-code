from pathlib import Path
from typing import Sequence

input_path = Path(__file__).parent / "input.txt"
data = input_path.read_text()

reports = [[int(num) for num in line.split()] for line in data.split("\n")]
reports.pop()

def is_report_safe(report: Sequence[int]) -> bool:
    differences = [report[i] - report[i + 1] for i in range(len(report) - 1)]
    if differences[0] == 0:
        return False

    sign_check = (lambda x: x > 0) if differences[0] > 0 else (lambda x: x < 0)
    return all(sign_check(diff) and abs(diff) <= 3 for diff in differences)

def is_report_kinda_safe(report: Sequence[int]) -> bool:
    if is_report_safe(report):
        return True

    for i in range(len(report)):
        report_copy = list(report).copy()
        del report_copy[i];

        if is_report_safe(report_copy):
            return True


part_1 = [is_report_safe(report) for report in reports].count(True)
print(f"Part 1: {part_1}")
part_2 = [is_report_kinda_safe(report) for report in reports].count(True)
print(f"Part 2: {part_2}")
