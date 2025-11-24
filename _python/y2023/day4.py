# with open("day4_input.txt", 'r') as file:
#     data = " "
#     cleaned = []
#     sum_fin = 0
#     while data:
#         if not data:
#             break
#         data = file.readline().strip()
#         cld = data.split("|")
#         cld[0] = cld[0].split(":")
#         if len(cld[0]) > 1:
#             cld[0] = cld[0][1]
#             winning = cld[0].split(" ")
#             mine = cld[1].split(" ")
#         winning = [int(w) for w in winning if w]
#         mine = [int(m) for m in mine if m]
#         has_won = list(set([val for val in mine if val in winning]))

#         sum_fin += 2**(len(has_won)-1) if len(has_won) > 0 else 0
#         # print(sum_fin)


with open("day4_input.txt", 'r') as file:
    data = " "
    pile = [1]
    sum_fin = 0
    i = 0
    while data:
        if not data:
            break
        data = file.readline().strip()
        i += 1
        print(f"card {i}")
        cld = data.split("|")
        cld[0] = cld[0].split(":")
        if len(cld[0]) > 1:
            cld[0] = cld[0][1]
            winning = cld[0].split(" ")
            mine = cld[1].split(" ")
        winning = [int(w) for w in winning if w]
        mine = [int(m) for m in mine if m]
        has_won = list(set([val for val in mine if val in winning]))
        to_add = len(has_won)


print(len(pile))
