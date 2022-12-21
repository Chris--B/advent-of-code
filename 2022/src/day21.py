input = """
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
""".strip()

with open("input/2022/day21.txt") as f:
    input = f.read()

lines = [line.replace(': ', ' = ').strip() for line in input.split("\n")]

if False:
    print("Part 1!")
    for x in range(45):
        for line in lines:
            try:
                exec(line)
            except NameError:
                continue
    print()

    print("root", "=", root)
    print()

print("Part 2!")

humn = 3.782852515583e12  # Adjust as needed


def fix_line(line):
    if line.startswith("root"):
        return line.replace("+", "==")

    if line.startswith("humn"):
        return "x=1"

    return line


lines = [fix_line(line) for line in lines]
for x in range(45):
    for line in lines:
        try:
            exec(line)
        except NameError:
            continue

for line in lines:
    if line.startswith("root"):
        a = line.split()
        x, y = (int(vars()[a[2]]), int(vars()[a[4]]))
        print("{:>15}\n{:>15}".format(x, y))

print(x / y)
print(x - y)
print("root", "=", root)
print("humn", "=", humn)
