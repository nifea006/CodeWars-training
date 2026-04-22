def maskify(inserted_value):
    if len(inserted_value) >= 4:
        inserted_value = '#' * (len(inserted_value) - 4) + inserted_value[-4:]
    elif len(inserted_value) < 4:
        return inserted_value
    else:
        return None
    return inserted_value

# def maskify(cc):
#     return "#"*(len(cc)-4) + cc[-4:]