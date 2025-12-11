from typing import List, Tuple
import sys
import pulp

def parse_switch(token: str) -> List[int]:
    token = token.strip()
    if token.startswith('(') and token.endswith(')'):
        inner = token[1:-1]
        if inner.strip() == '':
            return []
        return [int(x.strip()) for x in inner.split(',') if x.strip()]
    raise ValueError(f"Invalid switch token: {token}")

def parse_target(token: str) -> List[int]:
    token = token.strip()
    if token.startswith('{') and token.endswith('}'):
        inner = token[1:-1]
        if inner.strip() == '':
            return []
        return [int(x.strip()) for x in inner.split(',') if x.strip()]

def parse_line(line: str) -> Tuple[List[List[int]], List[int]]:
    parts = [p for p in line.strip().split(' ') if p]

    if parts[0].startswith('['):
        parts = parts[1:]

    target = parse_target(parts[-1])
    switches = [parse_switch(tok) for tok in parts[:-1]]
    return switches, target

# MILP
def solve_ilp_with_pulp(buttons: List[List[int]], target: List[int]) -> Tuple[int, List[int]]:
    n = len(target)
    m = len(buttons)

    A = [[0]*m for _ in range(n)]
    for j, btn in enumerate(buttons):
        for i in btn:
            if i < 0 or i >= n:
                raise ValueError(f"Button {j} references counter index {i} out of range 0..{n-1}")
            A[i][j] = 1

    prob = pulp.LpProblem('Joltage', pulp.LpMinimize)
    x = [pulp.LpVariable(f'x_{j}', lowBound=0, cat='Integer') for j in range(m)]

    # Objectif: minimiser la somme des composantes de x
    prob += pulp.lpSum(x)

    # Avec la contrainte A x = t
    for i in range(n):
        prob += pulp.lpSum(A[i][j]*x[j] for j in range(m)) == target[i], f"counter_{i}"

    solver = pulp.PULP_CBC_CMD(msg=False)
    status = prob.solve(solver)

    presses = [int(round(v.value())) for v in x]
    total = sum(presses)
    return total, presses

def main():
    lines: List[str] = []
    for line in sys.stdin:
        if line.strip():
            lines.append(line.rstrip('\n'))

    total_sum = 0
    for line in lines:
        buttons, target = parse_line(line)
        total, presses = solve_ilp_with_pulp(buttons, target)
        total_sum += total
    print(f"Sum of minimum presses across all machines: {total_sum}")

if __name__ == "__main__":
    main()

