with open("2024/day1_1.txt")as day1:
    data = day1.read().splitlines()

list1 = sorted([int(d.split("   ")[0]) for d in data])
list2 = sorted([int(d.split("   ")[1]) for d in data])


sum_ = 0
for a,b in zip(list1,list2):
    sum_ += abs(a-b)

print(sum_)