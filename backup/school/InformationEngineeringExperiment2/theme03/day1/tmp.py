import math
i0 = math.exp(1) 
i1 = math.exp(1 / 4 + 1)
i2 = math.exp(2 / 4 + 1)
i3 = math.exp(3 / 4 + 1)
i4 = math.exp(4 / 4 + 1)

b4 = 0.5 * ((i0 + i1) + (i1 + i2) + (i2 + i3) + (i3 + i4)) / 4

a = math.exp(2) - math.exp(1)

print(a)

b1 = 0.5 * (math.exp(1) + math.exp(2)) /2
b2 = 0.5 * ((math.exp(1) + math.exp(1/2 + 1)) + (math.exp(1/2 + 1) + math.exp(2))) / 2
b3 = 0.5 *( (math.exp(1) + math.exp(1/3 + 1)) + (math.exp(1/3 + 1) + math.exp(2 / 3)) + (math.exp(2 / 3 + 1) + math.exp(3 / 3 + 1)) ) / 3
print(b1)
print(b2)
print(b3)
print(b4)

c1 = (b1 - a) * 100 / a
c2 = (b2 - a) * 100 / a
c3 = (b3 - a) * 100 / a
c4 = (b4 - a) * 100 / a

print(c1)
print(c2)
print(c3)
print(c4)
