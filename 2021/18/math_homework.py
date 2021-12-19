import anytree
from anytree import Node, RenderTree
import math
import ast
import json
from itertools import combinations

def gen_tree(pairs, parent=None):
    for p in pairs:
        if isinstance(p, list):
            gen_tree(p, Node('', parent=parent))
        else:
            Node(p, parent=parent)

def render_tree(root):
    for pre, _, node in RenderTree(root):
        print(f'{pre}{node.name}')

def find_left_number(node, descend=False):
    if node is None:
        return None
    if node.is_leaf:
        return node
    if descend:
        return find_left_number(node.children[1], descend=True)
    else:
        sib = anytree.util.leftsibling(node)
        if sib is None:
            return find_left_number(node.parent)
        if sib.is_leaf:
            return sib
        else:
            return find_left_number(sib.children[1], descend=True)

def find_right_number(node, descend=None):
    if node is None:
        return None
    if node.is_leaf:
        return node
    if descend:
        return find_right_number(node.children[0], descend=True)
    else:
        sib = anytree.util.rightsibling(node)
        if sib is None:
            return find_right_number(node.parent)
        if sib.is_leaf:
            return sib
        else:
            return find_right_number(sib.children[0], descend=True)

def explode(node):
    sib_node = node.siblings[0]
    parent_node = node.parent

    left = find_left_number(parent_node)
    if left:
        left.name = int(left.name) + int(node.name)
    right = find_right_number(sib_node.parent)
    if right:
        right.name = int(right.name) + int(sib_node.name)

    # remove node
    node.parent = None
    sib_node.parent = None
    parent_node.name = 0
    parent_node.children = []

def split(node):
    Node(math.floor(int(node.name) / 2), parent=node)
    Node(math.ceil(int(node.name) / 2), parent=node)
    node.name = ""

def check_magnitude(node):
    [left, right] = node.children

    left_magnitude = (int(left.name) if left.is_leaf else check_magnitude(left)) * 3
    right_magnitude = (int(right.name) if right.is_leaf else check_magnitude(right)) * 2

    return left_magnitude + right_magnitude

def reduce(root):
    while True:
        node_to_explode = next((n for n in root.leaves if n.depth > 4), None)
        if node_to_explode:
            explode(node_to_explode)
            continue

        node_to_split = next((n for n in root.leaves if int(n.name) >= 10), None)
        if node_to_split:
            split(node_to_split)
            continue

        if not node_to_explode and not node_to_split:
            break

with open('in') as f:
    lines = f.readlines()

def part1():
    root = None

    for l in lines:
        l = json.loads(l)

        line_root = Node('')
        gen_tree(l, line_root)

        if not root:
            root = line_root
        else:
            new_root = Node('')
            new_root.children = [root, line_root]
            root = new_root

        reduce(root)

    print(f'magnitude of sum: {check_magnitude(root)}')

def part2():
    # this is sloooow
    largest_magnitude = 0
    eval_lines = [json.loads(l) for l in lines]
    for [l1, l2] in combinations(eval_lines, 2):
        root1 = Node('')
        gen_tree(l1, root1)
        root2 = Node('')
        gen_tree(l2, root2)
        root = Node('')
        root.children = [root1, root2]

        reduce(root)

        magnitude = check_magnitude(root)
        if magnitude > largest_magnitude:
            largest_magnitude = magnitude

    print(f'largest magnitude of 2 numbers: {largest_magnitude}')

part1()
part2()