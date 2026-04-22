def is_square(n):
    if n < 0:
        return False
    result = pow(n, 0.5) 
    if result == int(result):
        result = True
    else:
        result = False
    return result

# def is_square(n):
#     return n >= 0 and (n**0.5) % 1 == 0