z = 3
def foo(x, y):
    k = z + x + y
    return lambda g: -(g + k)
v = 100 if foo(input(), input()) else 33
c = v == 100
