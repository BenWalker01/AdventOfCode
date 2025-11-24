with open("day1_input.txt") as infile:
    data = " "
    sum_fin = 0
    while data:
        data = infile.readline().strip()
        if data == "":
            break
        start = 0
        end = len(data) - 1
        number = [-1,-1]
        while -1 in number:
            if data[start].isnumeric():
                number[0] = data[start]

            if number[0] == -1:
                start += 1

            if data[end].isnumeric():
                number[1] = data[end]

            if number[1] == -1:
                end -= 1
        sum_fin += int("".join(number))
        
print(sum_fin)































def find_first_word(s, word_numbers):
    for i in range(len(s)):
        for word in word_numbers:
            if s[i:i+len(word)] == word:
                return word, i
    return None, -1

def find_last_word(s, word_numbers):
    for i in range(len(s), -1, -1):
        for word in word_numbers:
            if s[i-len(word):i] == word:
                return word, i
    return None, -1

def process(key):
    word_numbers = {
        "zero": 0,
        "one": 1,
        "two": 2,
        "three": 3,
        "four": 4,
        "five": 5,
        "six": 6,
        "seven": 7,
        "eight": 8,
        "nine": 9
    }
    ret = [None, None]
    for i in range(len(key)):
        if key[i].isnumeric():
            ret[0] = int(key[i])
            break
        else:
            for word, num in word_numbers.items():
                if key.startswith(word, i):
                    ret[0] = num
                    break
            if ret[0] is not None:
                break

    for i in range(len(key)-1, -1, -1):
        if key[i].isnumeric():
            ret[1] = int(key[i])
            break
        else:
            for word, num in word_numbers.items():
                if key.endswith(word, 0, i+1):
                    ret[1] = num
                    break
            if ret[1] is not None:
                break

    if all(x is None for x in ret):
        return None

    return int("".join(map(str, ret)))




with open("day1_input.txt") as infile:
    data = "0"
    sum_fin = 0
    while data:
        data = infile.readline().strip()
        result = process(data)
        print(result)
        if result is not None:
            sum_fin += int(process(data))

print(sum_fin)



