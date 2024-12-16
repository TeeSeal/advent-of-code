from pathlib import Path
from typing import NamedTuple, Callable
from collections import Counter

input_path = Path(__file__).parent / "input.txt"
data = input_path.read_text()


class Point(NamedTuple):
    x: int
    y: int


PointTransformer = Callable[[Point], Point]


class Matrix:
    def __init__(self, text: str):
        self.rows = [list(line) for line in data.splitlines()]

    def at(self, point: Point) -> str | None:
        if point.x < 0 or point.y < 0:
            return None

        try:
            return self.rows[point.y][point.x]
        except IndexError:
            return None


matrix = Matrix(data)


def build_word(
    point: Point, predicate: PointTransformer, length: int = 4, result: str = ""
) -> str:
    if length == 0:
        return result

    char = matrix.at(point)
    if char is None:
        return result

    return build_word(
        predicate(point),
        predicate,
        length - 1,
        result + char,
    )


ort_transformers: list[PointTransformer] = [
    lambda p: Point(p.x + 1, p.y),
    lambda p: Point(p.x, p.y + 1),
    lambda p: Point(p.x - 1, p.y),
    lambda p: Point(p.x, p.y - 1),
]

diag_transformers: list[PointTransformer] = [
    lambda p: Point(p.x + 1, p.y + 1),
    lambda p: Point(p.x - 1, p.y + 1),
    lambda p: Point(p.x - 1, p.y - 1),
    lambda p: Point(p.x + 1, p.y - 1),
]

part_1 = 0
mas_centers = []
for y in range(len(matrix.rows)):
    for x in range(len(matrix.rows[0])):
        coords = Point(x, y)

        if matrix.at(coords) == "X":
            for transformer in ort_transformers + diag_transformers:
                word = build_word(coords, transformer)
                if word == "XMAS":
                    part_1 += 1

        if matrix.at(coords) == "M":
            for transformer in diag_transformers:
                word = build_word(coords, transformer, length=3)
                if word == "MAS":
                    mas_centers.append(transformer(coords))


print(f"Part 1: {part_1}")

part_2 = list(Counter(mas_centers).values()).count(2)
print(f"Part 2: {part_2}")
