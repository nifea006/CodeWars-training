def decode(s):
    if not s:
        return ""
    s = "".join(sorted(list(s.replace(" ", "").lower())))
    result = ""
    count = 1
    for i in range(1, len(s)):
        if s[i] == s[i - 1]:
            count += 1
        else:
            result += s[i - 1] + (str(count) if count >= 1 else "") + "; "
            count = 1
    result += s[-1] + (str(count) if count >= 1 else "")
    return result

print(decode("Nå kan mann skrive inn hva som helst for å finne ut antall bokstaver og symboler!"))