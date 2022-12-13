lines = []

with open("input/2022/day13.txt") as f:
    for line in f:
        if len(line.strip()) != 0:
            lines.append(eval(line))


def to_rust(x):
    if isinstance(x, int):
        print("Int(%d)" % x, end=' ')
    elif isinstance(x, list):
        print("List(vec![", end=' ')
        for y in x:
            to_rust(y)
            print(', ', end=' ')
        print("])", end=' ')
    else:
        print("???")


lines = [(a, b)for (a, b) in zip(lines[::2], lines[1::2])]
for (a, b) in lines:
    print('(', end='')
    to_rust(a)
    print(',', end='')
    to_rust(b)
    print('),')
