import sys

def read_grid():
    lines = [line.rstrip("\n") for line in sys.stdin]
    if not lines:
        return [], 0, 0, None
    h = len(lines)
    w = max(len(s) for s in lines)
    grid = [s.ljust(w, '.') for s in lines]
    start = None
    for r, row in enumerate(grid):
        c = row.find('S')
        if c != -1:
            start = (r, c)
            break
    return grid, h, w, start

def count_timelines_quantum(grid, h, w, start):
    if start is None:
        return 0
    sr, sc = start
    active = [0] * w
    active[sc] = 1
    exits = 0

    for r in range(sr + 1, h):
        row = grid[r]
        next_active = [0] * w
        for j, cnt in enumerate(active):
            if cnt == 0:
                continue
            cell = row[j]
            if cell == '^':
                if j - 1 >= 0:
                    next_active[j - 1] += cnt
                else:
                    exits += cnt
                if j + 1 < w:
                    next_active[j + 1] += cnt
                else:
                    exits += cnt
            else:
                next_active[j] += cnt
        active = next_active

    return exits + sum(active)

def main():
    grid, h, w, start = read_grid()
    print(count_timelines_quantum(grid, h, w, start))

if __name__ == "__main__":
    main()
