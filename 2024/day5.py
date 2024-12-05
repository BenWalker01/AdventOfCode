with open("day5.txt", "r")as f:
    data = f.read().split("\n\n")

rules = [d.split("\n") for d in data][0]
updates = [d.split("\n") for d in data][1][:-1]
updates = [d.split(",") for d in updates]

answer = 0
for update in updates:
    valid = True
    for rule in rules:
        a, b = rule.split("|")
        if a in update and b in update:
            if update.index(a) > update.index(b):
                valid = False
                break
    if valid:
        answer += int(update[len(update)//2])
print(answer)
