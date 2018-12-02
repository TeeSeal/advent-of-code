from itertools import groupby

def different_char_indexes(s1, s2):
    indexes = []
    for index, char in enumerate(s1):
        if s2[index] != char:
            indexes.append(index)
    return indexes

with open('./input.txt') as f:
    ids = list(f)
    two = 0
    three = 0

    for id in ids:
        counts = [id.count(char) for char in id]
        if 2 in counts:
            two += 1
        if 3 in counts:
            three += 1

    print("Part 1:")
    print(two * three)

    result = None
    for id in ids:
        for other in ids:
            diffs = different_char_indexes(id, other)
            if len(diffs) == 1:
                index = diffs[0]
                result = id[:index] + id[(index + 1):]
                break

        if result:
            break

    print("Part 2:")
    print(result)
