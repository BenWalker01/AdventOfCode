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


def _generate_valid(update):
    valid_list = []
    for rule in rules:
        a, b = rule.split("|")
        if a in update and b in update:
            if a in valid_list and b in valid_list:
                if valid_list.index(a) > valid_list.index(b):
                    valid_list[valid_list.index(a)], valid_list[valid_list.index(
                        b)] = valid_list[valid_list.index(b)], valid_list[valid_list.index(a)]
            elif a in valid_list:
                valid_list.insert(valid_list.index(a) + 1, b)
            elif b in valid_list:
                valid_list.insert(valid_list.index(b), a)
            else:
                valid_list.append(a)
                valid_list.append(b)
    return valid_list


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
