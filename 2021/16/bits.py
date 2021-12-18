import functools

with open('in') as f:
    raw = f.readline()
    total_bits = len(raw * 4)
    raw_packet = bin(int(raw, 16))[2:].zfill(total_bits)

b2i = lambda x: int(x, 2)

type_ops = {
    0: lambda x: sum([p['value'] for p in x]),
    1: lambda x: functools.reduce(lambda acc, p: acc * p['value'], x, 1),
    2: lambda x: min([p['value'] for p in x]),
    3: lambda x: max([p['value'] for p in x]),
    5: lambda x: x[0]['value'] > x[1]['value'],
    6: lambda x: x[0]['value'] < x[1]['value'],
    7: lambda x: x[0]['value'] == x[1]['value']
}

def parse_literal(p):
    value = ''
    while True:
        header_bit = p[0]
        value += p[1:5]
        p = p[5:]
        if b2i(header_bit) == 0:
            break
    return (p, b2i(value))

def parse_operator_bits(p):
    sub_packets = []
    while len(p) > 0:
        (p, packet) = parse_packet(p)
        sub_packets.append(packet)
    return sub_packets

def parse_operator_packets(p, sub_packet_length):
    sub_packets = []
    for _ in range(sub_packet_length):
        (p, packet) = parse_packet(p)
        sub_packets.append(packet)
    return (p, sub_packets)

def parse_packet(p):
    packet = {
        'version': b2i(p[0:3]),
        'type': b2i(p[3:6]),
        'value': None,
        'sub_packets': []
    }
    if packet['type'] == 4:
        (p, value) = parse_literal(p[6:])
        packet['value'] = value
    else:
        length_type_id = b2i(p[6])
        if length_type_id == 0:
            total_length = b2i(p[7:22])
            sub_packets = parse_operator_bits(p[22:22 + total_length])
            p = p[22 + total_length:]
        else:
            sub_packet_length = b2i(p[7:18])
            (p, sub_packets) = parse_operator_packets(p[18:], sub_packet_length)
        packet['sub_packets'] = sub_packets
        packet['value'] = type_ops[packet['type']](sub_packets)
    return (p, packet)

(p, packet) = parse_packet(raw_packet)

def get_packet_versions(packet):
    for p in packet['sub_packets']:
        yield from get_packet_versions(p)
    yield packet['version']

packet_version_sum = sum([p for p in get_packet_versions(packet)])

print(f'packet version sum: {packet_version_sum}')
print(f'packet value: {packet["value"]}')