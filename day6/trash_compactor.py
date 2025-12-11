import sys

def read_input():
    lines = [line.rstrip("\n") for line in sys.stdin]
    if not lines: 
        print(0); sys.exit(0)
    w = max(len(s) for s in lines)
    return [s.ljust(w) for s in lines], w

def is_blank_col(grid, c):
    return all(row[c] == ' ' for row in grid)

def spans(grid, w):
    res = []
    c = 0
    while c < w:
        if is_blank_col(grid, c):
            c += 1
        else:
            s = c
            while c < w and not is_blank_col(grid, c):
                c += 1
            res.append((s, c))
    return res

def extract_pos_ints(s):
    out, i, n = [], 0, len(s)
    while i < n:
        if s[i].isdigit():
            j = i
            while j < n and s[j].isdigit(): j += 1
            out.append(int(s[i:j]))
            i = j
        else:
            i += 1
    return out

def main():
    grid, w = read_input()
    h = len(grid)
    bottom = grid[-1]
    total = 0

    for a, b in spans(grid, w):
        op = None
        for c in range(a, b):
            if bottom[c] in "+*":
                op = bottom[c]
        if op is None:
            continue

        nums = []
        for r in range(h - 1):
            nums.extend(extract_pos_ints(grid[r][a:b]))

        if op == '+':
            total += sum(nums)
        else:  # '*'
            prod = 1
            for x in nums:
                prod *= x
            total += prod

    print(total)

if __name__ == "__main__":
    main()
