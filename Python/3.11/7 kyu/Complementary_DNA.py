def DNA_strand(dna):
    output = {'A': 'T', 'T': 'A', 'C': 'G', 'G': 'C'}
    return ''.join([output[symbol] for symbol in dna])

# def DNA_strand(dna):
#     return dna.translate(str.maketrans("ATCG","TAGC"))