import sys

class Tree:
    def __init__(self, w: int, h: int, presents: list[int]):
        self.w = w
        self.h = h
        self.presents = presents

def is_solveable(tree: Tree) -> bool:
    return tree.h * tree.w >= sum(tree.presents) * 9

def main():
    data = sys.stdin.read()
    parts = data.split("\n\n")
    last = parts[-1]

    trees: list[Tree] = []
    for line in last.strip().splitlines():
        dims, nums = line.split(": ", 1)
        w_str, h_str = dims.split("x", 1)
        w = int(w_str)
        h = int(h_str)
        presents = [int(x) for x in nums.split()]
        trees.append(Tree(w, h, presents))

    solvable = sum(1 for t in trees if is_solveable(t))
    print(f"solvable: {solvable}")

if __name__ == "__main__":
    main()

