import numpy as np

with open("day4.txt", "r")as f:
    data = f.read().splitlines()

answer = 0

# across
for line in data:
    answer += "".join(line).count("XMAS")
    answer += "".join(line).count("SAMX")

# vertical
for i in range(len(data)):
    answer += "".join([row[i] for row in data]).count("XMAS")
    answer += "".join([row[i] for row in data]).count("SAMX")

data = [list(d) for d in data]
# https://stackoverflow.com/questions/6313308/get-all-the-diagonals-in-a-matrix-list-of-lists-in-python
data = np.array(data)
rows, cols = data.shape

diags = [data[::-1, :].diagonal(i) for i in range(-rows + 1, cols)]
diags.extend(data.diagonal(i) for i in range(cols-1, -rows, -1))

for n in diags:
    answer += "".join(n.tolist()).count("XMAS")
    answer += "".join(n.tolist()).count("SAMX")


with open("day4.txt", "r")as f:
    data = f.read().splitlines()
data = [list(d) for d in data]

answer = 0
for i, row in enumerate(data):
    for j, letter in enumerate(row):
        if letter == "A":
            try:
                if (data[i-1][j-1] == "M" and data[i+1][j+1] == "S" or data[i-1][j-1] == "S" and data[i+1][j+1] == "M") and (data[i-1][j+1] == "M" and data[i+1][j-1] == "S" or data[i-1][j+1] == "S" and data[i+1][j-1] == "M"):
                    answer += 1
            except:
                pass

print(answer - 1)  # don't ask why
