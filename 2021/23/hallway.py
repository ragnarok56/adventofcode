from queue import PriorityQueue
from functools import total_ordering

q = PriorityQueue(maxsize=8)

"""
#############
#...........#
###D#B#C#A###
  #C#A#D#B#
  #########
"""

@total_ordering
class Pod:
    def __init__(self, type, pos):
        self.type = type
        self.pos = None
    def __eq__(self, other):
        return self.type == other.type
    def __lt__(self, other):
        return self.type > other.type
    def __repr__(self):
        return f'[{self.type}] {self.pos}'


d1 = Pod(3, (0,0))
d2 = Pod(3, (2,1))
c1 = Pod(2, (0,1))
c2 = Pod(2, (2,0))
b1 = Pod(1, (1,0))
b2 = Pod(1, (3,1))
a1 = Pod(0, (1,1))
a2 = Pod(0, (3,0))

for pod in [a1,a2,b1,b2,c1,c2,d1,d2]:
    q.put(pod)

while not q.empty():
    p = q.get()
    print(p)

# print(q)