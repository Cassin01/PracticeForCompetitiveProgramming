NK = [int(i) for i in input().split()]
L = [int(i) for i in input().split()]

K = NK[1]

f = 0

max_ = 0
for i in range(K + 1):
    if max_ < f:
        max_ = f

print(max_)
