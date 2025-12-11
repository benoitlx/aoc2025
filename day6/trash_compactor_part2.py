
import sys

g = [line.rstrip("\n") for line in sys.stdin]
if not g: print(0); sys.exit(0)
w = max(map(len, g))
g = [s.ljust(w) for s in g]
h = len(g)

def blank(c): return all(row[c] == ' ' for row in g)

spans, c = [], 0
while c < w:
    if blank(c): c += 1
    else:
        s = c
        while c < w and not blank(c): c += 1
        spans.append((s, c))

total = 0
for a, b in spans:
    op = next((g[-1][c] for c in range(a, b) if g[-1][c] in "+*"), "+")
    nums = []
    for c in range(b - 1, a - 1, -1):
        digs = [g[r][c] for r in range(h - 1) if g[r][c].isdigit()]
        if digs:
            nums.append(int("".join(digs)))
    if op == '+':
        total += sum(nums)
    else:
        p = 1
        for x in nums: p *= x
        total += p

print(total)

