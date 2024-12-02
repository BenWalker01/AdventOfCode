with open("day2.txt", "r")as f:
    data = [[int(x) for x in line.split(" ")]
            for line in f.read().splitlines()]
safe = 0
for line in data:
    if all([abs(line[i] - line[i+1]) in (1, 2, 3)
            for i in range(len(line) - 1)]) and (all(line[i] < line[i+1] for i in range(len(line) - 1))
                                                 or all(line[i] > line[i+1] for i in range(len(line) - 1))):
        safe += 1

print(safe)
