def sum_part_numbers(filename):
    with open(filename, 'r') as file:
        lines = [list(line.strip()) for line in file.readlines()]

    directions = [(0, 1), (1, 0), (0, -1), (-1, 0),
                  (-1, -1), (-1, 1), (1, -1), (1, 1)]
    symbols = set(['*', '#', '+', '$'])
    total = 0

    for i in range(len(lines)):
        for j in range(len(lines[i])):
            if lines[i][j].isdigit():
                if any(0 <= i + di < len(lines) and 0 <= j + dj < len(lines[i]) and lines[i + di][j + dj] in symbols for di, dj in directions):
                    start, end = j, j
                    while start - 1 >= 0 and lines[i][start - 1].isdigit():
                        start -= 1
                    while end + 1 < len(lines[i]) and lines[i][end + 1].isdigit():
                        end += 1
                    total += int(''.join(lines[i][start:end+1]))

    return total


print(sum_part_numbers('day3_input.txt'))
