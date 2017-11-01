def foo(x, y):
    k = z + x + y
    return lambda g: -(g + k)
v = 100 if foo(input(), input()) else 33
#c = c == 100 != 33
