with open("2024/day1_1.txt")as day1:
    data = day1.read().splitlines()

list1 = sorted([int(d.split("   ")[0]) for d in data])
list2 = sorted([int(d.split("   ")[1]) for d in data])


answer = 0
for a,b in zip(list1,list2):
    answer += abs(a-b)

print(f"Part 1: {answer}")
answer = 0

for num in list1:
    answer += num * list2.count(num)

print(f"Part 2: {answer}") 
