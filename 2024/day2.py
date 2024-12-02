def is_valid(report: list[int]) -> bool:
    return all([abs(report[i] - report[i+1]) in (1, 2, 3)
                for i in range(len(report) - 1)]) and (all(report[i] < report[i+1] for i in range(len(report) - 1))
                                                       or all(report[i] > report[i+1] for i in range(len(report) - 1)))


with open("day2.txt", "r")as f:
    data = [[int(x) for x in line.split(" ")]
            for line in f.read().splitlines()]
safe = 0
for line in data:
    if is_valid(line):
        safe += 1

print(f"Part 1: {safe}")

safe = 0
for line in data:
    if not is_valid(line):
        if [is_valid(line[:i] + line[1+i:]) for i in range(len(line))].count(True) >= 1:
            safe += 1
    else:
        safe += 1

print(f"Part 2 : {safe}")
