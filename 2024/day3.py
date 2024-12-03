import re; print(sum([int(mul.split(",")[0].split("(")[-1]
               ) * int(mul.split(",")[1][:-1]) for mul in re.findall(r"mul\(\d+,\d+\)", "".join(open("day3.txt", "r").read().splitlines()))]))
