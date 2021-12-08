#!/usr/bin/python3

with open('in') as f:
    codes = f.readlines()

code_length = len(codes[0]) - 1

def cull(codes, position, check_winner):
    place_counts = 0
    pools = ([], [])
    for i, c in enumerate(codes):
        b = int(c[position])
        pools[b].append(i)
        place_counts = place_counts + b

    winner = check_winner(place_counts)

    valid_codes = []
    for i in pools[winner]:
        valid_codes.append(codes[i])
    return valid_codes

ogr_codes = codes
for i in range(code_length):
    min_bit_count = len(ogr_codes) / 2
    ogr_codes = cull(ogr_codes, i, lambda x: 1 if x == min_bit_count else x > min_bit_count)
    if len(ogr_codes) == 1:
        break
oxygen_generator_rating = int(''.join(ogr_codes[0].rstrip()), 2)

cs_codes = codes
for i in range(code_length):
    min_bit_count = len(cs_codes) / 2
    cs_codes = cull(cs_codes, i, lambda x: 0 if x == min_bit_count else x < min_bit_count)
    if len(cs_codes) == 1:
        break
co2_scrubber_rating = int(''.join(cs_codes[0].rstrip()), 2)

print(oxygen_generator_rating * co2_scrubber_rating)