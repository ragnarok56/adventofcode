#!/usr/bin/python3

class Node:
    def __init__(self, num, index):
        self.next = None
        self.prev = None
        self.num = num
        self.index = index

    def __repr__(self):
        return f'{self.num}'

def part(key, mixes):

    with open('in') as f:
        nodes = [Node(int(l) * key, i) for (i, l) in enumerate(f.readlines())]

        zero_node = None
        for (i, n) in enumerate(nodes):
            if n.num == 0:
                zero_node = n
            if i == 0 or i == len(nodes):
                continue
            
            n.prev = nodes[i-1]
            n.prev.next = n
        nodes[len(nodes)-1].next = nodes[0]
        nodes[0].prev = nodes[len(nodes)-1]

        orig = [n for n in nodes]
        orig_len = len(orig)

        for _ in range(mixes):
            for cur in nodes:
                if cur.num == 0:
                    continue

                dir_attr = 'prev' if cur.num < 0 else 'next'
                iterations = abs(cur.num) % (orig_len - 1)
                iter_node = cur

                # swap out existing 
                cur_prev = cur.prev
                cur_next = cur.next
                cur_prev.next = cur_next
                cur_next.prev = cur_prev
                
                # move to destination
                for r in range(iterations):
                    iter_node = getattr(iter_node, dir_attr)

                if dir_attr == 'prev':
                    iter_node_prev = iter_node.prev
                    iter_node_prev.next = cur
                    iter_node.prev = cur
                    cur.next = iter_node
                    cur.prev = iter_node_prev
                else:
                    iter_node_next = iter_node.next
                    iter_node_next.prev = cur
                    iter_node.next = cur
                    cur.next = iter_node_next
                    cur.prev = iter_node

        total = 0
        for r in range(1,4):
            iter_node = zero_node
            for _ in range((r * 1000) % orig_len):
                iter_node = iter_node.next
            total += iter_node.num
            
        print(total)

if __name__ == '__main__':
    part(1, 1)
    part(811589153, 10)