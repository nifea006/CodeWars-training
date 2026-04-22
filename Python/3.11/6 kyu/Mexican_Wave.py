def wave(people):
    person_list = []
    for i in range(len(people)):
        if people[i] == ' ':
            continue
        person_list.append(people[:i] + people[i].upper() + people[i+1:])
    return person_list

# def wave(str):
#     return [str[:i] + str[i].upper() + str[i+1:] for i in range(len(str)) if str[i].isalpha()]