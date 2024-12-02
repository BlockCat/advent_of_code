from collections import Counter

with open("./input/day_01.txt") as f:
    data = [list(map(int, x.split("   "))) for x in f.read().strip().splitlines()]

left_list, right_list = sorted(x[0] for x in data), sorted(x[1] for x in data)

counter_left, counter_right = Counter(left_list), Counter(right_list)

print("ex1:", sum(abs(x - y) for x, y in zip(left_list, right_list)))
print("ex2:", sum(x * counter_left[x] * counter_right[x] for x in counter_left))
