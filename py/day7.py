#!/usr/bin/env python3 -i

from pprint import pprint
lns = [ln.strip() for ln in open("../inputs/day7", 'r')]
# lns = [ln.strip() for ln in open("../inputs/example7", 'r')]

fs = {"/": {}}
pwd = ["/"]  # absolute path
i = 0
def subfs(path):
    curr = fs
    for part in path:
        curr = curr[part]
    return curr
def touch(parent, name, size):
    curr = subfs(parent)
    curr[name] = size
def mkdir(parent, name):
    curr = subfs(parent)
    curr[name] = {}
def sum_sizes_under_size(curr, max_size):
    # curr is a dir child of fs
    # returns current_size, sum of current_size's so far
    total_size, total_sum = 0, 0
    for sub in curr.values():
        if isinstance(sub, int):
            total_size += sub
        elif isinstance(sub, dict):
            sub_size, sub_sum = sum_sizes_under_size(sub, max_size)
            total_size += sub_size
            total_sum += sub_sum
    if total_size <= max_size:
        total_sum += total_size
    return total_size, total_sum
def find_dir(curr, min_size):
    # Find dir with smallest total_size that is higher than min_size
    # curr is a dir child of fs
    # returns total_size, smallest total_size greater than min_size so far
    total_size = 0
    current_min = float("inf")
    for sub in curr.values():
        if isinstance(sub, int):
            total_size += sub
        elif isinstance(sub, dict):
            sub_size, sub_min = find_dir(sub, min_size)
            total_size += sub_size
            current_min = min(current_min, sub_min)
    if total_size >= min_size:
        current_min = min(current_min, total_size)
    return total_size, current_min
while i < len(lns):
    ln = lns[i]
    if ln.startswith("$ cd"):
        _, cmd, arg = ln.split(" ")
        if arg == "/":
            pwd = ["/"]
        elif arg == "..":
            pwd = pwd[:-1]
        else:
            pwd.append(arg)
    elif ln.startswith("$ ls"):
        i += 1
        while i < len(lns):
            ln = lns[i]
            size, name = ln.split(" ")
            if size == "dir":
                mkdir(pwd, name)
            else:
                size = int(size, 10)
                touch(pwd, name, size)
            if i + 1 < len(lns) and lns[i+1].startswith("$ "):
                break
            else:
                i += 1
    i += 1

total_size, total_sum = sum_sizes_under_size(fs, 100000)
to_delete = 30000000 - (70000000 - total_size)
if to_delete > 0:
    print(find_dir(fs, to_delete))
else:
    print("nothing to delete")
