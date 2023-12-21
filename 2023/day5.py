
# def find_lowest_location(filename):
#     with open(filename, 'r') as file:
#         seeds = list(map(int, file.readline().strip().split()[1:]))
#         maps = []
#         buffer = []
#         for line in file:
#             if line.strip().endswith('map:'):
#                 if buffer:
#                     maps.append(buffer)
#                 buffer = [line.strip()]
#             else:
#                 buffer.append(line.strip())
#         if buffer:
#             maps.append(buffer)

#     locations = []
#     for seed in seeds:
#         n = seed
#         for mpt in maps:
#             n = apply_map(n, mpt)
#         locations.append(n)

#     return min(locations)

from bisect import bisect_right


def apply_map(n, mpt):
    # Convert the map to a list of tuples
    ranges = [(int(line.split()[1]), int(line.split()[0]), int(
        line.split()[2])) for line in mpt[1:] if line.strip()]
    # Sort the list by the start of the source range
    ranges.sort()

    # Use binary search to find the appropriate range
    i = bisect_right(ranges, (n,)) - 1
    if i >= 0 and ranges[i][0] <= n < ranges[i][0] + ranges[i][2]:
        return ranges[i][1] + (n - ranges[i][0])
    return n


def grab_seeds(filename):
    with open(filename, 'r') as file:
        seed_ranges = list(map(int, file.readline().strip().split()[1:]))
        for i in range(0, len(seed_ranges), 2):
            start, length = seed_ranges[i], seed_ranges[i+1]
            for seed in range(start, start + length):
                yield seed


def get_maps(filename):
    with open(filename, 'r') as file:
        maps = []
        current_map = []
        for line in file:
            if line.strip().endswith('map:'):
                if current_map:
                    maps.append(current_map)
                current_map = [line.strip()]
            else:
                current_map.append(line.strip())
        if current_map:
            maps.append(current_map)
        return maps


maps = get_maps("day5_input.txt")
print("got maps")
seeds = grab_seeds("day5_input.txt")
print("Seeds 'ere")


current_min = float("inf")
for seed in seeds:
    n = seed
    for mpt in maps:
        n = apply_map(n, mpt)
        if n < current_min:
            current_min = n

print(current_min)
