def _is_valid(update):
    return all((a in update and b in update and update.index(a) < update.index(b)) or (a not in update or b not in update) for a, b in (rule.split("|") for rule in rules))


with open("day5.txt", "r")as f:
    data = f.read().split("\n\n")

rules = [d.split("\n") for d in data][0]
updates = [d.split("\n") for d in data][1][:-1]
updates = [d.split(",") for d in updates]

print(sum((int(update[len(update)//2])
      if _is_valid(update) else 0 for update in updates)))

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
