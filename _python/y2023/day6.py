with open("day6_input.txt", "r")as in_file:
    times = in_file.readline().strip()
    distances = in_file.readline().strip()

times = [time for time in times.split() if times]
distances = [dist for dist in distances.split() if dist]

total = 1

for t, d in zip(times[1:], distances[1:]):
    further = 0
    for speed in range(int(t)):
        distance = speed * (int(t) - speed)
        if distance > int(d):
            further += 1

    total *= further
print(total)

######################################################

with open("day6_input.txt", "r")as in_file:
    times = in_file.readline().strip()
    distances = in_file.readline().strip()

times = [time for time in times.split() if times]
distances = [dist for dist in distances.split() if dist]

times = int("".join(times[1:]))
distances = int("".join(distances[1:]))

print(times, distances)

further = 0
for speed in range(times):
    distance = speed * (times - speed)
    if distance > distances:
        further += 1

print(further)
