import re
print(f"Part 1: {sum([int(mul.split(",")[0].split("(")[-1]
                          ) * int(mul.split(",")[1][:-1]) for mul in re.findall(r"mul\(\d+,\d+\)", "".join(open("day3.txt", "r").read().splitlines()))])}")
do_ = True
print(f"Part 2: {sum(int(mul.split(",")[0].split("(")[-1]) * int(mul.split(",")[1][:-1]) for mul in re.findall(r"mul\(\d+,\d+\)|do\(\)|don't\(\)", "".join(
    open("day3.txt", "r").read().splitlines())) if (do_ := (mul == "do()") or (do_ if mul != "don't()" else False)) and mul.startswith("mul"))}")
