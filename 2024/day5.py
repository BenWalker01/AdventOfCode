def _is_valid(update):
    valid = True
    for rule in rules:
        a, b = rule.split("|")
        if a in update and b in update:
            if update.index(a) > update.index(b):
                valid = False
                print(f"Violates {a} | {b}")
                break
    return valid


with open("day5.txt", "r")as f:
    data = f.read().split("\n\n")

rules = [d.split("\n") for d in data][0]
updates = [d.split("\n") for d in data][1][:-1]
updates = [d.split(",") for d in updates]

answer = 0
for update in updates:
    if _is_valid(update):
        answer += int(update[len(update)//2])
print(answer)

answer = 0
for update in updates:
    if not _is_valid(update):
        while not _is_valid(update):
            for rule in rules:
                a, b = rule.split("|")
                if a in update and b in update:
                    update[update.index(a)], update[update.index(
                        b)] = update[update.index(b)], update[update.index(a)]
        answer += int(update[len(update)//2])

print(answer)
