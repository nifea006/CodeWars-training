def high(x):
    max_score = 0
    best_word = ""
    for word in x.split():
        score = sum(ord(c) - 96 for c in word)
        if score > max_score:
            max_score = score
            best_word = word
    return best_word

# def high(x):
#     list = []
#     for i in x.split():
#         scores = [sum([ord(char) - 96 for char in i])]
#         list.append(scores)
#     return x.split()[list.index(max(list))]