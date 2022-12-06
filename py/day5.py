#!/usr/bin/env python3 -i

"""
    [H]         [H]         [V]
    [V]         [V] [J]     [F] [F]
    [S] [L]     [M] [B]     [L] [J]
    [C] [N] [B] [W] [D]     [D] [M]
[G] [L] [M] [S] [S] [C]     [T] [V]
[P] [B] [B] [P] [Q] [S] [L] [H] [B]
[N] [J] [D] [V] [C] [Q] [Q] [M] [P]
[R] [T] [T] [R] [G] [W] [F] [W] [L]
 1   2   3   4   5   6   7   8   9
 """

def move_one(ss, i0, i1):
    ss[i0], ss[i1] = ss[i0][:-1], ss[i1] + ss[i0][-1]

def move_n(ss, n, i0, i1):
    ss[i0], ss[i1] = ss[i0][:-n], ss[i1] + ss[i0][-n:]

ls = [x.strip() for x in open("day5/input", "r")]
moves = ls[10:]
# moves = """move 1 from 2 to 1
# move 3 from 1 to 3
# move 2 from 2 to 1
# move 1 from 1 to 2""".split("\n")
mvs = [(int(m.split(' from ')[0][5:]), int(m.split(' from ')[1].split(' to ')[0]), int(m.split(' to ')[1])) for m in moves]

stacks_orig = ["", "RNPG", "TJBLCSVH", "TDBMNL", "RVPSB", "GCQSWMVH", "WQSCDBJ", "FQL", "WMHTDLFV", "LPBVMJF"]
# stacks_orig = ['', 'ZN', 'MCD', 'P']
stacks = stacks_orig[:]

for num, i0, i1 in mvs:
    for n in range(num):
        move_one(stacks, i0, i1)

# for num, i0, i1 in mvs:
#   move_n(stacks, num, i0, i1)

print(''.join(s[-1] for s in stacks[1:]))