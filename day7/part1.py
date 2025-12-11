
import sys

def read_grid():
    lines = [line.rstrip("\n") for line in sys.stdin]
    if not lines:
        return [], 0, 0, None
    h = len(lines)
    w = max(len(s) for s in lines)
    grid = [s.ljust(w, '.') for s in lines]
    start_row, start_col = None, None
    for r, row in enumerate(grid):
        c = row.find('S')
        if c != -1:
            start_row, start_col = r, c
            break
    return grid, h, w, (start_row, start_col)

def count_splits(grid, h, w, start):
    if start is None:
        return 0
    sr, sc = start
    active = [False] * w
    active[sc] = True
    splits = 0

    for r in range(sr + 1, h):
        next_active = [False] * w
        row = grid[r]
        for j, on in enumerate(active):
            if not on:
                continue
            cell = row[j]
            if cell == '^':
                splits += 1
                if j - 1 >= 0:
                    next_active[j - 1] = True
                if j + 1 < w:
                    next_active[j + 1] = True
            else:
                next_active[j] = True
        active = next_active
    return splits

def main():
    grid, h, w, start = read_grid()
    print(count_splits(grid, h, w, start))

if __name__ == "__main__":
    main()

