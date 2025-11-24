import ast
import re

with open("day8_input.txt", "r")as in_file:
    path = in_file.readline().strip()
    nodes = [line.strip().split(" = ") for line in in_file if line]
nodes = nodes[1:]
graph = {}
for n in nodes:
    tuple_str = re.sub(r'\b(\w+)\b', r'"\1"', n[1])
    graph[n[0]] = ast.literal_eval(tuple_str)

current = "AAA"
end = "ZZZ"
count = 0
while current != end:
    count += 1
    inst = path[(count-1) % len(path)]
    if inst == "L":
        current = graph[current][0]

    else:
        current = graph[current][1]
print(count)
