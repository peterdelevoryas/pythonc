def f(a):
    def g():
        return a
    return g
a = 3
print f(1)()
