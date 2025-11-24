import re
from functools import reduce
import operator

cfg = {"red":12,"green":13,"blue":14}

with open("day2_input.txt")as infile:
    sum_fin = 0
    data = " "
    while data:
        data = infile.readline().strip()
        if data == "":
            break
        clnd = re.split("[:;]",data)
        
        num = clnd[0][5:]
        poss = True
        for hint in clnd[1:]:
            if poss == False:
                break
            hints = hint.split(",")
            for col in hints:
                col = col.split(" ")
                if cfg[col[2]] < int(col[1]):#game not poss
                    poss = False

        if poss == True:
            sum_fin += int(num)

#print(sum_fin)



with open("day2_input.txt")as infile:
    sum_fin = 0
    data = " "
    while data:
        smallest = {"red":float("-inf"),"blue":float("-inf"),"green":float("-inf"),}
        data = infile.readline().strip()
        if data == "":
            break
        clnd = re.split("[:;]",data)
        for hint in clnd[1:]:
            if poss == False:
                break
            hints = hint.split(",")
            for col in hints:
                col = col.split(" ")
                smallest[col[2]] = max(smallest[col[2]],int(col[1]))
        
        
        
        power = reduce(operator.mul, smallest.values(), 1)
        sum_fin += power


print(sum_fin)
                


