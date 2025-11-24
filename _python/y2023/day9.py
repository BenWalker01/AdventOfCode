with open("day9_input.txt", "r") as in_file:
    lines = in_file.readlines()

total = 0
for line in lines:
    sequence = [int(l) for l in line.split()]

    sequences = [sequence]

    while sequences[-1] != [0] * (len(sequences[-1]) - 1):
        diff = [j - i for i, j in zip(sequences[-1][:-1], sequences[-1][1:])]
        sequences.append(diff)

    for i in range(len(sequences) - 3, -1, -1):
        sequences[i].append(sequences[i][-1] + sequences[i+1][-1])

    next_value = sequences[0][-1]
    total += next_value
    # print(next_value)
print(total)

###### part 2 ############

with open("day9_input.txt", "r") as in_file:
    lines = in_file.readlines()

total = 0
for line in lines:
    sequence = [int(l) for l in line.split()]
    sequence.reverse()

    sequences = [sequence]

    while sequences[-1] != [0] * (len(sequences[-1]) - 1):
        diff = [j - i for i, j in zip(sequences[-1][:-1], sequences[-1][1:])]
        sequences.append(diff)

    for i in range(len(sequences) - 3, -1, -1):
        sequences[i].append(sequences[i][-1] + sequences[i+1][-1])

    next_value = sequences[0][-1]
    total += next_value
    # print(next_value)
print(total)
